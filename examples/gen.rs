use clap::Parser;
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, fs::read_to_string, rc::Rc};

use crate::common::{
    write_rest, write_types, EnumType, Field, FieldCase, FieldType, MethodStruct, Parameter,
    ResponseType, StructType,
};

mod common;

const RESERVED_WORDS: &[&str] = &["type", "self", "use"];

/// Generate Rust code from Keycloak REST Description in HTML
#[derive(Parser)]
enum Cli {
    /// Generate types
    Types,
    /// Generate method callers
    Rest,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    match cli {
        Cli::Types => generate_types()?,
        Cli::Rest => generate_rest()?,
    }
    Ok(())
}

fn parse_document() -> Result<Html, std::io::Error> {
    Ok(Html::parse_document(&read_to_string(
        "./docs/rest-api.html",
    )?))
}

fn generate_rest() -> Result<(), std::io::Error> {
    let document = parse_document()?;
    let stream_mapping: HashMap<String, String> =
        toml::from_str(include_str!("stream.toml")).unwrap();
    let methods = read_methods_info(&document, &stream_mapping)?;
    write_rest(&methods, &stream_mapping);
    Ok(())
}

fn generate_types() -> Result<(), std::io::Error> {
    let document = parse_document()?;
    let (enums, structs) = read_types_info(&document)?;
    write_types(&enums, &structs);
    Ok(())
}

fn check_array(value: &str) -> Option<&str> {
    if value.starts_with("< ") && value.ends_with(" > array") {
        Some(&value[2..value.len() - 8])
    } else {
        None
    }
}

fn check_optional(value: &str) -> bool {
    "optional" == value
}

type TypeDuo = (Vec<EnumType>, Vec<Rc<StructType>>);
fn read_types_info(document: &scraper::Html) -> Result<TypeDuo, std::io::Error> {
    let definitions_selector =
        Selector::parse("#_definitions ~ div.sectionbody > div.sect2").unwrap();

    let definitions = document.select(&definitions_selector);

    let mut rename_table = HashMap::new();
    rename_table.insert("Userinfo", "UserInfo".to_string());

    let mut structs = vec![];
    let mut enums = vec![];
    for definition in definitions {
        let struct_name = text(&definition, "h3").replace('-', "");

        let fields_selector = Selector::parse("tbody tr").unwrap();

        let fields = definition.select(&fields_selector);
        let mut result_fields = vec![];
        let mut camel_case_count = 0;
        let mut snake_case_count = 0;
        for field in fields {
            let original_field = text(&field, "strong");
            let mut field_name = original_field.to_snake_case();
            let fld_camel_case = original_field.to_lower_camel_case();

            let mut field_case = if field_name == fld_camel_case {
                FieldCase::Unknown
            } else if original_field == field_name {
                snake_case_count += 1;
                FieldCase::SnakeCase
            } else if original_field == fld_camel_case {
                camel_case_count += 1;
                FieldCase::CamelCase
            } else {
                FieldCase::Custom
            };

            let original_field_type = text(&field, "td ~ td p").replace('-', "");

            let array_field = check_array(&original_field_type);
            let is_array = array_field.is_some();

            let field_type = match if is_array {
                convert_type(array_field.unwrap())
            } else {
                convert_type(&original_field_type)
            } {
                Ok(field_type) => field_type,
                Err(ConvertTypeFail::Enum(enum_fields)) => {
                    let enum_name = format!("{}{}", struct_name, field_name.to_upper_camel_case());

                    let is_upper_case = enum_fields
                        .split(", ")
                        .all(|x| x.chars().all(|x| x.is_uppercase()));
                    let enum_ = EnumType {
                        name: enum_name.clone(),
                        is_upper_case,
                        fields: enum_fields
                            .split(", ")
                            .map(|enum_field| {
                                let enum_field = enum_field.to_upper_camel_case();
                                rename_table
                                    .get(enum_field.as_str())
                                    .unwrap_or(&enum_field)
                                    .clone()
                            })
                            .collect(),
                    };
                    enums.push(enum_);
                    FieldType::Simple(enum_name)
                }
                Err(err) => panic!("err: {:?}", err),
            };

            let optional_required = text(&field, "em");

            let is_optional = check_optional(&optional_required);

            if RESERVED_WORDS.contains(&field_name.as_str()) {
                field_case = FieldCase::Custom;
                field_name += "_";
            }

            let field = Field {
                field_name,
                original_field,
                field_case,
                field_type,
                is_array,
                is_optional,
            };

            result_fields.push(field);
        }

        let is_camel_case = camel_case_count > snake_case_count;

        let struct_ = Rc::new(StructType {
            name: struct_name.clone(),
            is_camel_case,
            fields: result_fields,
        });
        structs.push(struct_);
    }

    Ok((enums, structs))
}

