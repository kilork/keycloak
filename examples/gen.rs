use heck::*;
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, fs::read_to_string, rc::Rc};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Generate Rust code from Keycloak REST Description in HTML
enum Cli {
    /// Generate types
    Types,
    /// Generate method callers
    Rest,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();
    match cli {
        Cli::Types => generate_types()?,
        Cli::Rest => generate_rest()?,
    }
    Ok(())
}

fn generate_rest() -> Result<(), std::io::Error> {
    Ok(())
}

fn generate_types() -> Result<(), std::io::Error> {
    let document = Html::parse_document(&read_to_string("./docs/rest-api-9.html")?);

    let definitions_selector =
        Selector::parse("#_definitions ~ div.sectionbody > div.sect2").unwrap();

    let definitions = document.select(&definitions_selector);

    let mut rename_table = HashMap::new();
    rename_table.insert("Userinfo", "UserInfo".to_string());

    let mut structs = vec![];
    let mut type_registry = HashMap::new();
    let mut enums = vec![];
    for definition in definitions {
        let struct_name = text(&definition, "h3").replace("-", "");

        let fields_selector = Selector::parse("tbody tr").unwrap();

        let fields = definition.select(&fields_selector);
        let mut result_fields = vec![];
        let mut is_camel_case = false;
        for field in fields {
            let original_field = text(&field, "strong");
            let mut field_name = original_field.to_snake_case();
            let mut is_rename = false;
            if original_field != field_name {
                if field_name.to_mixed_case() == original_field {
                    is_camel_case = true;
                } else {
                    is_rename = true;
                }
            }

            let original_field_type = text(&field, "td ~ td p").replace("-", "");

            let is_array =
                original_field_type.starts_with("< ") && original_field_type.ends_with(" > array");

            let field_type = match if is_array {
                convert_type(&original_field_type[2..original_field_type.len() - 8])
            } else {
                convert_type(&original_field_type)
            } {
                Ok(field_type) => field_type,
                Err(ConvertTypeFail::Enum(enum_fields)) => {
                    let enum_name = format!("{}{}", struct_name, field_name.to_camel_case());

                    let is_upper_case = enum_fields
                        .split(", ")
                        .all(|x| x.chars().all(|x| x.is_uppercase()));
                    let enum_ = EnumType {
                        name: enum_name.clone(),
                        is_upper_case,
                        fields: enum_fields
                            .split(", ")
                            .map(|enum_field| {
                                let enum_field = enum_field.to_camel_case();
                                rename_table
                                    .get(enum_field.as_str())
                                    .unwrap_or_else(|| &enum_field)
                                    .clone()
                            })
                            .collect(),
                    };
                    enums.push(enum_);
                    FieldType::Simple(enum_name.into())
                }
                Err(err) => panic!("err: {:?}", err),
            };

            let optional_required = text(&field, "em");

            let is_optional = "optional" == optional_required;

            if field_name == "type" || field_name == "self" {
                is_rename = true;
                field_name = format!("{}_", field_name);
            }

            let field = Field {
                field_name,
                original_field,
                field_type,
                is_array,
                is_optional,
                is_rename,
            };

            result_fields.push(field);
        }

        let struct_ = Rc::new(StructType {
            name: struct_name.clone(),
            is_camel_case,
            fields: result_fields,
        });
        type_registry.insert(struct_name, struct_.clone());
        structs.push(struct_);
    }

    println!("use serde::{{Deserialize, Serialize}};");
    println!("use serde_json::Value;");
    println!("use std::{{borrow::Cow, collections::HashMap}};\n");

    for e in enums {
        println!("#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]");
        if e.is_upper_case {
            println!(r#"#[serde(rename_all = "UPPERCASE")]"#);
        }

        println!("pub enum {} {{", e.name);

        for field in e.fields {
            println!("    {},", field);
        }

        println!("}}\n");
    }

    for s in structs {
        let is_lifetime = s.is_lifetime(&type_registry);
        println!("#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]");
        if s.is_camel_case {
            println!(r#"#[serde(rename_all = "camelCase")]"#);
        }
        println!(
            "pub struct {}{} {{",
            s.name,
            if is_lifetime { "<'a>" } else { "" }
        );

        for field in &s.fields {
            if field.is_rename {
                println!(r#"    #[serde(rename = "{}")]"#, field.original_field);
            }
            let mut field_type = field.field_type.name(&type_registry);
            if field.is_array {
                field_type = format!("Vec<{}>", field_type);
            }
            if field.is_optional {
                field_type = format!("Option<{}>", field_type);
            }
            println!("    pub {}: {},", field.field_name, field_type);
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

struct StructType {
    name: String,
    is_camel_case: bool,
    fields: Vec<Field>,
}

struct EnumType {
    name: String,
    is_upper_case: bool,
    fields: Vec<String>,
}

impl StructType {
    fn is_lifetime(&self, type_registry: &HashMap<String, Rc<StructType>>) -> bool {
        self.fields
            .iter()
            .any(|x| x.field_type.is_lifetime(type_registry))
    }
}

struct Field {
    field_name: String,
    original_field: String,
    is_optional: bool,
    is_array: bool,
    is_rename: bool,
    field_type: FieldType,
}

enum FieldType {
    Simple(String),
    WithLifetime(String),
    Registry(String),
}

impl FieldType {
    fn is_lifetime(&self, type_registry: &HashMap<String, Rc<StructType>>) -> bool {
        match self {
            FieldType::WithLifetime(_) => true,
            FieldType::Registry(t) => type_registry
                .get(t)
                .map(|s| s.is_lifetime(type_registry))
                .unwrap_or_default(),
            FieldType::Simple(_) => false,
        }
    }

    fn name(&self, type_registry: &HashMap<String, Rc<StructType>>) -> String {
        match self {
            FieldType::Registry(name) => format!(
                "{}{}",
                name,
                if self.is_lifetime(type_registry) {
                    "<'a>"
                } else {
                    ""
                }
            ),
            FieldType::WithLifetime(name) | FieldType::Simple(name) => name.clone(),
        }
    }
}

fn convert_type(original: &str) -> Result<FieldType, ConvertTypeFail> {
    Ok(match original {
        "string" => FieldType::WithLifetime("Cow<'a, str>".into()),
        "integer(int32)" => FieldType::Simple("i32".into()),
        "integer(int64)" => FieldType::Simple("i64".into()),
        "number(float)" => FieldType::Simple("f32".into()),
        "boolean" => FieldType::Simple("bool".into()),
        "Map" => FieldType::WithLifetime("HashMap<Cow<'a, str>, Cow<'a, str>>".into()),
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
    Unknown(String),
}
