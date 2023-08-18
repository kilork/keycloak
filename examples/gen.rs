use core::panic;
use heck::*;
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, fs::read_to_string, rc::Rc, str::FromStr};
use structopt::StructOpt;

const RESERVED_WORDS: &[&str] = &["type", "self", "use"];

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
                field_name = format!("{}_", field_name);
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

fn write_types(enums: &[EnumType], structs: &[Rc<StructType>]) {
    println!(r#"#[cfg(feature = "schemars")]"#);
    println!("use schemars::JsonSchema;");
    println!("use serde::{{Deserialize, Serialize}};");
    println!("use serde_json::Value;");
    println!("use serde_with::skip_serializing_none;");
    println!("use std::collections::HashMap;\n");

    for e in enums {
        println!("#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]");
        println!(r#"#[cfg_attr(feature = "schemars", derive(JsonSchema))]"#);
        if e.is_upper_case {
            println!(r#"#[serde(rename_all = "UPPERCASE")]"#);
        }

        println!("pub enum {} {{", e.name);

        for field in &e.fields {
            println!("    {},", field);
        }

        println!("}}\n");
    }

    for s in structs {
        println!("#[skip_serializing_none]");
        println!("#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]");
        println!(r#"#[cfg_attr(feature = "schemars", derive(JsonSchema))]"#);
        if s.is_camel_case {
            println!(r#"#[serde(rename_all = "camelCase")]"#);
        }
        println!("pub struct {} {{", s.name,);

        for field in &s.fields {
            let is_rename = match field.field_case {
                FieldCase::Custom => true,
                FieldCase::Unknown => false,
                FieldCase::CamelCase => !s.is_camel_case,
                FieldCase::SnakeCase => s.is_camel_case,
            };
            if is_rename {
                println!(r#"    #[serde(rename = "{}")]"#, field.original_field);
            }
            let mut field_type = field.field_type.name();
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
}

fn process_method_parameters(
    method: &MethodStruct,
    mapping: &mut HashMap<String, (String, Rc<Parameter>)>,
    body_parameter: &mut Option<Vec<(String, Rc<Parameter>)>>,
    has_query_params: &mut bool,
    is_form: &mut bool,
) -> Vec<String> {
    let mut parameters = vec![];
    for parameter in &method.parameters {
        let mut name = parameter.name.to_snake_case();
        if ["ref", "type"].contains(&name.as_str()) {
            name += "_";
        }
        mapping.insert(parameter.name.clone(), (name.clone(), parameter.clone()));
        parameters.push((name, parameter.clone()));
    }

    let mut path_params = vec![];
    let path_parts = method
        .path
        .split('/')
        .filter(|x| x.starts_with('{') && x.ends_with('}'));
    for path_part in path_parts {
        let parameter_name = &path_part[1..path_part.len() - 1];
        if let Some(parameter_position) = parameters
            .iter()
            .position(|(_, p)| p.name == parameter_name)
        {
            let p = parameters.remove(parameter_position);
            path_params.push(p);
        }
    }

    let mut param_lines = vec![];
    for (name, parameter) in path_params.iter().chain(parameters.iter()) {
        let mut parameter_type = parameter.parameter_type.name();
        if parameter.is_array {
            parameter_type = format!("Vec<{}>", parameter_type);
        }
        if parameter.is_optional {
            parameter_type = format!("Option<{}>", parameter_type);
        }
        if !parameter.is_array && !parameter.is_optional && parameter_type == "String" {
            parameter_type = "&str".into();
        }
        param_lines.push(format!("        {}: {},", name, parameter_type));
        if let ParameterKind::FormData = parameter.kind {
            *is_form = true;
        }
        match parameter.kind {
            ParameterKind::Body | ParameterKind::FormData => {
                if body_parameter.is_none() {
                    *body_parameter = Some(vec![(name.clone(), parameter.clone())]);
                } else {
                    body_parameter
                        .iter_mut()
                        .next()
                        .unwrap()
                        .push((name.clone(), parameter.clone()));
                }
            }
            ParameterKind::Query => *has_query_params = true,
            _ => (),
        }
    }
    param_lines
}

fn write_rest(methods: &[MethodStruct], stream_mapping: &HashMap<String, String>) {
    let keycloak_version =
        std::env::var("KEYCLOAK_VERSION").expect("environment variable KEYCLOAK_VERSION");
    println!("use serde_json::{{json, Value}};");
    println!("use std::collections::HashMap;\n");
    println!("use reqwest::header::CONTENT_LENGTH;\n");
    println!("use super::*;\n");
    println!("impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {{");

    for method in methods {
        let mut method_name = method.path.clone();

        let parameters = &method.parameters;

        for parameter in parameters {
            if let ParameterKind::Path = parameter.kind {
                let parameter_with = (if parameter.name == "realm" {
                    ""
                } else {
                    "with_"
                })
                .to_string()
                    + parameter.name.to_snake_case().as_str();
                method_name = method_name
                    .replace(&format!("{{{}}}", parameter.name), parameter_with.as_str());
            }
        }
        method_name = (method_name + &method.method).to_snake_case();

        let mut mapping = HashMap::new();
        let mut body_parameter = None;
        let mut has_query_params = false;
        let mut is_form = false;
        let method_params = process_method_parameters(
            method,
            &mut mapping,
            &mut body_parameter,
            &mut has_query_params,
            &mut is_form,
        );
        let mut path = method.path.clone();
        let path_parts = method
            .path
            .split('/')
            .filter(|x| x.starts_with('{') && x.ends_with('}'));
        if method.parameters.is_empty() {
            for path_part in path_parts {
                path = path.replace(path_part, "");
            }
        } else {
            for path_part in path_parts {
                if let Some((s, _)) = mapping.get(&path_part[1..path_part.len() - 1]) {
                    path = path.replace(path_part, &format!("{{{s}}}"));
                }
            }
        }
        let rest_comment = format!("{} {}", method.method, path);

        let mut comments = vec![];
        if method.name != rest_comment {
            comments.push(vec![method.name.to_string()]);
        }
        if let Some(description) = method.description.as_ref().map(|x| x.replace('\n', " ")) {
            comments.push(vec![description]);
        }
        if !parameters.is_empty() {
            comments.push(vec!["Parameters:".to_string()]);
            comments.push(
                parameters
                    .iter()
                    .map(|parameter| {
                        format!(
                            "- `{}`{}",
                            match parameter.kind {
                                ParameterKind::Query =>
                                    &mapping.get(parameter.name.as_str()).unwrap().0,
                                _ => &parameter.name,
                            }
                            .to_snake_case(),
                            parameter
                                .comment
                                .as_ref()
                                .map(|comment| format!(": {}", comment.replace('\n', "")))
                                .unwrap_or_default()
                        )
                    })
                    .collect(),
            );
        }
        comments.push(vec![format!("Resource: `{}`", method.comment)]);
        comments.push(vec![format!("`{}`", rest_comment)]);
        if let Some(anchor) = &method.anchor {
            comments.push(vec![format!(
                "Documentation: <https://www.keycloak.org/docs-api/{}/rest-api/index.html#{}>",
                keycloak_version, anchor
            )]);
        }
        if path != method.path {
            comments.push(vec![format!(
                "REST method: `{} {}`",
                method.method, method.path
            )]);
        }
        let comments = comments
            .into_iter()
            .map(|c| {
                c.into_iter()
                    .map(|l| format!("    /// {}\n", l))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("    ///\n");
        print!("{}", comments);

        println!("    pub async fn {}(", method_name);
        println!("        &self,");
        for method_param in method_params {
            println!("{}", method_param);
        }

        let mut response_type = method.response.return_type.name().replace("'a", "'static");
        if method.response.is_array {
            response_type = format!("Vec<{}>", response_type);
        }
        if response_type == "Stream" {
            let stream_type = stream_mapping
                .get(&method.path)
                .map(|f| {
                    FieldType::Registry(f.clone())
                        .name()
                        .replace("'a", "'static")
                })
                .map(|x| format!("Vec<{}>", x));
            if stream_type.is_none() {
                panic!("Stream for {} not found", method.path);
            }
            response_type = stream_type.expect("stream not found");
        }
        println!("    ) -> Result<{}, KeycloakError> {{", response_type);
        println!(
            "        let {}builder = self",
            if has_query_params { "mut " } else { "" }
        );
        println!("            .client");

        let method_http = match method.method.as_str() {
            "OPTIONS" => "request(reqwest::Method::OPTIONS, ".to_string(),
            _ => method.method.to_lowercase() + "(",
        };
        println!(
            r#"            .{}&format!("{{}}/admin/realms{}", self.url))"#,
            method_http, path,
        );

        if let Some(x) = body_parameter {
            if is_form {
                println!("            .form(&json!({{",);
                for (name, parameter) in x {
                    println!(r#"                "{}": {},"#, parameter.name, name);
                }
                println!("            }}))",);
            } else if let Some((name, _parameter)) = x.get(0) {
                println!("            .json(&{})", name);
            }
        } else if method.method.eq("PUT") {
            println!("            .header( CONTENT_LENGTH, \"0\")"); // PUT without body expects Content-Length header to be 0
        }

        println!("            .bearer_auth(self.token_supplier.get(&self.url).await?);");

        for parameter in parameters {
            if let ParameterKind::Query = parameter.kind {
                println!(
                    "        if let Some(v) = {} {{",
                    mapping.get(parameter.name.as_str()).unwrap().0
                );
                println!(
                    r#"            builder = builder.query(&[("{}", v)]);"#,
                    parameter.name
                );
                println!("        }}");
            }
        }

        println!("        let response = builder.send().await?;");

        match response_type.as_str() {
            "()" => {
                println!("        error_check(response).await?;");
                println!("        Ok(())");
            }
            typ => {
                let (des, postfix) = match typ {
                    "Vec<u8>" => ("bytes", ".to_vec()"),
                    _ => ("json", ""),
                };
                println!(
                    "        Ok(error_check(response).await?.{}().await?{})",
                    des, postfix
                );
            }
        }

        println!("    }}\n");
    }

    println!("}}");
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

struct Field {
    field_name: String,
    original_field: String,
    is_optional: bool,
    is_array: bool,
    field_case: FieldCase,
    field_type: FieldType,
}

#[derive(Debug)]
enum FieldCase {
    CamelCase,
    SnakeCase,
    Custom,
    Unknown,
}

#[derive(Debug)]
enum FieldType {
    Simple(String),
    Registry(String),
}

impl FieldType {
    fn name(&self) -> String {
        match self {
            FieldType::Registry(name) | FieldType::Simple(name) => name.clone(),
        }
    }
}

struct Parameter {
    name: String,
    is_optional: bool,
    comment: Option<String>,
    kind: ParameterKind,
    is_array: bool,
    parameter_type: FieldType,
}

#[derive(Debug)]
enum ParameterKind {
    Path,
    Query,
    Body,
    FormData,
}

impl FromStr for ParameterKind {
    type Err = String;
    fn from_str(value: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match value {
            "Path" => Ok(ParameterKind::Path),
            "Query" => Ok(ParameterKind::Query),
            "Body" => Ok(ParameterKind::Body),
            "FormData" => Ok(ParameterKind::FormData),
            _ => Err(format!("Unknown parameter kind: {}", value)),
        }
    }
}

struct MethodStruct {
    anchor: Option<String>,
    name: String,
    comment: String,
    path: String,
    parameters: Vec<Rc<Parameter>>,
    method: String,
    response: ResponseType,
    description: Option<String>,
}

#[derive(Debug)]
struct ResponseType {
    is_array: bool,
    return_type: FieldType,
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
    Unknown(String),
}
