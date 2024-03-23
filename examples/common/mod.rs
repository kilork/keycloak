use std::{collections::HashMap, rc::Rc, str::FromStr};

use heck::ToSnakeCase;

pub struct StructType {
    pub name: String,
    pub is_camel_case: bool,
    pub fields: Vec<Field>,
}

pub struct EnumType {
    pub name: String,
    pub is_upper_case: bool,
    pub fields: Vec<String>,
}

pub struct Field {
    pub field_name: String,
    pub original_field: String,
    pub is_optional: bool,
    pub is_array: bool,
    pub field_case: FieldCase,
    pub field_type: FieldType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldCase {
    CamelCase,
    SnakeCase,
    Custom,
    Unknown,
}

#[derive(Debug)]
pub enum FieldType {
    Simple(String),
    Registry(String),
}

impl FieldType {
    pub fn name(&self) -> String {
        match self {
            FieldType::Registry(name) | FieldType::Simple(name) => name.clone(),
        }
    }
}

pub struct Parameter {
    pub name: String,
    pub is_optional: bool,
    pub comment: Option<String>,
    pub kind: ParameterKind,
    pub is_array: bool,
    pub parameter_type: FieldType,
}

#[derive(Debug)]
pub enum ParameterKind {
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

pub struct MethodStruct {
    pub anchor: Option<String>,
    pub name: String,
    pub comment: String,
    pub path: String,
    pub parameters: Vec<Rc<Parameter>>,
    pub method: String,
    pub response: ResponseType,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct ResponseType {
    pub is_array: bool,
    pub return_type: FieldType,
}

pub fn write_types(enums: &[EnumType], structs: &[Rc<StructType>]) {
    println!("use std::collections::HashMap;\n");
    println!();
    println!(r#"#[cfg(feature = "schemars")]"#);
    println!("use schemars::JsonSchema;");
    println!("use serde::{{Deserialize, Serialize}};");
    println!("use serde_json::Value;");
    println!("use serde_with::skip_serializing_none;");
    println!();

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

pub fn write_rest(methods: &[MethodStruct], stream_mapping: &HashMap<String, String>) {
    let keycloak_version =
        std::env::var("KEYCLOAK_VERSION").expect("environment variable KEYCLOAK_VERSION");
    println!("use std::collections::HashMap;");
    println!();
    println!("use reqwest::header::CONTENT_LENGTH;");
    println!("use serde_json::Value;");
    println!();
    println!("use super::*;");
    println!();
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
            } else if let Some((name, _parameter)) = x.first() {
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
