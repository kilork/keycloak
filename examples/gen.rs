use heck::*;
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, fs::read_to_string, rc::Rc, str::FromStr};
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

fn parse_document() -> Result<Html, std::io::Error> {
    Ok(Html::parse_document(&read_to_string(
        "./docs/rest-api.html",
    )?))
}

fn generate_rest() -> Result<(), std::io::Error> {
    let document = parse_document()?;
    let (_, _, type_registry) = read_types_info(&document)?;
    let methods = read_methods_info(&document)?;
    write_rest(&type_registry, &methods);
    Ok(())
}

fn generate_types() -> Result<(), std::io::Error> {
    let document = parse_document()?;
    let (enums, structs, type_registry) = read_types_info(&document)?;
    write_types(&enums, &structs, &type_registry);
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

type TypeTrio = (
    Vec<EnumType>,
    Vec<Rc<StructType>>,
    HashMap<String, Rc<StructType>>,
);
fn read_types_info(document: &scraper::Html) -> Result<TypeTrio, std::io::Error> {
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

            let array_field = check_array(&original_field_type);
            let is_array = array_field.is_some();

            let field_type = match if is_array {
                convert_type(array_field.unwrap())
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

            let is_optional = check_optional(&optional_required);

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

    Ok((enums, structs, type_registry))
}

fn read_methods_info(document: &scraper::Html) -> Result<Vec<MethodStruct>, std::io::Error> {
    let resources_selector = Selector::parse("#_paths ~ div.sectionbody > div.sect2").unwrap();

    let resources_html = document.select(&resources_selector);
    let mut methods = vec![];
    for resource in resources_html {
        let resource_name = text(&resource, "h3");

        let methods_selector = Selector::parse("div.sect3").unwrap();

        let methods_html = resource.select(&methods_selector);
        for method in methods_html {
            let method_name = text(&method, "h4");
            let path = text_opt(&method, "pre").unwrap_or_else(|| method_name.clone());
            let mut path_parts = path.split(" ");
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
                            let parameter_ = Parameter {
                                name,
                                comment,
                                is_optional,
                                is_array: array.is_some(),
                                kind: parameter_kind.parse().unwrap(),
                                parameter_type: array
                                    .or_else(|| Some(parameter_type.as_str()))
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
                                .or_else(|| Some(response_type.as_str()))
                                .map(|x| convert_type(&x.replace("-", "")))
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

fn write_types(
    enums: &[EnumType],
    structs: &[Rc<StructType>],
    type_registry: &HashMap<String, Rc<StructType>>,
) {
    println!("use serde::{{Deserialize, Serialize}};");
    println!("use serde_json::Value;");
    println!("use std::{{borrow::Cow, collections::HashMap}};\n");

    for e in enums {
        println!("#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]");
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
}

fn renames() -> HashMap<(&'static str, &'static str), String> {
    let mut rename_methods_table = HashMap::new();
    rename_methods_table.insert(
        (
            "/{realm}/attack-detection/brute-force/users/{userId}",
            "DELETE",
        ),
        "attack_detection_brute_force_user_delete".to_string(),
    );
    rename_methods_table.insert(
        ("/{realm}/client-scopes/{id}", "GET"),
        "client_scope_get".to_string(),
    );
    rename_methods_table.insert(("/{realm}/clients/{id}", "GET"), "client_get".to_string());
    rename_methods_table.insert(
        ("/{realm}/components/{id}", "GET"),
        "component_get".to_string(),
    );
    rename_methods_table.insert(("/{realm}/groups/{id}", "GET"), "group_get".to_string());
    rename_methods_table.insert(
        ("/{realm}/identity-provider/instances/{alias}", "GET"),
        "identity_provider_instance_get".to_string(),
    );
    rename_methods_table.insert(
        (
            "/{realm}/identity-provider/instances/{alias}/mappers/{id}",
            "GET",
        ),
        "identity_provider_instances_mapper_get".to_string(),
    );
    rename_methods_table.insert(
        (
            "/{realm}/client-scopes/{id}/protocol-mappers/models/{id}",
            "GET",
        ),
        "client_scopes_protocol_mappers_model_get".to_string(),
    );
    rename_methods_table.insert(
        ("/{realm}/clients/{id}/protocol-mappers/models/{id}", "GET"),
        "clients_protocol_mappers_model_get".to_string(),
    );
    rename_methods_table.insert(
        ("/{realm}/clients/{id}/roles/{role-name}", "GET"),
        "clients_role_get".to_string(),
    );
    rename_methods_table.insert(
        ("/{realm}/roles/{role-name}", "GET"),
        "role_get".to_string(),
    );
    rename_methods_table.insert(("/{realm}/users/{id}", "GET"), "user_get".to_string());
    rename_methods_table.insert(("/", "GET"), "root_get".to_string());

    rename_methods_table.insert(
        ("/{realm}/authentication/required-actions/{alias}", "GET"),
        "authentication_required_action_get".to_string(),
    );
    rename_methods_table.insert(
        ("/{realm}/authentication/flows/{id}", "GET"),
        "authentication_flow_get".to_string(),
    );
    rename_methods_table
}

fn process_method_parameters(
    type_registry: &HashMap<String, Rc<StructType>>,
    method: &MethodStruct,
    mapping: &mut HashMap<String, (String, Rc<Parameter>)>,
    body_parameter: &mut Option<Vec<(String, Rc<Parameter>)>>,
    has_query_params: &mut bool,
    is_form: &mut bool,
) {
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

    for (name, parameter) in path_params.iter().chain(parameters.iter()) {
        let mut parameter_type = parameter
            .parameter_type
            .name(&type_registry)
            .replace("'a", "'_")
            .replace("Cow<'_, str>", "&str");
        if parameter.is_array {
            parameter_type = format!("Vec<{}>", parameter_type);
        }
        if parameter.is_optional {
            parameter_type = format!("Option<{}>", parameter_type);
        }
        println!("        {}: {},", name, parameter_type);
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
}

fn write_rest(type_registry: &HashMap<String, Rc<StructType>>, methods: &[MethodStruct]) {
    let rename_methods_table = renames();

    println!("use serde_json::{{json, Value}};");
    println!("use std::{{borrow::Cow, collections::HashMap}};\n");
    println!("use super::*;\n");
    println!("impl<'a> KeycloakAdmin<'a> {{");

    for method in methods {
        let mut method_name = method.path.clone();
        if let Some(redefined_method_name) =
            rename_methods_table.get(&(method_name.as_str(), method.method.as_str()))
        {
            method_name = redefined_method_name.clone()
        } else {
            for parameter in &method.parameters {
                if let ParameterKind::Path = parameter.kind {
                    method_name = method_name.replace(&format!("{{{}}}", parameter.name), "");
                }
            }
            method_name = (method_name + &method.method).to_snake_case();
        }
        let rest_comment = format!("{} {}", method.method, method.path);

        if method.name != rest_comment {
            println!("    /// {}", method.name);
        }
        if let Some(description) = method.description.as_ref().map(|x| x.replace("\n", " ")) {
            println!("    /// {}", description);
        }
        println!("    /// {}", rest_comment);

        println!("    pub async fn {}(", method_name);
        println!("        &self,");

        let mut mapping = HashMap::new();
        let mut body_parameter = None;
        let mut has_query_params = false;
        let mut is_form = false;
        process_method_parameters(
            type_registry,
            method,
            &mut mapping,
            &mut body_parameter,
            &mut has_query_params,
            &mut is_form,
        );

        let mut response_type = method
            .response
            .return_type
            .name(type_registry)
            .replace("'a", "'_");
        if method.response.is_array {
            response_type = format!("Vec<{}>", response_type);
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
        let mut path = method.path.clone();
        let mut path_params = vec![];
        let path_parts = method
            .path
            .split('/')
            .filter(|x| x.starts_with('{') && x.ends_with('}'));
        for path_part in path_parts {
            if let Some((s, _)) = mapping.get(&path_part[1..path_part.len() - 1]) {
                path_params.push(s.as_str());
            }
            path = path.replace(
                path_part,
                if method.parameters.is_empty() {
                    ""
                } else {
                    "{}"
                },
            );
        }
        println!(
            r#"            .{}&format!("{{}}/auth/admin/realms{}", self.url, {}))"#,
            method_http,
            path,
            path_params.join(", ")
        );

        if let Some(x) = body_parameter {
            if is_form {
                println!("            .form(&json!({{",);
                for (name, parameter) in x {
                    println!(r#"                "{}": {},"#, parameter.name, name);
                }
                println!("            }}))",);
            } else if let Some((name, _parameter)) = x.iter().next() {
                println!("            .json(&{})", name);
            }
        }

        println!("            .bearer_auth(self.admin_token.get(&self.url).await?);");

        for parameter in &method.parameters {
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

struct Parameter {
    name: String,
    is_optional: bool,
    comment: Option<String>,
    kind: ParameterKind,
    is_array: bool,
    parameter_type: FieldType,
}

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
    name: String,
    comment: String,
    path: String,
    parameters: Vec<Rc<Parameter>>,
    method: String,
    response: ResponseType,
    description: Option<String>,
}

struct ResponseType {
    is_array: bool,
    return_type: FieldType,
}

fn convert_type(original: &str) -> Result<FieldType, ConvertTypeFail> {
    Ok(match original {
        "No Content" | "Response" => FieldType::Simple("()".into()),
        "file" => FieldType::Simple("&[u8]".into()),
        "string" | "< string > array(csv)" => FieldType::WithLifetime("Cow<'a, str>".into()),
        "string(byte)" => FieldType::Simple("u8".into()),
        "integer(int32)" => FieldType::Simple("i32".into()),
        "integer(int64)" => FieldType::Simple("i64".into()),
        "number(float)" => FieldType::Simple("f32".into()),
        "boolean" => FieldType::Simple("bool".into()),
        "Map" => FieldType::WithLifetime("HashMap<Cow<'a, str>, Value>".into()),
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
