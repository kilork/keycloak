use clap::Parser;

mod common;

/// Generate Rust code from Keycloak REST Description in HTML
#[derive(Parser)]
enum Cli {
    /// Generate types
    Types,
    /// Generate method callers
    Rest,
    /// Specs
    Specs,
}

const RESERVED_WORDS: &[&str] = &["type", "self", "static", "use"];

mod openapi {
    use std::{borrow::Cow, fmt::Display, str::FromStr};

    use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
    use indexmap::IndexMap;
    use serde::Deserialize;

    use crate::{common::FieldCase, RESERVED_WORDS};

    #[derive(Debug, Deserialize)]
    pub struct Spec {
        pub openapi: String,
        pub info: Info,
        pub tags: Vec<Tag>,
        pub paths: IndexMap<String, SpecPath>,
        pub components: Components,
    }

    #[derive(Debug, Deserialize)]
    pub struct Info {
        pub title: String,
        pub description: String,
        pub version: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Tag {
        pub name: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct SpecPath {
        #[serde(flatten)]
        pub calls: IndexMap<Method, Call>,
        pub parameters: Option<Vec<Parameter>>,
    }

    impl SpecPath {
        pub fn to_rust_method(&self, path: &str) -> String {
            self.calls
                .iter()
                .map(|(method, call)| call.to_rust_method(path, method, self.parameters.as_deref()))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    #[derive(Debug, Deserialize)]
    struct RequestBody {
        content: Content,
    }

    impl RequestBody {
        fn to_rust_parameter_type(&self) -> Option<Cow<'_, str>> {
            self.content.to_rust_parameter_type()
        }

        fn to_rust_reqwest_body_calls(&self, body_name: &str) -> Option<Vec<String>> {
            self.content.to_rust_reqwest_body_calls(body_name)
        }
    }

    #[derive(Debug, Deserialize)]
    struct Response {
        content: Option<Content>,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
    enum ContentType {
        #[serde(rename = "application/json")]
        ApplicationJson,
        #[serde(rename = "application/octet-stream")]
        ApplicationOctetStream,
        #[serde(rename = "application/xml")]
        ApplicationXml,
        #[serde(rename = "text/plain")]
        TextPlain,
        #[serde(rename = "application/x-www-form-urlencoded")]
        HtmlForm,
        #[serde(rename = "*/*")]
        Any,
    }

    #[derive(Debug, Deserialize)]
    struct Content(IndexMap<ContentType, ContentSchema>);

    impl Content {
        fn as_json(&self) -> Option<&ContentSchema> {
            self.0.get(&ContentType::ApplicationJson)
        }

        fn as_text_plain(&self) -> Option<&ContentSchema> {
            self.0.get(&ContentType::TextPlain)
        }

        fn as_binary_text(&self) -> Option<&ContentSchema> {
            self.as_binary_blob()
                .filter(|content_schema| content_schema.is_string())
        }

        fn as_binary_blob(&self) -> Option<&ContentSchema> {
            self.0.get(&ContentType::ApplicationOctetStream)
        }

        fn as_any(&self) -> Option<&ContentSchema> {
            self.0.get(&ContentType::Any)
        }

        fn as_content_schema(&self) -> Option<&ContentSchema> {
            self.as_json()
                .or(self.as_text_plain())
                .or(self.as_binary_text())
                .or(self.as_any())
        }

        fn to_rust_parameter_type(&self) -> Option<Cow<'_, str>> {
            self.as_content_schema()
                .map(|content_schema| content_schema.to_rust_parameter_body_type())
        }

        fn to_rust_return_type(&self) -> Option<Cow<'_, str>> {
            self.as_content_schema()
                .map(|content_schema| content_schema.to_rust_return_type())
        }

        fn to_rust_reqwest_body_calls(&self, body_name: &str) -> Option<Vec<String>> {
            self.as_json()
                .map(|_| vec![format!("json(&{body_name})")])
                .or(self
                    .as_text_plain()
                    .map(|_| vec![format!("body({body_name})")]))
        }

        fn to_rust_reqwest_parse_body_call(&self) -> Option<(Cow<'_, str>, Option<Cow<'_, str>>)> {
            self.as_json()
                .or(self.as_any())
                .map(|_| ("json", None))
                .or(self
                    .as_binary_text()
                    .or(self.as_text_plain())
                    .map(|_| ("text", Some(".map(From::from)"))))
                .or(self.as_binary_blob().map(|_| ("bytes", None)))
                .map(|(method, conv)| (method.into(), conv.map(From::from)))
        }
    }

    #[derive(Debug, Deserialize)]
    struct ContentSchema {
        schema: Kind,
    }

    impl ContentSchema {
        fn to_rust_parameter_body_type(&self) -> Cow<str> {
            self.schema.to_rust_type_ref(RefMode::Std)
        }

        fn to_rust_return_type(&self) -> Cow<str> {
            self.schema.to_rust_type_ref(RefMode::Owned)
        }

        fn is_string(&self) -> bool {
            matches!(self.schema, Kind::Generic(Generic::String))
        }
    }

    #[derive(Debug, Deserialize)]
    struct Responses(IndexMap<String, Response>);

    impl Responses {
        fn to_reqwest_status_response(&self) -> Option<(reqwest::StatusCode, &Response)> {
            if self.0.len() != 1 {
                return None;
            }
            let (status, response) = self.0.first()?;
            let status = reqwest::StatusCode::from_str(status).ok()?;
            Some((status, response))
        }

        fn to_rust_return_type(&self) -> Option<Cow<str>> {
            let (_, response) = self.to_reqwest_status_response()?;
            if response.content.is_none() {
                return Some("()".into());
            }
            response.content.as_ref()?.to_rust_return_type()
        }

        fn to_rust_reqwest_parse_body_call(&self) -> Option<(Cow<'_, str>, Option<Cow<'_, str>>)> {
            let (_, response) = self.to_reqwest_status_response()?;
            response.content.as_ref()?.to_rust_reqwest_parse_body_call()
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Call {
        pub tags: Vec<String>,
        summary: Option<String>,
        #[serde(default)]
        deprecated: bool,
        parameters: Option<Vec<Parameter>>,
        request_body: Option<RequestBody>,
        responses: Responses,
    }

    impl Call {
        fn to_rust_method(
            &self,
            path: &str,
            method: &Method,
            parameters: Option<&[Parameter]>,
        ) -> String {
            let mut method_name = path
                .strip_prefix("/admin/realms/")
                .unwrap_or(path)
                .to_string();

            let mut path_snake_case = path.to_string();

            let parameters = parameters
                .into_iter()
                .chain(self.parameters.as_deref())
                .flatten()
                .map(|parameter| {
                    let mut param_name = parameter.name.to_snake_case();
                    while RESERVED_WORDS.contains(&param_name.as_str()) {
                        param_name += "_";
                    }

                    (parameter, param_name)
                })
                .collect::<Vec<_>>();
            for (parameter, parameter_name) in &parameters {
                if parameter.position == ParameterPosition::Path {
                    let parameter_with = if parameter.name == "realm" {
                        ""
                    } else {
                        "with_"
                    }
                    .to_string()
                        + parameter_name.as_str();
                    method_name =
                        method_name.replace(&format!("{{{}}}", parameter.name), &parameter_with);
                }
                if parameter_name != &parameter.name {
                    path_snake_case = path_snake_case.replace(
                        &format!("{{{}}}", parameter.name),
                        &format!("{{{parameter_name}}}"),
                    );
                }
            }

            let method_string = method.to_string();

            method_name = (method_name + &method_string).to_snake_case();

            let mut comments: Vec<Vec<Cow<str>>> = vec![];

            if let Some(comment) = self
                .summary
                .as_ref()
                .map(|s| s.trim().lines().map(Cow::Borrowed).collect())
            {
                comments.push(comment);
            }
            let has_request_body = self.request_body.is_some();
            if !parameters.is_empty() || has_request_body {
                comments.push(vec!["Parameters:".into()]);
                comments.push(
                    parameters
                        .iter()
                        .map(|(parameter, parameter_name)| {
                            format!(
                                "- `{parameter_name}`{}",
                                parameter
                                    .description
                                    .as_ref()
                                    .map(|comment| format!(": {}", comment.replace('\n', "")))
                                    .unwrap_or_default()
                            )
                        })
                        .map(Cow::Owned)
                        .chain(if has_request_body {
                            vec![Cow::Borrowed("- `body`")]
                        } else {
                            vec![]
                        })
                        .collect(),
                );
            }
            if let [tag] = self.tags.as_slice() {
                comments.push(vec![format!("Resource: `{}`", tag).into()]);
            }
            comments.push(vec![format!(
                "`{} {path_snake_case}`",
                method_string.to_ascii_uppercase()
            )
            .into()]);

            let mut anchor_suffix = path.replace('-', "_");
            anchor_suffix.retain(|c| !"{}/".contains(c));
            let anchor_suffix = anchor_suffix.to_ascii_lowercase();
            let keycloak_version = format!(
                "{}.{}.{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH")
                    .parse::<u64>()
                    .expect("number")
                    / 100,
            );

            let method_string_lc = method_string.to_ascii_lowercase();
            let anchor = format!("_{method_string_lc}_{anchor_suffix}");
            comments.push(vec![format!(
                "Documentation: <https://www.keycloak.org/docs-api/{keycloak_version}/rest-api/index.html#{anchor}>"
            )
            .into()]);

            if path_snake_case != path {
                comments.push(vec![format!(
                    "REST method: `{} {path}`",
                    method_string.to_ascii_uppercase()
                )
                .into()]);
            }

            let comments: Vec<_> = comments
                .into_iter()
                .map(|c| {
                    c.into_iter()
                        .map(|l| format!("/// {}\n", l))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("///\n")
                .lines()
                .map(ToString::to_string)
                .collect();

            let mut output = vec![];

            output.extend(comments);

            if self.deprecated {
                output.push("#[deprecated]".into());
            }

            let request_body = self.request_body.as_ref();
            let parameters_of_method = parameters
                .iter()
                .map(|(parameter, param_name)| {
                    let param_type = parameter.schema.to_rust_parameter_type(parameter.required);

                    format!("{param_name}: {param_type}")
                })
                .chain(
                    request_body
                        .and_then(|body| body.to_rust_parameter_type())
                        .map(|param_type| format!("body: {param_type}")),
                )
                .map(|param_line| format!("    {param_line},"))
                .collect::<Vec<_>>();

            if parameters_of_method.len() > 6 {
                output.push("#[allow(clippy::too_many_arguments)]".into());
            }

            output.push(format!("pub async fn {method_name}("));
            output.push("    &self,".into());

            // fill parameters

            output.extend(parameters_of_method);

            let Some(result_type) = self.responses.to_rust_return_type() else {
                panic!(
                    "{path}: could not determine result type for responses: {:?}",
                    self.responses
                );
            };

            output.push(format!(") -> Result<{result_type}, KeycloakError> {{"));

            let query_parameters = parameters
                .iter()
                .filter(|(parameter, _)| parameter.position == ParameterPosition::Query)
                .collect::<Vec<_>>();
            let has_query_parameters = !query_parameters.is_empty();

            output.push(
                if has_query_parameters {
                    "    let mut builder = self"
                } else {
                    "    let builder = self"
                }
                .into(),
            );
            output.push("        .client".into());
            output.push(format!("        .{method_string_lc}(&format!("));
            output.push(format!(r#"            "{{}}{path_snake_case}","#));
            output.push("            self.url".into());
            output.push("        ))".into());
            if let Some(reqwest_body) = request_body {
                let Some(reqwest_body_calls) = reqwest_body.to_rust_reqwest_body_calls("body")
                else {
                    panic!("could not convert reqwest body: {reqwest_body:?}")
                };
                for reqwest_body_call in reqwest_body_calls {
                    output.push(format!("        .{reqwest_body_call}"));
                }
            } else if matches!(method, Method::Put) {
                output.push(r#"        .header(CONTENT_LENGTH, "0")"#.into());
            }
            output.push("        .bearer_auth(self.token_supplier.get(&self.url).await?);".into());

            output.extend(query_parameters.into_iter().flat_map(
                |(query_parameter, query_parameter_name)| {
                    [
                        format!("if let Some(v) = {query_parameter_name} {{"),
                        format!(
                            r#"    builder = builder.query(&[("{}", v)]);"#,
                            query_parameter.name
                        ),
                        "}".into(),
                    ]
                    .map(|line| format!("    {line}"))
                },
            ));

            if let Some((parse_body_call, conv_body_call)) =
                self.responses.to_rust_reqwest_parse_body_call()
            {
                output.push("    let response = builder.send().await?;".into());
                output.push(format!(
                    "    Ok(error_check(response).await?.{parse_body_call}().await{}?)",
                    conv_body_call.unwrap_or_default()
                ));
            } else {
                output.push("    let response = builder.send().await?;".into());
                output.push("    error_check(response).await?;".into());
                output.push("    Ok(())".into());
            }

            output.push("}".into());

            output
                .into_iter()
                .map(|s| "    ".to_string() + &s)
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(rename_all = "lowercase")]
    pub enum Method {
        Delete,
        Get,
        Post,
        Put,
    }

    impl Display for Method {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Parameter {
        pub name: String,
        #[serde(rename = "in")]
        pub position: ParameterPosition,
        pub description: Option<String>,
        #[serde(default)]
        pub required: bool,
        #[serde(default)]
        pub deprecated: bool,
        pub schema: Kind,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum ParameterPosition {
        Path,
        Query,
    }

    #[derive(Debug, Deserialize)]
    pub struct Components {
        pub schemas: IndexMap<String, SchemaObj>,
    }

    #[derive(Debug, Deserialize)]
    pub struct SchemaObj {
        #[serde(default)]
        deprecated: bool,
        #[serde(flatten)]
        schema: Schema,
    }

    impl SchemaObj {
        pub fn to_rust_type_definition(&self, name: &str, ref_mode: RefMode) -> String {
            format!(
                "{}{}",
                if self.deprecated {
                    "#[deprecated]\n"
                } else {
                    ""
                },
                self.schema.to_rust_type_definition(name, ref_mode)
            )
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase", tag = "type")]
    pub enum Schema {
        Object(ObjectSchema<Property>),
        String(StringSchema),
    }

    impl Schema {
        pub fn to_rust_type_definition(&self, name: &str, ref_mode: RefMode) -> String {
            match self {
                Schema::Object(schema_obj) => schema_obj.to_rust_type_definition(name, ref_mode),
                Schema::String(schema_str) => schema_str.to_rust_type_definition(name),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum ObjectSchema<P> {
        Struct(SchemaStruct<P>),
        Map(SchemaMap<P>),
        AllOf(SchemaAllOf<P>),
        Value {},
    }

    impl ObjectSchema<Property> {
        fn to_rust_type_definition(&self, name: &str, ref_mode: RefMode) -> String {
            match self {
                ObjectSchema::Struct(schema_struct) => {
                    schema_struct.to_rust_type_definition(name, ref_mode)
                }
                ObjectSchema::Map(schema_map) => schema_map.to_rust_type_definition(name, ref_mode),
                ObjectSchema::AllOf(_) => todo!(),
                ObjectSchema::Value {} => {
                    format!("pub type {name} = TypeMap<String, TypeValue>;\n")
                }
            }
        }
    }

    impl ObjectSchema<Kind> {
        fn to_rust_type(&self, ref_mode: RefMode) -> Cow<str> {
            match self {
                ObjectSchema::Struct(schema_struct) => schema_struct.to_rust_type(ref_mode),
                ObjectSchema::Map(schema_map) => schema_map.to_rust_type(ref_mode),
                ObjectSchema::AllOf(schema_all_of) => schema_all_of.to_rust_type(ref_mode),
                ObjectSchema::Value {} => "Value".into(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct SchemaStruct<P> {
        pub properties: IndexMap<String, P>,
    }

    impl SchemaStruct<Property> {
        fn to_rust_type_definition(&self, name: &str, ref_mode: RefMode) -> String {
            let fields = self
                .properties
                .iter()
                .map(|(field, prop)| {
                    let mut field_name = field.to_snake_case();

                    let field_case = if RESERVED_WORDS.contains(&field_name.as_str()) {
                        field_name += "_";
                        FieldCase::Custom
                    } else if field == &field_name {
                        if field.contains('_') {
                            FieldCase::SnakeCase
                        } else {
                            FieldCase::Unknown
                        }
                    } else if field == &field.to_lower_camel_case() {
                        FieldCase::CamelCase
                    } else {
                        FieldCase::Custom
                    };
                    if field != &field_name {
                        while self.properties.contains_key(&field_name) {
                            field_name += "_";
                        }
                    }

                    (
                        field.as_str(),
                        field_name,
                        field_case,
                        prop.to_rust_type_opt(ref_mode),
                        prop.deprecated,
                    )
                })
                .collect::<Vec<_>>();

            let count_snake_case = fields
                .iter()
                .filter(|(_, _, field_case, _, _)| &FieldCase::SnakeCase == field_case)
                .count();
            let count_camel_case = fields
                .iter()
                .filter(|(_, _, field_case, _, _)| &FieldCase::CamelCase == field_case)
                .count();

            let (rename_to_camel_case, type_prefix) = if count_camel_case > count_snake_case {
                (
                    true,
                    r##"
#[serde(rename_all = "camelCase")]"##,
                )
            } else {
                (false, "")
            };
            format!(
                r##"#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]{}
pub struct {name} {{
{}
}}
"##,
                type_prefix,
                fields
                    .into_iter()
                    .map(|(field, field_name, field_case, field_type, deprecated)| {
                        let is_rename = match field_case {
                            FieldCase::Custom => true,
                            FieldCase::Unknown => false,
                            FieldCase::CamelCase => !rename_to_camel_case,
                            FieldCase::SnakeCase => rename_to_camel_case,
                        };

                        let field_desc = if !is_rename {
                            format!(r##"    pub {field_name}: {field_type},"##)
                        } else {
                            format!(
                                r##"    #[serde(rename = "{field}")]
    pub {field_name}: {field_type},"##,
                            )
                        };
                        if !deprecated {
                            field_desc
                        } else {
                            format!("    #[deprecated]\n{field_desc}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }

    impl SchemaStruct<Kind> {
        fn to_rust_type(&self, _: RefMode) -> Cow<str> {
            todo!()
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SchemaMap<P> {
        pub additional_properties: P,
    }

    impl SchemaMap<Property> {
        fn to_rust_type_definition(&self, name: &str, ref_mode: RefMode) -> String {
            format!(
                "type {name} = TypeMap<String, {}>;\n",
                self.additional_properties.to_rust_type(ref_mode)
            )
        }
    }

    impl SchemaMap<Kind> {
        fn to_rust_type(&self, ref_mode: RefMode) -> Cow<str> {
            format!(
                "TypeMap<String, {}>",
                self.additional_properties.to_rust_type(ref_mode)
            )
            .into()
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SchemaAllOf<P> {
        pub all_of: Vec<P>,
    }

    impl SchemaAllOf<Kind> {
        fn to_rust_type(&self, ref_mode: RefMode) -> Cow<str> {
            match &self.all_of.as_slice() {
                &[property] => property.to_rust_type(ref_mode),
                _ => todo!(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum StringSchema {
        Enum(Vec<String>),
    }

    impl StringSchema {
        fn to_rust_type_definition(&self, name: &str) -> String {
            match self {
                StringSchema::Enum(variants) => {
                    let is_uppercase = variants
                        .iter()
                        .all(|variant| variant.chars().all(|c| c.is_uppercase()));
                    format!(
                        r##"#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]{}
pub enum {name} {{
{}
}}
"##,
                        if is_uppercase {
                            "\n#[serde(rename_all = \"UPPERCASE\")]"
                        } else {
                            ""
                        },
                        variants
                            .iter()
                            .map(|variant| format!("    {},", variant.to_upper_camel_case()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum RefMode {
        Owned,
        Borrowed,
        Std,
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum Kind {
        Generic(Generic),
        Ref(Ref),
        DefaultValue {},
    }

    impl Kind {
        fn to_rust_type(&self, ref_mode: RefMode) -> Cow<str> {
            self.to_rust_type_ref(ref_mode)
        }

        fn to_rust_parameter_type(&self, required: bool) -> Cow<str> {
            let parameter_type = self.to_rust_type_ref(if required {
                RefMode::Borrowed
            } else {
                RefMode::Owned
            });
            if required {
                parameter_type
            } else {
                Cow::Owned(format!("Option<{parameter_type}>"))
            }
        }

        fn to_rust_type_ref(&self, ref_mode: RefMode) -> Cow<str> {
            match self {
                Kind::Generic(obj) => match obj {
                    Generic::Array {
                        items,
                        unique_items: _,
                    } => {
                        let item_type = if let Some(items) = items {
                            items.to_rust_type(ref_mode)
                        } else {
                            "TypeValue".into()
                        };
                        match ref_mode {
                            RefMode::Owned => format!("TypeVec<{item_type}>"),
                            RefMode::Borrowed => format!("&[{item_type}]"),
                            RefMode::Std => format!("Vec<{item_type}>"),
                        }
                        .into()
                    }
                    Generic::Boolean => "bool".into(),
                    Generic::Integer { format } => match format {
                        IntegerFormat::Int32 => "i32".into(),
                        IntegerFormat::Int64 => "i64".into(),
                    },
                    Generic::Object(obj) => obj.to_rust_type(ref_mode),
                    Generic::String => match ref_mode {
                        RefMode::Owned => "TypeString",
                        RefMode::Borrowed => "&str",
                        RefMode::Std => "String",
                    }
                    .into(),
                },
                Kind::Ref(Ref { reference }) => reference
                    .strip_prefix("#/components/schemas/")
                    .expect("#/components/schemas/ prefixed type name")
                    .into(),
                Kind::DefaultValue {} => "()".into(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Property {
        #[serde(default)]
        deprecated: bool,
        #[serde(default)]
        required: bool,
        #[serde(flatten)]
        kind: Kind,
    }

    impl Property {
        fn to_rust_type_opt(&self, ref_mode: RefMode) -> Cow<str> {
            let rust_type = self.to_rust_type(ref_mode);
            if self.required {
                rust_type
            } else {
                format!("Option<{rust_type}>").into()
            }
        }

        fn to_rust_type(&self, ref_mode: RefMode) -> Cow<str> {
            self.kind.to_rust_type(ref_mode)
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase", tag = "type")]
    pub enum Generic {
        Array {
            items: Option<Box<Property>>,
            #[serde(rename = "uniqueItems", default)]
            unique_items: bool,
        },
        Boolean,
        Integer {
            format: IntegerFormat,
        },
        Object(Box<ObjectSchema<Kind>>),
        String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Ref {
        #[serde(rename = "$ref")]
        pub reference: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum IntegerFormat {
        Int32,
        Int64,
    }
}

fn main() {
    let cli = Cli::parse();

    let specs: openapi::Spec = serde_json::from_slice(include_bytes!("../docs/openapi.json"))
        .expect("valid openapi json specs");

    match cli {
        Cli::Types => generate_types(&specs),
        Cli::Rest => generate_rest(&specs),
        Cli::Specs => {
            println!("{specs:#?}");
        }
    }
}

fn generate_rest(spec: &openapi::Spec) {
    print!(
        r###"use reqwest::header::CONTENT_LENGTH;
use serde_json::Value;

use super::*;

impl<TS: KeycloakTokenSupplier> KeycloakAdmin<TS> {{
"###
    );
    let mut path_counts = spec.paths.len();
    let tag_paths = spec
        .tags
        .iter()
        .map(|tag| {
            (
                tag.name.as_str(),
                spec.paths
                    .iter()
                    .filter(|(_, path_spec)| {
                        path_spec
                            .calls
                            .iter()
                            .all(|(_, call)| call.tags == vec![tag.name.as_str()])
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    for (tag, paths) in tag_paths {
        println!("    // <h4>{tag}</h4>\n");

        for (path, path_spec) in paths {
            println!("{}", path_spec.to_rust_method(path));
            path_counts -= 1;
        }
    }
    println!("}}");
    if path_counts > 0 {
        println!("// not all paths processed");
        println!("// left {path_counts}");
    }
}

fn generate_types(spec: &openapi::Spec) {
    println!(
        r###"use std::collections::HashMap;
#[cfg(any(feature = "rc-str", feature = "rc-vec"))]
use std::sync::Arc;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{{Deserialize, Serialize}};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[cfg(not(feature = "rc-map"))]
pub type TypeMap<K, V> = HashMap<K, V>;
#[cfg(not(feature = "rc-str"))]
pub type TypeString = String;
#[cfg(not(feature = "rc-val"))]
pub type TypeValue = Value;
#[cfg(not(feature = "rc-vec"))]
pub type TypeVec<I> = Vec<I>;

#[cfg(feature = "rc-map")]
pub type TypeMap<K, V> = Arc<HashMap<K, V>>;
#[cfg(feature = "rc-str")]
pub type TypeString = Arc<str>;
#[cfg(feature = "rc-val")]
pub type TypeValue = Arc<Value>;
#[cfg(feature = "rc-vec")]
pub type TypeVec<I> = Arc<[I]>;"###
    );

    for (schema_name, schema_obj) in &spec.components.schemas {
        print!(
            "\n{}",
            schema_obj.to_rust_type_definition(schema_name, openapi::RefMode::Owned)
        );
    }
}
