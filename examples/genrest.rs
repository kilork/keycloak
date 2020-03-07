use heck::*;
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, fs::read_to_string};

fn main() -> Result<(), std::io::Error> {
    let document = Html::parse_document(&read_to_string("./docs/rest-api-9.html")?);

    let definitions_selector =
        Selector::parse("#_definitions ~ div.sectionbody > div.sect2").unwrap();

    let definitions = document.select(&definitions_selector);

    let mut rename_table = HashMap::new();
    rename_table.insert("Userinfo", "UserInfo".to_string());

    println!("use serde::{{Serialize, Deserialize}};");
    println!("use serde_json::Value;");
    println!("use std::{{borrow::Cow, collections::HashMap}};\n");

    for definition in definitions {
        let struct_name = text(&definition, "h3").replace("-", "");

        let fields_selector = Selector::parse("tbody tr").unwrap();

        let fields = definition.select(&fields_selector);
        let mut result_fields = vec![];
        let mut camel_case = false;
        for field in fields {
            let original_field = text(&field, "strong");
            let field_name = original_field.to_snake_case();
            let mut rename = None;
            if original_field != field_name {
                if field_name.to_mixed_case() == original_field {
                    camel_case = true;
                } else {
                    rename = Some(original_field);
                }
            }

            let original_field_type = text(&field, "td ~ td p").replace("-", "");

            let is_array =
                original_field_type.starts_with("< ") && original_field_type.ends_with(" > array");

            let mut field_type = match if is_array {
                convert_type(&original_field_type[2..original_field_type.len() - 8])
            } else {
                convert_type(&original_field_type)
            } {
                Ok(field_type) => field_type,
                Err(ConvertTypeFail::Enum(enum_fields)) => {
                    let enum_name = format!("{}{}", struct_name, field_name.to_camel_case());
                    println!("#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]");

                    let is_upper_case = enum_fields
                        .split(", ")
                        .all(|x| x.chars().all(|x| x.is_uppercase()));
                    if is_upper_case {
                        println!(r#"#[serde(rename_all="UPPERCASE")]"#);
                    }

                    println!("pub enum {} {{", enum_name);
                    for enum_field in enum_fields.split(", ") {
                        let enum_field = enum_field.to_camel_case();
                        println!(
                            "    {},",
                            rename_table
                                .get(enum_field.as_str())
                                .unwrap_or_else(|| &enum_field)
                        );
                    }
                    println!("}}\n");
                    enum_name
                }
                Err(err) => panic!("err: {:?}", err),
            };

            if is_array {
                field_type = format!("Vec<{}>", field_type);
            }

            let optional_required = text(&field, "em");

            if "optional" == optional_required {
                field_type = format!("Option<{}>", field_type);
            }

            result_fields.push((field_name, field_type, rename));
        }

        println!("#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]");
        if camel_case {
            println!(r#"#[serde(rename_all="camelCase")]"#);
        }
        println!("pub struct {}<'a> {{", struct_name);

        for (field_name, field_type, rename) in result_fields {
            if let Some(rename) = rename {
                println!(r#"    #[serde(rename="{}")]"#, rename);
            }
            let field_name = match field_name.as_str() {
                "type" | "self" => format!("r#{}", field_name),
                _ => field_name,
            };
            println!("    pub {}: {},", field_name, field_type);
        }

        println!("}}\n");
    }
    Ok(())
}

fn text(element: &ElementRef, selector: &str) -> String {
    let selector = Selector::parse(selector).unwrap();
    let element = element.select(&selector).next().unwrap();
    element_text(&element)
}

fn element_text(element: &ElementRef) -> String {
    element.text().collect::<Vec<_>>().join("")
}

fn convert_type(original: &str) -> Result<String, ConvertTypeFail> {
    Ok(match original {
        "string" => "Cow<'a, str>",
        "integer(int32)" => "i32",
        "integer(int64)" => "i64",
        "number(float)" => "f32",
        "boolean" => "bool",
        "Map" => "HashMap<Cow<'a, str>, Cow<'a, str>>",
        "Object" => "Value",
        _ => {
            if original.starts_with("enum (") {
                return Err(ConvertTypeFail::Enum(
                    (&original[6..original.len() - 1]).into(),
                ));
            } else if original.chars().next().unwrap().is_uppercase() {
                return Ok(format!("{}<'a>", original));
            } else {
                return Err(ConvertTypeFail::Unknown(original.into()));
            }
        }
    }
    .into())
}

#[derive(Debug)]
enum ConvertTypeFail {
    Enum(String),
    Unknown(String),
}