fn read_methods_info(
    document: &scraper::Html,
    stream_mapping: &HashMap<String, String>,
) -> Result<Vec<MethodStruct>, std::io::Error> {
    let resources_selector = Selector::parse("#_paths ~ div.sectionbody > div.sect2").unwrap();

    let resources_html = document.select(&resources_selector);
    let mut methods = vec![];
    for resource in resources_html {
        let resource_name = text(&resource, "h3");

        let methods_selector = Selector::parse("div.sect3").unwrap();

        let methods_html = resource.select(&methods_selector);
        for method in methods_html {
            let method_name = text(&method, "h4");
            let anchor = attr(&method, "h4", "id").map(str::to_string);
            let path = text_opt(&method, "pre").unwrap_or_else(|| method_name.clone());
            let mut path_parts = path.split(' ');
            let method_http = path_parts.next().unwrap();
            let path = path_parts.next().unwrap();

            let blocks_selector = Selector::parse("div.sect4").unwrap();
            let blocks_html = method.select(&blocks_selector);

            let mut parameters = vec![];
            let mut response = None;
            let mut description = None;
            for block in blocks_html {
                let block_name = text(&block, "h5");

                match block_name.as_str() {
                    "Parameters" => {
                        let parameters_selector = Selector::parse("tbody > tr").unwrap();
                        let parameters_html = block.select(&parameters_selector);

                        for parameter in parameters_html {
                            let parameter_kind = text(&parameter, "td:nth-child(1) > p > strong");
                            let name = text(&parameter, "td:nth-child(2) > p > strong");
                            let optional_required = text(&parameter, "td:nth-child(2) > p > em");
                            let comment = text_opt(&parameter, "td:nth-child(3) > p");
                            let parameter_type = text_opt(&parameter, "td:nth-child(4) > p")
                                .unwrap_or_else(|| {
                                    text_opt(&parameter, "td:last-child > p").unwrap()
                                });

                            let array = check_array(&parameter_type);

                            let is_optional = check_optional(&optional_required);
                            let name_copy = name.clone();
                            let parameter_ = Parameter {
                                name,
                                comment,
                                is_optional,
                                is_array: array.is_some(),
                                kind: parameter_kind.parse().unwrap(),
                                parameter_type: array
                                    .or(Some(parameter_type.as_str()))
                                    .map(|param_type| {
                                        stream_mapping
                                            .get(&format!("{}:{}:{}", path, name_copy, param_type))
                                            .map(String::as_str)
                                            .unwrap_or(param_type)
                                    })
                                    .map(convert_type)
                                    .unwrap()
                                    .unwrap(),
                            };
                            parameters.push(Rc::new(parameter_));
                        }
                    }
                    "Responses" => {
                        let response_type = text(&block, "tbody > tr > td:nth-child(3) > p");
                        let array = check_array(&response_type);
                        response = Some(ResponseType {
                            is_array: array.is_some(),
                            return_type: array
                                .or(Some(response_type.as_str()))
                                .map(|x| convert_type(&x.replace('-', "")))
                                .unwrap()
                                .unwrap(),
                        });
                    }
                    "Consumes" | "Produces" => {}
                    "Description" => {
                        let description_text = text(&block, "div.paragraph > p");
                        description = Some(description_text)
                    }
                    _ => eprintln!("Unsupported block {}", block_name),
                }
            }

            let method = MethodStruct {
                anchor,
                comment: resource_name.clone(),
                name: method_name,
                parameters,
                path: path.into(),
                method: method_http.into(),
                response: response.unwrap(),
                description,
            };
            methods.push(method);
        }
    }

    Ok(methods)
}

fn text_opt(element: &ElementRef, selector: &str) -> Option<String> {
    let selector = Selector::parse(selector).unwrap();
    element.select(&selector).next().map(|x| element_text(&x))
}

fn text(element: &ElementRef, selector: &str) -> String {
    text_opt(element, selector).unwrap()
}

fn element_text(element: &ElementRef) -> String {
    element.text().collect::<Vec<_>>().join("")
}

fn attr<'a>(element: &'a ElementRef, selector: &str, name: &str) -> Option<&'a str> {
    let selector = Selector::parse(selector).unwrap();
    element
        .select(&selector)
        .next()
        .and_then(|x| x.value().attr(name))
}

fn convert_type(original: &str) -> Result<FieldType, ConvertTypeFail> {
    Ok(match original {
        "No Content" | "Response" => FieldType::Simple("()".into()),
        "file" => FieldType::Simple("&[u8]".into()),
        "string" | "< string > array(csv)" => FieldType::Simple("String".into()),
        "string(byte)" => FieldType::Simple("u8".into()),
        "integer(int32)" => FieldType::Simple("i32".into()),
        "integer(int64)" => FieldType::Simple("i64".into()),
        "number(float)" => FieldType::Simple("f32".into()),
        "boolean" => FieldType::Simple("bool".into()),
        "Map" => FieldType::Simple("HashMap<String, Value>".into()),
        "MultivaluedHashMap" => FieldType::Simple("HashMap<String, Vec<Value>>".into()),
        "Object" => FieldType::Simple("Value".into()),
        _ => {
            if original.starts_with("enum (") {
                return Err(ConvertTypeFail::Enum(
                    (&original[6..original.len() - 1]).into(),
                ));
            } else if original.chars().next().unwrap().is_uppercase() {
                FieldType::Registry(original.into())
            } else {
                return Err(ConvertTypeFail::Unknown(original.into()));
            }
        }
    })
}

#[derive(Debug)]
enum ConvertTypeFail {
    Enum(String),
    Unknown(#[allow(dead_code)] String),
}
