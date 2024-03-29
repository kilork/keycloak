use std::collections::HashMap;
#[cfg(any(feature = "rc-str", feature = "rc-vec"))]
use std::sync::Arc;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
pub type TypeVec<I> = Arc<[I]>;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Access {
    pub roles: Option<TypeVec<String>>,
    pub verify_caller: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AccessToken {
    pub acr: Option<TypeString>,
    pub address: Option<AddressClaimSet>,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Option<TypeVec<String>>,
    pub at_hash: Option<TypeString>,
    #[deprecated]
    #[serde(rename = "authTime")]
    pub auth_time_: Option<i32>,
    pub auth_time: Option<i64>,
    pub authorization: Option<Authorization>,
    pub azp: Option<TypeString>,
    pub birthdate: Option<TypeString>,
    pub c_hash: Option<TypeString>,
    pub claims_locales: Option<TypeString>,
    pub cnf: Option<Confirmation>,
    pub email: Option<TypeString>,
    pub email_verified: Option<bool>,
    pub exp: Option<i64>,
    pub family_name: Option<TypeString>,
    pub gender: Option<TypeString>,
    pub given_name: Option<TypeString>,
    pub iat: Option<i64>,
    pub iss: Option<TypeString>,
    pub jti: Option<TypeString>,
    pub locale: Option<TypeString>,
    pub middle_name: Option<TypeString>,
    pub name: Option<TypeString>,
    pub nbf: Option<i64>,
    pub nickname: Option<TypeString>,
    pub nonce: Option<TypeString>,
    #[serde(rename = "otherClaims")]
    pub other_claims: Option<TypeMap<String, Value>>,
    pub phone_number: Option<TypeString>,
    pub phone_number_verified: Option<bool>,
    pub picture: Option<TypeString>,
    pub preferred_username: Option<TypeString>,
    pub profile: Option<TypeString>,
    pub realm_access: Option<Access>,
    pub resource_access: Option<TypeMap<String, Access>>,
    pub s_hash: Option<TypeString>,
    pub scope: Option<TypeString>,
    pub session_state: Option<TypeString>,
    pub sid: Option<TypeString>,
    pub sub: Option<TypeString>,
    #[serde(rename = "trusted-certs")]
    pub trusted_certs: Option<TypeVec<String>>,
    pub typ: Option<TypeString>,
    pub updated_at: Option<i64>,
    pub website: Option<TypeString>,
    pub zoneinfo: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AddressClaimSet {
    pub country: Option<TypeString>,
    pub formatted: Option<TypeString>,
    pub locality: Option<TypeString>,
    pub postal_code: Option<TypeString>,
    pub region: Option<TypeString>,
    pub street_address: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AdminEventRepresentation {
    pub auth_details: Option<AuthDetailsRepresentation>,
    pub error: Option<TypeString>,
    pub operation_type: Option<TypeString>,
    pub realm_id: Option<TypeString>,
    pub representation: Option<TypeString>,
    pub resource_path: Option<TypeString>,
    pub resource_type: Option<TypeString>,
    pub time: Option<i64>,
}

#[deprecated]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ApplicationRepresentation {
    pub access: Option<TypeMap<String, bool>>,
    pub admin_url: Option<TypeString>,
    pub always_display_in_console: Option<bool>,
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub authentication_flow_binding_overrides: Option<TypeMap<String, TypeString>>,
    pub authorization_services_enabled: Option<bool>,
    pub authorization_settings: Option<ResourceServerRepresentation>,
    pub base_url: Option<TypeString>,
    pub bearer_only: Option<bool>,
    #[deprecated]
    pub claims: Option<ClaimRepresentation>,
    pub client_authenticator_type: Option<TypeString>,
    pub client_id: Option<TypeString>,
    #[deprecated]
    pub client_template: Option<TypeString>,
    pub consent_required: Option<bool>,
    pub default_client_scopes: Option<TypeVec<String>>,
    #[deprecated]
    pub default_roles: Option<TypeVec<String>>,
    pub description: Option<TypeString>,
    pub direct_access_grants_enabled: Option<bool>,
    #[deprecated]
    pub direct_grants_only: Option<bool>,
    pub enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<TypeString>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub node_re_registration_timeout: Option<i32>,
    pub not_before: Option<i32>,
    pub oauth2_device_authorization_grant_enabled: Option<bool>,
    pub optional_client_scopes: Option<TypeVec<String>>,
    pub origin: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
    pub public_client: Option<bool>,
    pub redirect_uris: Option<TypeVec<String>>,
    pub registered_nodes: Option<TypeMap<String, i32>>,
    pub registration_access_token: Option<TypeString>,
    pub root_url: Option<TypeString>,
    pub secret: Option<TypeString>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
    pub surrogate_auth_required: Option<bool>,
    #[deprecated]
    pub use_template_config: Option<bool>,
    #[deprecated]
    pub use_template_mappers: Option<bool>,
    #[deprecated]
    pub use_template_scope: Option<bool>,
    pub web_origins: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthDetailsRepresentation {
    pub client_id: Option<TypeString>,
    pub ip_address: Option<TypeString>,
    pub realm_id: Option<TypeString>,
    pub user_id: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionExportRepresentation {
    pub authenticator: Option<TypeString>,
    pub authenticator_config: Option<TypeString>,
    pub authenticator_flow: Option<bool>,
    #[deprecated]
    pub autheticator_flow: Option<bool>,
    pub flow_alias: Option<TypeString>,
    pub priority: Option<i32>,
    pub requirement: Option<TypeString>,
    pub user_setup_allowed: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionInfoRepresentation {
    pub alias: Option<TypeString>,
    pub authentication_config: Option<TypeString>,
    pub authentication_flow: Option<bool>,
    pub configurable: Option<bool>,
    pub description: Option<TypeString>,
    pub display_name: Option<TypeString>,
    pub flow_id: Option<TypeString>,
    pub id: Option<TypeString>,
    pub index: Option<i32>,
    pub level: Option<i32>,
    pub provider_id: Option<TypeString>,
    pub requirement: Option<TypeString>,
    pub requirement_choices: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionRepresentation {
    pub authenticator: Option<TypeString>,
    pub authenticator_config: Option<TypeString>,
    pub authenticator_flow: Option<bool>,
    #[deprecated]
    pub autheticator_flow: Option<bool>,
    pub flow_id: Option<TypeString>,
    pub id: Option<TypeString>,
    pub parent_flow: Option<TypeString>,
    pub priority: Option<i32>,
    pub requirement: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationFlowRepresentation {
    pub alias: Option<TypeString>,
    pub authentication_executions: Option<TypeVec<AuthenticationExecutionExportRepresentation>>,
    pub built_in: Option<bool>,
    pub description: Option<TypeString>,
    pub id: Option<TypeString>,
    pub provider_id: Option<TypeString>,
    pub top_level: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorConfigInfoRepresentation {
    pub help_text: Option<TypeString>,
    pub name: Option<TypeString>,
    pub properties: Option<TypeVec<ConfigPropertyRepresentation>>,
    pub provider_id: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AuthenticatorConfigRepresentation {
    pub alias: Option<TypeString>,
    pub config: Option<TypeMap<String, TypeString>>,
    pub id: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Authorization {
    pub permissions: Option<TypeVec<Permission>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CertificateRepresentation {
    pub certificate: Option<TypeString>,
    pub kid: Option<TypeString>,
    pub private_key: Option<TypeString>,
    pub public_key: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClaimRepresentation {
    pub address: Option<bool>,
    pub email: Option<bool>,
    pub gender: Option<bool>,
    pub locale: Option<bool>,
    pub name: Option<bool>,
    pub phone: Option<bool>,
    pub picture: Option<bool>,
    pub profile: Option<bool>,
    pub username: Option<bool>,
    pub website: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientInitialAccessCreatePresentation {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientInitialAccessPresentation {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
    pub id: Option<TypeString>,
    pub remaining_count: Option<i32>,
    pub timestamp: Option<i32>,
    pub token: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientMappingsRepresentation {
    pub client: Option<TypeString>,
    pub id: Option<TypeString>,
    pub mappings: Option<TypeVec<RoleRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPoliciesRepresentation {
    pub policies: Option<TypeVec<ClientPolicyRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyConditionRepresentation {
    pub condition: Option<TypeString>,
    pub configuration: Option<TypeVec<TypeValue>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyExecutorRepresentation {
    pub configuration: Option<TypeVec<TypeValue>>,
    pub executor: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyRepresentation {
    pub conditions: Option<TypeVec<ClientPolicyConditionRepresentation>>,
    pub description: Option<TypeString>,
    pub enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub profiles: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientProfileRepresentation {
    pub description: Option<TypeString>,
    pub executors: Option<TypeVec<ClientPolicyExecutorRepresentation>>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientProfilesRepresentation {
    pub global_profiles: Option<TypeVec<ClientProfileRepresentation>>,
    pub profiles: Option<TypeVec<ClientProfileRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientRepresentation {
    pub access: Option<TypeMap<String, bool>>,
    pub admin_url: Option<TypeString>,
    pub always_display_in_console: Option<bool>,
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub authentication_flow_binding_overrides: Option<TypeMap<String, TypeString>>,
    pub authorization_services_enabled: Option<bool>,
    pub authorization_settings: Option<ResourceServerRepresentation>,
    pub base_url: Option<TypeString>,
    pub bearer_only: Option<bool>,
    pub client_authenticator_type: Option<TypeString>,
    pub client_id: Option<TypeString>,
    #[deprecated]
    pub client_template: Option<TypeString>,
    pub consent_required: Option<bool>,
    pub default_client_scopes: Option<TypeVec<String>>,
    #[deprecated]
    pub default_roles: Option<TypeVec<String>>,
    pub description: Option<TypeString>,
    pub direct_access_grants_enabled: Option<bool>,
    #[deprecated]
    pub direct_grants_only: Option<bool>,
    pub enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<TypeString>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub node_re_registration_timeout: Option<i32>,
    pub not_before: Option<i32>,
    pub oauth2_device_authorization_grant_enabled: Option<bool>,
    pub optional_client_scopes: Option<TypeVec<String>>,
    pub origin: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
    pub public_client: Option<bool>,
    pub redirect_uris: Option<TypeVec<String>>,
    pub registered_nodes: Option<TypeMap<String, i32>>,
    pub registration_access_token: Option<TypeString>,
    pub root_url: Option<TypeString>,
    pub secret: Option<TypeString>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
    pub surrogate_auth_required: Option<bool>,
    #[deprecated]
    pub use_template_config: Option<bool>,
    #[deprecated]
    pub use_template_mappers: Option<bool>,
    #[deprecated]
    pub use_template_scope: Option<bool>,
    pub web_origins: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRepresentation {
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub description: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
}

#[deprecated]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientTemplateRepresentation {
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub bearer_only: Option<bool>,
    pub consent_required: Option<bool>,
    pub description: Option<TypeString>,
    pub direct_access_grants_enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<TypeString>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
    pub public_client: Option<bool>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentExportRepresentation {
    pub config: Option<MultivaluedHashMapStringString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub provider_id: Option<TypeString>,
    pub sub_components: Option<MultivaluedHashMapStringComponentExportRepresentation>,
    pub sub_type: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentRepresentation {
    pub config: Option<MultivaluedHashMapStringString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub parent_id: Option<TypeString>,
    pub provider_id: Option<TypeString>,
    pub provider_type: Option<TypeString>,
    pub sub_type: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentTypeRepresentation {
    pub help_text: Option<TypeString>,
    pub id: Option<TypeString>,
    pub metadata: Option<TypeMap<String, Value>>,
    pub properties: Option<TypeVec<ConfigPropertyRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Composites {
    #[deprecated]
    pub application: Option<TypeMap<String, TypeVec<String>>>,
    pub client: Option<TypeMap<String, TypeVec<String>>>,
    pub realm: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ConfigPropertyRepresentation {
    pub default_value: Option<Value>,
    pub help_text: Option<TypeString>,
    pub label: Option<TypeString>,
    pub name: Option<TypeString>,
    pub options: Option<TypeVec<String>>,
    pub read_only: Option<bool>,
    pub required: Option<bool>,
    pub secret: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Confirmation {
    pub jkt: Option<TypeString>,
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CredentialRepresentation {
    #[deprecated]
    pub algorithm: Option<TypeString>,
    #[deprecated]
    pub config: Option<MultivaluedHashMapStringString>,
    #[deprecated]
    pub counter: Option<i32>,
    pub created_date: Option<i64>,
    pub credential_data: Option<TypeString>,
    #[deprecated]
    pub device: Option<TypeString>,
    #[deprecated]
    pub digits: Option<i32>,
    #[deprecated]
    pub hash_iterations: Option<i32>,
    #[deprecated]
    pub hashed_salted_value: Option<TypeString>,
    pub id: Option<TypeString>,
    #[deprecated]
    pub period: Option<i32>,
    pub priority: Option<i32>,
    #[deprecated]
    pub salt: Option<TypeString>,
    pub secret_data: Option<TypeString>,
    pub temporary: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
    pub user_label: Option<TypeString>,
    pub value: Option<TypeString>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum DecisionStrategy {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum EnforcementMode {
    Permissive,
    Enforcing,
    Disabled,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct EventRepresentation {
    pub client_id: Option<TypeString>,
    pub details: Option<TypeMap<String, TypeString>>,
    pub error: Option<TypeString>,
    pub ip_address: Option<TypeString>,
    pub realm_id: Option<TypeString>,
    pub session_id: Option<TypeString>,
    pub time: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
    pub user_id: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FederatedIdentityRepresentation {
    pub identity_provider: Option<TypeString>,
    pub user_id: Option<TypeString>,
    pub user_name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GlobalRequestResult {
    pub failed_requests: Option<TypeVec<String>>,
    pub success_requests: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation {
    pub access: Option<TypeMap<String, bool>>,
    pub attributes: Option<TypeMap<String, TypeVec<String>>>,
    pub client_roles: Option<TypeMap<String, TypeVec<String>>>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub parent_id: Option<TypeString>,
    pub path: Option<TypeString>,
    pub realm_roles: Option<TypeVec<String>>,
    pub sub_group_count: Option<i64>,
    pub sub_groups: Option<TypeVec<GroupRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct IDToken {
    pub acr: Option<TypeString>,
    pub address: Option<AddressClaimSet>,
    pub at_hash: Option<TypeString>,
    #[deprecated]
    #[serde(rename = "authTime")]
    pub auth_time_: Option<i32>,
    pub auth_time: Option<i64>,
    pub azp: Option<TypeString>,
    pub birthdate: Option<TypeString>,
    pub c_hash: Option<TypeString>,
    pub claims_locales: Option<TypeString>,
    pub email: Option<TypeString>,
    pub email_verified: Option<bool>,
    pub exp: Option<i64>,
    pub family_name: Option<TypeString>,
    pub gender: Option<TypeString>,
    pub given_name: Option<TypeString>,
    pub iat: Option<i64>,
    pub iss: Option<TypeString>,
    pub jti: Option<TypeString>,
    pub locale: Option<TypeString>,
    pub middle_name: Option<TypeString>,
    pub name: Option<TypeString>,
    pub nbf: Option<i64>,
    pub nickname: Option<TypeString>,
    pub nonce: Option<TypeString>,
    #[serde(rename = "otherClaims")]
    pub other_claims: Option<TypeMap<String, Value>>,
    pub phone_number: Option<TypeString>,
    pub phone_number_verified: Option<bool>,
    pub picture: Option<TypeString>,
    pub preferred_username: Option<TypeString>,
    pub profile: Option<TypeString>,
    pub s_hash: Option<TypeString>,
    pub session_state: Option<TypeString>,
    pub sid: Option<TypeString>,
    pub sub: Option<TypeString>,
    pub typ: Option<TypeString>,
    pub updated_at: Option<i64>,
    pub website: Option<TypeString>,
    pub zoneinfo: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperRepresentation {
    pub config: Option<TypeMap<String, TypeString>>,
    pub id: Option<TypeString>,
    pub identity_provider_alias: Option<TypeString>,
    pub identity_provider_mapper: Option<TypeString>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperTypeRepresentation {
    pub category: Option<TypeString>,
    pub help_text: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub properties: Option<TypeVec<ConfigPropertyRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderRepresentation {
    pub add_read_token_role_on_create: Option<bool>,
    pub alias: Option<TypeString>,
    pub authenticate_by_default: Option<bool>,
    pub config: Option<TypeMap<String, TypeString>>,
    pub display_name: Option<TypeString>,
    pub enabled: Option<bool>,
    pub first_broker_login_flow_alias: Option<TypeString>,
    pub internal_id: Option<TypeString>,
    pub link_only: Option<bool>,
    pub post_broker_login_flow_alias: Option<TypeString>,
    pub provider_id: Option<TypeString>,
    pub store_token: Option<bool>,
    pub trust_email: Option<bool>,
    #[deprecated]
    pub update_profile_first_login: Option<bool>,
    #[deprecated]
    pub update_profile_first_login_mode: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct InstallationAdapterConfig {
    #[serde(rename = "auth-server-url")]
    pub auth_server_url: Option<TypeString>,
    #[serde(rename = "bearer-only")]
    pub bearer_only: Option<bool>,
    #[serde(rename = "confidential-port")]
    pub confidential_port: Option<i32>,
    pub credentials: Option<TypeMap<String, Value>>,
    #[serde(rename = "policy-enforcer")]
    pub policy_enforcer: Option<PolicyEnforcerConfig>,
    #[serde(rename = "public-client")]
    pub public_client: Option<bool>,
    pub realm: Option<TypeString>,
    #[serde(rename = "realm-public-key")]
    pub realm_public_key: Option<TypeString>,
    pub resource: Option<TypeString>,
    #[serde(rename = "ssl-required")]
    pub ssl_required: Option<TypeString>,
    #[serde(rename = "use-resource-role-mappings")]
    pub use_resource_role_mappings: Option<bool>,
    #[serde(rename = "verify-token-audience")]
    pub verify_token_audience: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct KeyMetadataRepresentation {
    pub algorithm: Option<TypeString>,
    pub certificate: Option<TypeString>,
    pub kid: Option<TypeString>,
    pub provider_id: Option<TypeString>,
    pub provider_priority: Option<i64>,
    pub public_key: Option<TypeString>,
    pub status: Option<TypeString>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
    #[serde(rename = "use")]
    pub use_: Option<KeyUse>,
    pub valid_to: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct KeyStoreConfig {
    pub format: Option<TypeString>,
    pub key_alias: Option<TypeString>,
    pub key_password: Option<TypeString>,
    pub realm_alias: Option<TypeString>,
    pub realm_certificate: Option<bool>,
    pub store_password: Option<TypeString>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum KeyUse {
    Sig,
    Enc,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct KeysMetadataRepresentation {
    pub active: Option<TypeMap<String, TypeString>>,
    pub keys: Option<TypeVec<KeyMetadataRepresentation>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum Logic {
    Positive,
    Negative,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ManagementPermissionReference {
    pub enabled: Option<bool>,
    pub resource: Option<TypeString>,
    pub scope_permissions: Option<TypeMap<String, TypeString>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MappingsRepresentation {
    pub client_mappings: Option<TypeMap<String, ClientMappingsRepresentation>>,
    pub realm_mappings: Option<TypeVec<RoleRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct MethodConfig {
    pub method: Option<TypeString>,
    pub scopes: Option<TypeVec<String>>,
    #[serde(rename = "scopes-enforcement-mode")]
    pub scopes_enforcement_mode: Option<ScopeEnforcementMode>,
}

pub type MultivaluedHashMapStringComponentExportRepresentation =
    TypeMap<String, TypeVec<ComponentExportRepresentation>>;

pub type MultivaluedHashMapStringString = TypeMap<String, TypeVec<String>>;

#[deprecated]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct OAuthClientRepresentation {
    pub access: Option<TypeMap<String, bool>>,
    pub admin_url: Option<TypeString>,
    pub always_display_in_console: Option<bool>,
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub authentication_flow_binding_overrides: Option<TypeMap<String, TypeString>>,
    pub authorization_services_enabled: Option<bool>,
    pub authorization_settings: Option<ResourceServerRepresentation>,
    pub base_url: Option<TypeString>,
    pub bearer_only: Option<bool>,
    #[deprecated]
    pub claims: Option<ClaimRepresentation>,
    pub client_authenticator_type: Option<TypeString>,
    pub client_id: Option<TypeString>,
    #[deprecated]
    pub client_template: Option<TypeString>,
    pub consent_required: Option<bool>,
    pub default_client_scopes: Option<TypeVec<String>>,
    #[deprecated]
    pub default_roles: Option<TypeVec<String>>,
    pub description: Option<TypeString>,
    pub direct_access_grants_enabled: Option<bool>,
    #[deprecated]
    pub direct_grants_only: Option<bool>,
    pub enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<TypeString>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub node_re_registration_timeout: Option<i32>,
    pub not_before: Option<i32>,
    pub oauth2_device_authorization_grant_enabled: Option<bool>,
    pub optional_client_scopes: Option<TypeVec<String>>,
    pub origin: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
    pub public_client: Option<bool>,
    pub redirect_uris: Option<TypeVec<String>>,
    pub registered_nodes: Option<TypeMap<String, i32>>,
    pub registration_access_token: Option<TypeString>,
    pub root_url: Option<TypeString>,
    pub secret: Option<TypeString>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
    pub surrogate_auth_required: Option<bool>,
    #[deprecated]
    pub use_template_config: Option<bool>,
    #[deprecated]
    pub use_template_mappers: Option<bool>,
    #[deprecated]
    pub use_template_scope: Option<bool>,
    pub web_origins: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PathCacheConfig {
    pub lifespan: Option<i64>,
    #[serde(rename = "max-entries")]
    pub max_entries: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PathConfig {
    #[serde(rename = "claim-information-point")]
    pub claim_information_point: Option<TypeMap<String, TypeMap<String, Value>>>,
    #[serde(rename = "enforcement-mode")]
    pub enforcement_mode: Option<EnforcementMode>,
    pub id: Option<TypeString>,
    pub invalidated: Option<bool>,
    pub methods: Option<TypeVec<MethodConfig>>,
    pub name: Option<TypeString>,
    pub path: Option<TypeString>,
    pub scopes: Option<TypeVec<String>>,
    #[serde(rename = "static")]
    pub static_: Option<bool>,
    pub static_path: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Permission {
    pub claims: Option<TypeMap<String, TypeVec<String>>>,
    pub rsid: Option<TypeString>,
    pub rsname: Option<TypeString>,
    pub scopes: Option<TypeVec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyEnforcementMode {
    Enforcing,
    Permissive,
    Disabled,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PolicyEnforcerConfig {
    #[serde(rename = "auth-server-url")]
    pub auth_server_url: Option<TypeString>,
    #[serde(rename = "claim-information-point")]
    pub claim_information_point: Option<TypeMap<String, TypeMap<String, Value>>>,
    pub credentials: Option<TypeMap<String, Value>>,
    #[serde(rename = "enforcement-mode")]
    pub enforcement_mode: Option<EnforcementMode>,
    #[serde(rename = "http-method-as-scope")]
    pub http_method_as_scope: Option<bool>,
    #[serde(rename = "lazy-load-paths")]
    pub lazy_load_paths: Option<bool>,
    #[serde(rename = "on-deny-redirect-to")]
    pub on_deny_redirect_to: Option<TypeString>,
    #[serde(rename = "path-cache")]
    pub path_cache: Option<PathCacheConfig>,
    pub paths: Option<TypeVec<PathConfig>>,
    pub realm: Option<TypeString>,
    pub resource: Option<TypeString>,
    #[serde(rename = "user-managed-access")]
    pub user_managed_access: Option<UserManagedAccessConfig>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PolicyRepresentation {
    pub config: Option<TypeMap<String, TypeString>>,
    pub decision_strategy: Option<DecisionStrategy>,
    pub description: Option<TypeString>,
    pub id: Option<TypeString>,
    pub logic: Option<Logic>,
    pub name: Option<TypeString>,
    pub owner: Option<TypeString>,
    pub policies: Option<TypeVec<String>>,
    pub resources: Option<TypeVec<String>>,
    pub resources_data: Option<TypeVec<ResourceRepresentation>>,
    pub scopes: Option<TypeVec<String>>,
    pub scopes_data: Option<TypeVec<ScopeRepresentation>>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperEvaluationRepresentation {
    pub container_id: Option<TypeString>,
    pub container_name: Option<TypeString>,
    pub container_type: Option<TypeString>,
    pub mapper_id: Option<TypeString>,
    pub mapper_name: Option<TypeString>,
    pub protocol_mapper: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperRepresentation {
    pub config: Option<TypeMap<String, TypeString>>,
    #[deprecated]
    pub consent_required: Option<bool>,
    #[deprecated]
    pub consent_text: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub protocol: Option<TypeString>,
    pub protocol_mapper: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PublishedRealmRepresentation {
    #[serde(rename = "account-service")]
    pub account_service: Option<TypeString>,
    pub public_key: Option<TypeString>,
    pub realm: Option<TypeString>,
    #[serde(rename = "token-service")]
    pub token_service: Option<TypeString>,
    #[serde(rename = "tokens-not-before")]
    pub tokens_not_before: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigRepresentation {
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub enabled_event_types: Option<TypeVec<String>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RealmRepresentation {
    pub access_code_lifespan: Option<i32>,
    pub access_code_lifespan_login: Option<i32>,
    pub access_code_lifespan_user_action: Option<i32>,
    pub access_token_lifespan: Option<i32>,
    pub access_token_lifespan_for_implicit_flow: Option<i32>,
    pub account_theme: Option<TypeString>,
    pub action_token_generated_by_admin_lifespan: Option<i32>,
    pub action_token_generated_by_user_lifespan: Option<i32>,
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub admin_theme: Option<TypeString>,
    #[deprecated]
    pub application_scope_mappings: Option<TypeMap<String, TypeVec<ScopeMappingRepresentation>>>,
    #[deprecated]
    pub applications: Option<TypeVec<ApplicationRepresentation>>,
    pub attributes: Option<TypeMap<String, TypeString>>,
    pub authentication_flows: Option<TypeVec<AuthenticationFlowRepresentation>>,
    pub authenticator_config: Option<TypeVec<AuthenticatorConfigRepresentation>>,
    pub browser_flow: Option<TypeString>,
    pub browser_security_headers: Option<TypeMap<String, TypeString>>,
    pub brute_force_protected: Option<bool>,
    #[deprecated]
    pub certificate: Option<TypeString>,
    pub client_authentication_flow: Option<TypeString>,
    pub client_offline_session_idle_timeout: Option<i32>,
    pub client_offline_session_max_lifespan: Option<i32>,
    pub client_policies: Option<ClientPoliciesRepresentation>,
    pub client_profiles: Option<ClientProfilesRepresentation>,
    pub client_scope_mappings: Option<TypeMap<String, TypeVec<ScopeMappingRepresentation>>>,
    pub client_scopes: Option<TypeVec<ClientScopeRepresentation>>,
    pub client_session_idle_timeout: Option<i32>,
    pub client_session_max_lifespan: Option<i32>,
    #[deprecated]
    pub client_templates: Option<TypeVec<ClientTemplateRepresentation>>,
    pub clients: Option<TypeVec<ClientRepresentation>>,
    #[deprecated]
    pub code_secret: Option<TypeString>,
    pub components: Option<MultivaluedHashMapStringComponentExportRepresentation>,
    pub default_default_client_scopes: Option<TypeVec<String>>,
    pub default_groups: Option<TypeVec<String>>,
    pub default_locale: Option<TypeString>,
    pub default_optional_client_scopes: Option<TypeVec<String>>,
    pub default_role: Option<RoleRepresentation>,
    #[deprecated]
    pub default_roles: Option<TypeVec<String>>,
    pub default_signature_algorithm: Option<TypeString>,
    pub direct_grant_flow: Option<TypeString>,
    pub display_name: Option<TypeString>,
    pub display_name_html: Option<TypeString>,
    pub docker_authentication_flow: Option<TypeString>,
    pub duplicate_emails_allowed: Option<bool>,
    pub edit_username_allowed: Option<bool>,
    pub email_theme: Option<TypeString>,
    pub enabled: Option<bool>,
    pub enabled_event_types: Option<TypeVec<String>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<TypeVec<String>>,
    pub failure_factor: Option<i32>,
    pub federated_users: Option<TypeVec<UserRepresentation>>,
    pub groups: Option<TypeVec<GroupRepresentation>>,
    pub id: Option<TypeString>,
    pub identity_provider_mappers: Option<TypeVec<IdentityProviderMapperRepresentation>>,
    pub identity_providers: Option<TypeVec<IdentityProviderRepresentation>>,
    pub internationalization_enabled: Option<bool>,
    pub keycloak_version: Option<TypeString>,
    pub localization_texts: Option<TypeMap<String, TypeMap<String, TypeString>>>,
    pub login_theme: Option<TypeString>,
    pub login_with_email_allowed: Option<bool>,
    pub max_delta_time_seconds: Option<i32>,
    pub max_failure_wait_seconds: Option<i32>,
    pub minimum_quick_login_wait_seconds: Option<i32>,
    pub not_before: Option<i32>,
    pub o_auth2_device_code_lifespan: Option<i32>,
    pub o_auth2_device_polling_interval: Option<i32>,
    pub oauth2_device_code_lifespan: Option<i32>,
    pub oauth2_device_polling_interval: Option<i32>,
    #[deprecated]
    pub oauth_clients: Option<TypeVec<OAuthClientRepresentation>>,
    pub offline_session_idle_timeout: Option<i32>,
    pub offline_session_max_lifespan: Option<i32>,
    pub offline_session_max_lifespan_enabled: Option<bool>,
    pub otp_policy_algorithm: Option<TypeString>,
    pub otp_policy_code_reusable: Option<bool>,
    pub otp_policy_digits: Option<i32>,
    pub otp_policy_initial_counter: Option<i32>,
    pub otp_policy_look_ahead_window: Option<i32>,
    pub otp_policy_period: Option<i32>,
    pub otp_policy_type: Option<TypeString>,
    pub otp_supported_applications: Option<TypeVec<String>>,
    #[deprecated]
    pub password_credential_grant_allowed: Option<bool>,
    pub password_policy: Option<TypeString>,
    pub permanent_lockout: Option<bool>,
    #[deprecated]
    pub private_key: Option<TypeString>,
    pub protocol_mappers: Option<TypeVec<ProtocolMapperRepresentation>>,
    #[deprecated]
    pub public_key: Option<TypeString>,
    pub quick_login_check_milli_seconds: Option<i64>,
    pub realm: Option<TypeString>,
    #[deprecated]
    pub realm_cache_enabled: Option<bool>,
    pub refresh_token_max_reuse: Option<i32>,
    pub registration_allowed: Option<bool>,
    pub registration_email_as_username: Option<bool>,
    pub registration_flow: Option<TypeString>,
    pub remember_me: Option<bool>,
    pub required_actions: Option<TypeVec<RequiredActionProviderRepresentation>>,
    #[deprecated]
    pub required_credentials: Option<TypeVec<String>>,
    pub reset_credentials_flow: Option<TypeString>,
    pub reset_password_allowed: Option<bool>,
    pub revoke_refresh_token: Option<bool>,
    pub roles: Option<RolesRepresentation>,
    pub scope_mappings: Option<TypeVec<ScopeMappingRepresentation>>,
    pub smtp_server: Option<TypeMap<String, TypeString>>,
    #[deprecated]
    pub social: Option<bool>,
    #[deprecated]
    pub social_providers: Option<TypeMap<String, TypeString>>,
    pub ssl_required: Option<TypeString>,
    pub sso_session_idle_timeout: Option<i32>,
    pub sso_session_idle_timeout_remember_me: Option<i32>,
    pub sso_session_max_lifespan: Option<i32>,
    pub sso_session_max_lifespan_remember_me: Option<i32>,
    pub supported_locales: Option<TypeVec<String>>,
    #[deprecated]
    pub update_profile_on_initial_social_login: Option<bool>,
    #[deprecated]
    pub user_cache_enabled: Option<bool>,
    pub user_federation_mappers: Option<TypeVec<UserFederationMapperRepresentation>>,
    pub user_federation_providers: Option<TypeVec<UserFederationProviderRepresentation>>,
    pub user_managed_access_allowed: Option<bool>,
    pub users: Option<TypeVec<UserRepresentation>>,
    pub verify_email: Option<bool>,
    pub wait_increment_seconds: Option<i32>,
    pub web_authn_policy_acceptable_aaguids: Option<TypeVec<String>>,
    pub web_authn_policy_attestation_conveyance_preference: Option<TypeString>,
    pub web_authn_policy_authenticator_attachment: Option<TypeString>,
    pub web_authn_policy_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_create_timeout: Option<i32>,
    pub web_authn_policy_extra_origins: Option<TypeVec<String>>,
    pub web_authn_policy_passwordless_acceptable_aaguids: Option<TypeVec<String>>,
    pub web_authn_policy_passwordless_attestation_conveyance_preference: Option<TypeString>,
    pub web_authn_policy_passwordless_authenticator_attachment: Option<TypeString>,
    pub web_authn_policy_passwordless_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_passwordless_create_timeout: Option<i32>,
    pub web_authn_policy_passwordless_extra_origins: Option<TypeVec<String>>,
    pub web_authn_policy_passwordless_require_resident_key: Option<TypeString>,
    pub web_authn_policy_passwordless_rp_entity_name: Option<TypeString>,
    pub web_authn_policy_passwordless_rp_id: Option<TypeString>,
    pub web_authn_policy_passwordless_signature_algorithms: Option<TypeVec<String>>,
    pub web_authn_policy_passwordless_user_verification_requirement: Option<TypeString>,
    pub web_authn_policy_require_resident_key: Option<TypeString>,
    pub web_authn_policy_rp_entity_name: Option<TypeString>,
    pub web_authn_policy_rp_id: Option<TypeString>,
    pub web_authn_policy_signature_algorithms: Option<TypeVec<String>>,
    pub web_authn_policy_user_verification_requirement: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RequiredActionProviderRepresentation {
    pub alias: Option<TypeString>,
    pub config: Option<TypeMap<String, TypeString>>,
    pub default_action: Option<bool>,
    pub enabled: Option<bool>,
    pub name: Option<TypeString>,
    pub priority: Option<i32>,
    pub provider_id: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ResourceOwnerRepresentation {
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ResourceRepresentation {
    #[serde(rename = "_id")]
    pub id: Option<TypeString>,
    pub attributes: Option<TypeMap<String, TypeVec<String>>>,
    pub display_name: Option<TypeString>,
    #[serde(rename = "icon_uri")]
    pub icon_uri: Option<TypeString>,
    pub name: Option<TypeString>,
    pub owner: Option<ResourceOwnerRepresentation>,
    pub owner_managed_access: Option<bool>,
    pub scopes: Option<TypeVec<ScopeRepresentation>>,
    pub scopes_uma: Option<TypeVec<ScopeRepresentation>>,
    #[serde(rename = "type")]
    pub type_: Option<TypeString>,
    #[deprecated]
    pub uri: Option<TypeString>,
    pub uris: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerRepresentation {
    pub allow_remote_resource_management: Option<bool>,
    pub client_id: Option<TypeString>,
    pub decision_strategy: Option<DecisionStrategy>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub policies: Option<TypeVec<PolicyRepresentation>>,
    pub policy_enforcement_mode: Option<PolicyEnforcementMode>,
    pub resources: Option<TypeVec<ResourceRepresentation>>,
    pub scopes: Option<TypeVec<ScopeRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RoleRepresentation {
    pub attributes: Option<TypeMap<String, TypeVec<String>>>,
    pub client_role: Option<bool>,
    pub composite: Option<bool>,
    pub composites: Option<Composites>,
    pub container_id: Option<TypeString>,
    pub description: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    #[deprecated]
    pub scope_param_required: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RolesRepresentation {
    #[deprecated]
    pub application: Option<TypeMap<String, TypeVec<RoleRepresentation>>>,
    pub client: Option<TypeMap<String, TypeVec<RoleRepresentation>>>,
    pub realm: Option<TypeVec<RoleRepresentation>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum ScopeEnforcementMode {
    All,
    Any,
    Disabled,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScopeMappingRepresentation {
    pub client: Option<TypeString>,
    pub client_scope: Option<TypeString>,
    #[deprecated]
    pub client_template: Option<TypeString>,
    pub roles: Option<TypeVec<String>>,
    #[serde(rename = "self")]
    pub self_: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScopeRepresentation {
    pub display_name: Option<TypeString>,
    pub icon_uri: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
    pub policies: Option<TypeVec<PolicyRepresentation>>,
    pub resources: Option<TypeVec<ResourceRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SocialLinkRepresentation {
    pub social_provider: Option<TypeString>,
    pub social_user_id: Option<TypeString>,
    pub social_username: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UPAttribute {
    pub annotations: Option<TypeMap<String, Value>>,
    pub display_name: Option<TypeString>,
    pub group: Option<TypeString>,
    pub name: Option<TypeString>,
    pub permissions: Option<UPAttributePermissions>,
    pub required: Option<UPAttributeRequired>,
    pub selector: Option<UPAttributeSelector>,
    pub validations: Option<TypeMap<String, TypeMap<String, Value>>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct UPAttributePermissions {
    pub edit: Option<TypeVec<String>>,
    pub view: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct UPAttributeRequired {
    pub roles: Option<TypeVec<String>>,
    pub scopes: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct UPAttributeSelector {
    pub scopes: Option<TypeVec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct UPConfig {
    pub attributes: Option<TypeVec<UPAttribute>>,
    pub groups: Option<TypeVec<UPGroup>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UPGroup {
    pub annotations: Option<TypeMap<String, Value>>,
    pub display_description: Option<TypeString>,
    pub display_header: Option<TypeString>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserConsentRepresentation {
    pub client_id: Option<TypeString>,
    pub created_date: Option<i64>,
    pub granted_client_scopes: Option<TypeVec<String>>,
    #[deprecated]
    pub granted_realm_roles: Option<TypeVec<String>>,
    pub last_updated_date: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserFederationMapperRepresentation {
    pub config: Option<TypeMap<String, TypeString>>,
    pub federation_mapper_type: Option<TypeString>,
    pub federation_provider_display_name: Option<TypeString>,
    pub id: Option<TypeString>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserFederationProviderRepresentation {
    pub changed_sync_period: Option<i32>,
    pub config: Option<TypeMap<String, TypeString>>,
    pub display_name: Option<TypeString>,
    pub full_sync_period: Option<i32>,
    pub id: Option<TypeString>,
    pub last_sync: Option<i32>,
    pub priority: Option<i32>,
    pub provider_name: Option<TypeString>,
}

pub type UserManagedAccessConfig = TypeMap<String, TypeValue>;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserProfileAttributeGroupMetadata {
    pub annotations: Option<TypeMap<String, Value>>,
    pub display_description: Option<TypeString>,
    pub display_header: Option<TypeString>,
    pub name: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserProfileAttributeMetadata {
    pub annotations: Option<TypeMap<String, Value>>,
    pub display_name: Option<TypeString>,
    pub group: Option<TypeString>,
    pub name: Option<TypeString>,
    pub read_only: Option<bool>,
    pub required: Option<bool>,
    pub validators: Option<TypeMap<String, TypeMap<String, Value>>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct UserProfileMetadata {
    pub attributes: Option<TypeVec<UserProfileAttributeMetadata>>,
    pub groups: Option<TypeVec<UserProfileAttributeGroupMetadata>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserRepresentation {
    pub access: Option<TypeMap<String, bool>>,
    #[deprecated]
    pub application_roles: Option<TypeMap<String, TypeVec<String>>>,
    pub attributes: Option<TypeMap<String, TypeVec<String>>>,
    pub client_consents: Option<TypeVec<UserConsentRepresentation>>,
    pub client_roles: Option<TypeMap<String, TypeVec<String>>>,
    pub created_timestamp: Option<i64>,
    pub credentials: Option<TypeVec<CredentialRepresentation>>,
    pub disableable_credential_types: Option<TypeVec<String>>,
    pub email: Option<TypeString>,
    pub email_verified: Option<bool>,
    pub enabled: Option<bool>,
    pub federated_identities: Option<TypeVec<FederatedIdentityRepresentation>>,
    pub federation_link: Option<TypeString>,
    pub first_name: Option<TypeString>,
    pub groups: Option<TypeVec<String>>,
    pub id: Option<TypeString>,
    pub last_name: Option<TypeString>,
    pub not_before: Option<i32>,
    pub origin: Option<TypeString>,
    pub realm_roles: Option<TypeVec<String>>,
    pub required_actions: Option<TypeVec<String>>,
    #[serde(rename = "self")]
    pub self_: Option<TypeString>,
    pub service_account_client_id: Option<TypeString>,
    #[deprecated]
    pub social_links: Option<TypeVec<SocialLinkRepresentation>>,
    pub totp: Option<bool>,
    pub user_profile_metadata: Option<UserProfileMetadata>,
    pub username: Option<TypeString>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserSessionRepresentation {
    pub clients: Option<TypeMap<String, TypeString>>,
    pub id: Option<TypeString>,
    pub ip_address: Option<TypeString>,
    pub last_access: Option<i64>,
    pub remember_me: Option<bool>,
    pub start: Option<i64>,
    pub user_id: Option<TypeString>,
    pub username: Option<TypeString>,
}
