#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum AccessTokenCategory {
    Internal,
    Access,
    Id,
    Admin,
    UserInfo,
    Logout,
    AuthorizationResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum IDTokenCategory {
    Internal,
    Access,
    Id,
    Admin,
    UserInfo,
    Logout,
    AuthorizationResponse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum JsonNodeNodeType {
    Array,
    Binary,
    Boolean,
    Missing,
    Null,
    Number,
    Object,
    Pojo,
    String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum KeysMetadataRepresentationKeyMetadataRepresentationUse {
    Sig,
    Enc,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum PartialImportRepresentationPolicy {
    Skip,
    Overwrite,
    Fail,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyRepresentationDecisionStrategy {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyRepresentationLogic {
    Positive,
    Negative,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceServerRepresentationDecisionStrategy {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceServerRepresentationPolicyEnforcementMode {
    Enforcing,
    Permissive,
    Disabled,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AccessToken {
    pub acr: Option<String>,
    pub address: Option<AddressClaimSet>,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Option<Vec<String>>,
    pub at_hash: Option<String>,
    pub auth_time: Option<i64>,
    pub authorization: Option<AccessTokenAuthorization>,
    pub azp: Option<String>,
    pub birthdate: Option<String>,
    pub c_hash: Option<String>,
    pub category: Option<AccessTokenCategory>,
    pub claims_locales: Option<String>,
    pub cnf: Option<AccessTokenCertConf>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub exp: Option<i64>,
    pub family_name: Option<String>,
    pub gender: Option<String>,
    pub given_name: Option<String>,
    pub iat: Option<i64>,
    pub iss: Option<String>,
    pub jti: Option<String>,
    pub locale: Option<String>,
    pub middle_name: Option<String>,
    pub name: Option<String>,
    pub nbf: Option<i64>,
    pub nickname: Option<String>,
    pub nonce: Option<String>,
    #[serde(rename = "otherClaims")]
    pub other_claims: Option<HashMap<String, Value>>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub picture: Option<String>,
    pub preferred_username: Option<String>,
    pub profile: Option<String>,
    pub realm_access: Option<AccessTokenAccess>,
    pub s_hash: Option<String>,
    pub scope: Option<String>,
    pub session_state: Option<String>,
    pub sid: Option<String>,
    pub sub: Option<String>,
    #[serde(rename = "trusted-certs")]
    pub trusted_certs: Option<Vec<String>>,
    pub typ: Option<String>,
    pub updated_at: Option<i64>,
    pub website: Option<String>,
    pub zoneinfo: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AccessTokenAccess {
    pub roles: Option<Vec<String>>,
    pub verify_caller: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AccessTokenAuthorization {
    pub permissions: Option<Vec<Permission>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AccessTokenCertConf {
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AddressClaimSet {
    pub country: Option<String>,
    pub formatted: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street_address: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionExportRepresentation {
    pub authenticator: Option<String>,
    pub authenticator_config: Option<String>,
    pub authenticator_flow: Option<bool>,
    pub flow_alias: Option<String>,
    pub priority: Option<i32>,
    pub requirement: Option<String>,
    pub user_setup_allowed: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionInfoRepresentation {
    pub alias: Option<String>,
    pub authentication_config: Option<String>,
    pub authentication_flow: Option<bool>,
    pub configurable: Option<bool>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub flow_id: Option<String>,
    pub id: Option<String>,
    pub index: Option<i32>,
    pub level: Option<i32>,
    pub provider_id: Option<String>,
    pub requirement: Option<String>,
    pub requirement_choices: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionRepresentation {
    pub authenticator: Option<String>,
    pub authenticator_config: Option<String>,
    pub authenticator_flow: Option<bool>,
    pub flow_id: Option<String>,
    pub id: Option<String>,
    pub parent_flow: Option<String>,
    pub priority: Option<i32>,
    pub requirement: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationFlowRepresentation {
    pub alias: Option<String>,
    pub authentication_executions: Option<Vec<AuthenticationExecutionExportRepresentation>>,
    pub built_in: Option<bool>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub provider_id: Option<String>,
    pub top_level: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorConfigInfoRepresentation {
    pub help_text: Option<String>,
    pub name: Option<String>,
    pub properties: Option<Vec<ConfigPropertyRepresentation>>,
    pub provider_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AuthenticatorConfigRepresentation {
    pub alias: Option<String>,
    pub config: Option<HashMap<String, Value>>,
    pub id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CertificateRepresentation {
    pub certificate: Option<String>,
    pub kid: Option<String>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientInitialAccessCreatePresentation {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientInitialAccessPresentation {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
    pub id: Option<String>,
    pub remaining_count: Option<i32>,
    pub timestamp: Option<i32>,
    pub token: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientMappingsRepresentation {
    pub client: Option<String>,
    pub id: Option<String>,
    pub mappings: Option<Vec<RoleRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPoliciesRepresentation {
    pub policies: Option<Vec<ClientPolicyRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyConditionRepresentation {
    pub condition: Option<String>,
    pub configuration: Option<JsonNode>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyExecutorRepresentation {
    pub configuration: Option<JsonNode>,
    pub executor: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientPolicyRepresentation {
    pub conditions: Option<Vec<ClientPolicyConditionRepresentation>>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub profiles: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ClientProfileRepresentation {
    pub description: Option<String>,
    pub executors: Option<Vec<ClientPolicyExecutorRepresentation>>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientProfilesRepresentation {
    pub global_profiles: Option<Vec<ClientProfileRepresentation>>,
    pub profiles: Option<Vec<ClientProfileRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientRepresentation {
    pub access: Option<HashMap<String, Value>>,
    pub admin_url: Option<String>,
    pub always_display_in_console: Option<bool>,
    pub attributes: Option<HashMap<String, Value>>,
    pub authentication_flow_binding_overrides: Option<HashMap<String, Value>>,
    pub authorization_services_enabled: Option<bool>,
    pub authorization_settings: Option<ResourceServerRepresentation>,
    pub base_url: Option<String>,
    pub bearer_only: Option<bool>,
    pub client_authenticator_type: Option<String>,
    pub client_id: Option<String>,
    pub consent_required: Option<bool>,
    pub default_client_scopes: Option<Vec<String>>,
    pub description: Option<String>,
    pub direct_access_grants_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<String>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<String>,
    pub node_re_registration_timeout: Option<i32>,
    pub not_before: Option<i32>,
    pub oauth2_device_authorization_grant_enabled: Option<bool>,
    pub optional_client_scopes: Option<Vec<String>>,
    pub origin: Option<String>,
    pub protocol: Option<String>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation>>,
    pub public_client: Option<bool>,
    pub redirect_uris: Option<Vec<String>>,
    pub registered_nodes: Option<HashMap<String, Value>>,
    pub registration_access_token: Option<String>,
    pub root_url: Option<String>,
    pub secret: Option<String>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
    pub surrogate_auth_required: Option<bool>,
    pub web_origins: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeEvaluateResourceProtocolMapperEvaluationRepresentation {
    pub container_id: Option<String>,
    pub container_name: Option<String>,
    pub container_type: Option<String>,
    pub mapper_id: Option<String>,
    pub mapper_name: Option<String>,
    pub protocol_mapper: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRepresentation {
    pub attributes: Option<HashMap<String, Value>>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub protocol: Option<String>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentExportRepresentation {
    pub config: Option<HashMap<String, Vec<Value>>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub provider_id: Option<String>,
    pub sub_components: Option<HashMap<String, Vec<Value>>>,
    pub sub_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ComponentRepresentation {
    pub config: Option<HashMap<String, Vec<Value>>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub provider_id: Option<String>,
    pub provider_type: Option<String>,
    pub sub_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ConfigPropertyRepresentation {
    pub default_value: Option<Value>,
    pub help_text: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub options: Option<Vec<String>>,
    pub read_only: Option<bool>,
    pub secret: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CredentialRepresentation {
    pub created_date: Option<i64>,
    pub credential_data: Option<String>,
    pub id: Option<String>,
    pub priority: Option<i32>,
    pub secret_data: Option<String>,
    pub temporary: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub user_label: Option<String>,
    pub value: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CryptoInfoRepresentation {
    pub crypto_provider: Option<String>,
    pub supported_keystore_types: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FederatedIdentityRepresentation {
    pub identity_provider: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GlobalRequestResult {
    pub failed_requests: Option<Vec<String>>,
    pub success_requests: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation {
    pub access: Option<HashMap<String, Value>>,
    pub attributes: Option<HashMap<String, Value>>,
    pub client_roles: Option<HashMap<String, Value>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub realm_roles: Option<Vec<String>>,
    pub sub_groups: Option<Vec<GroupRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct IDToken {
    pub acr: Option<String>,
    pub address: Option<AddressClaimSet>,
    pub at_hash: Option<String>,
    pub auth_time: Option<i64>,
    pub azp: Option<String>,
    pub birthdate: Option<String>,
    pub c_hash: Option<String>,
    pub category: Option<IDTokenCategory>,
    pub claims_locales: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub exp: Option<i64>,
    pub family_name: Option<String>,
    pub gender: Option<String>,
    pub given_name: Option<String>,
    pub iat: Option<i64>,
    pub iss: Option<String>,
    pub jti: Option<String>,
    pub locale: Option<String>,
    pub middle_name: Option<String>,
    pub name: Option<String>,
    pub nbf: Option<i64>,
    pub nickname: Option<String>,
    pub nonce: Option<String>,
    #[serde(rename = "otherClaims")]
    pub other_claims: Option<HashMap<String, Value>>,
    pub phone_number: Option<String>,
    pub phone_number_verified: Option<bool>,
    pub picture: Option<String>,
    pub preferred_username: Option<String>,
    pub profile: Option<String>,
    pub s_hash: Option<String>,
    pub session_state: Option<String>,
    pub sid: Option<String>,
    pub sub: Option<String>,
    pub typ: Option<String>,
    pub updated_at: Option<i64>,
    pub website: Option<String>,
    pub zoneinfo: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperRepresentation {
    pub config: Option<HashMap<String, Value>>,
    pub id: Option<String>,
    pub identity_provider_alias: Option<String>,
    pub identity_provider_mapper: Option<String>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderRepresentation {
    pub add_read_token_role_on_create: Option<bool>,
    pub alias: Option<String>,
    pub config: Option<HashMap<String, Value>>,
    pub display_name: Option<String>,
    pub enabled: Option<bool>,
    pub first_broker_login_flow_alias: Option<String>,
    pub internal_id: Option<String>,
    pub link_only: Option<bool>,
    pub post_broker_login_flow_alias: Option<String>,
    pub provider_id: Option<String>,
    pub store_token: Option<bool>,
    pub trust_email: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct JsonNode {
    pub array: Option<bool>,
    pub big_decimal: Option<bool>,
    pub big_integer: Option<bool>,
    pub binary: Option<bool>,
    pub boolean: Option<bool>,
    pub container_node: Option<bool>,
    pub double: Option<bool>,
    pub empty: Option<bool>,
    pub float: Option<bool>,
    pub floating_point_number: Option<bool>,
    pub int: Option<bool>,
    pub integral_number: Option<bool>,
    pub long: Option<bool>,
    pub missing_node: Option<bool>,
    pub node_type: Option<JsonNodeNodeType>,
    pub null: Option<bool>,
    pub number: Option<bool>,
    pub object: Option<bool>,
    pub pojo: Option<bool>,
    pub short: Option<bool>,
    pub textual: Option<bool>,
    pub value_node: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct KeyStoreConfig {
    pub format: Option<String>,
    pub key_alias: Option<String>,
    pub key_password: Option<String>,
    pub realm_alias: Option<String>,
    pub realm_certificate: Option<bool>,
    pub store_password: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct KeysMetadataRepresentation {
    pub active: Option<HashMap<String, Value>>,
    pub keys: Option<Vec<KeysMetadataRepresentationKeyMetadataRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct KeysMetadataRepresentationKeyMetadataRepresentation {
    pub algorithm: Option<String>,
    pub certificate: Option<String>,
    pub kid: Option<String>,
    pub provider_id: Option<String>,
    pub provider_priority: Option<i64>,
    pub public_key: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "use")]
    pub use_: Option<KeysMetadataRepresentationKeyMetadataRepresentationUse>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ManagementPermissionReference {
    pub enabled: Option<bool>,
    pub resource: Option<String>,
    pub scope_permissions: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MappingsRepresentation {
    pub client_mappings: Option<HashMap<String, Value>>,
    pub realm_mappings: Option<Vec<RoleRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfoRepresentation {
    pub free: Option<i64>,
    pub free_formated: Option<String>,
    pub free_percentage: Option<i64>,
    pub total: Option<i64>,
    pub total_formated: Option<String>,
    pub used: Option<i64>,
    pub used_formated: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MultivaluedHashMap {
    pub empty: Option<bool>,
    pub load_factor: Option<f32>,
    pub threshold: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PartialImportRepresentation {
    pub clients: Option<Vec<ClientRepresentation>>,
    pub groups: Option<Vec<GroupRepresentation>>,
    pub identity_provider_mappers: Option<Vec<IdentityProviderMapperRepresentation>>,
    pub identity_providers: Option<Vec<IdentityProviderRepresentation>>,
    pub if_resource_exists: Option<String>,
    pub policy: Option<PartialImportRepresentationPolicy>,
    pub roles: Option<RolesRepresentation>,
    pub users: Option<Vec<UserRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PasswordPolicyTypeRepresentation {
    pub config_type: Option<String>,
    pub default_value: Option<String>,
    pub display_name: Option<String>,
    pub id: Option<String>,
    pub multiple_supported: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Permission {
    pub claims: Option<HashMap<String, Value>>,
    pub rsid: Option<String>,
    pub rsname: Option<String>,
    pub scopes: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PolicyRepresentation {
    pub config: Option<HashMap<String, Value>>,
    pub decision_strategy: Option<PolicyRepresentationDecisionStrategy>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub logic: Option<PolicyRepresentationLogic>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub policies: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
    pub resources_data: Option<Vec<ResourceRepresentation>>,
    pub scopes: Option<Vec<String>>,
    pub scopes_data: Option<Vec<ScopeRepresentation>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoRepresentation {
    pub disabled_features: Option<Vec<String>>,
    pub experimental_features: Option<Vec<String>>,
    pub name: Option<String>,
    pub preview_features: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperRepresentation {
    pub config: Option<HashMap<String, Value>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub protocol: Option<String>,
    pub protocol_mapper: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProviderRepresentation {
    pub operational_info: Option<HashMap<String, Value>>,
    pub order: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigRepresentation {
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub enabled_event_types: Option<Vec<String>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RealmRepresentation {
    pub access_code_lifespan: Option<i32>,
    pub access_code_lifespan_login: Option<i32>,
    pub access_code_lifespan_user_action: Option<i32>,
    pub access_token_lifespan: Option<i32>,
    pub access_token_lifespan_for_implicit_flow: Option<i32>,
    pub account_theme: Option<String>,
    pub action_token_generated_by_admin_lifespan: Option<i32>,
    pub action_token_generated_by_user_lifespan: Option<i32>,
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub admin_theme: Option<String>,
    pub attributes: Option<HashMap<String, Value>>,
    pub authentication_flows: Option<Vec<AuthenticationFlowRepresentation>>,
    pub authenticator_config: Option<Vec<AuthenticatorConfigRepresentation>>,
    pub browser_flow: Option<String>,
    pub browser_security_headers: Option<HashMap<String, Value>>,
    pub brute_force_protected: Option<bool>,
    pub client_authentication_flow: Option<String>,
    pub client_offline_session_idle_timeout: Option<i32>,
    pub client_offline_session_max_lifespan: Option<i32>,
    pub client_policies: Option<JsonNode>,
    pub client_profiles: Option<JsonNode>,
    pub client_scope_mappings: Option<HashMap<String, Value>>,
    pub client_scopes: Option<Vec<ClientScopeRepresentation>>,
    pub client_session_idle_timeout: Option<i32>,
    pub client_session_max_lifespan: Option<i32>,
    pub clients: Option<Vec<ClientRepresentation>>,
    pub components: Option<HashMap<String, Vec<Value>>>,
    pub default_default_client_scopes: Option<Vec<String>>,
    pub default_groups: Option<Vec<String>>,
    pub default_locale: Option<String>,
    pub default_optional_client_scopes: Option<Vec<String>>,
    pub default_role: Option<RoleRepresentation>,
    pub default_signature_algorithm: Option<String>,
    pub direct_grant_flow: Option<String>,
    pub display_name: Option<String>,
    pub display_name_html: Option<String>,
    pub docker_authentication_flow: Option<String>,
    pub duplicate_emails_allowed: Option<bool>,
    pub edit_username_allowed: Option<bool>,
    pub email_theme: Option<String>,
    pub enabled: Option<bool>,
    pub enabled_event_types: Option<Vec<String>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<Vec<String>>,
    pub failure_factor: Option<i32>,
    pub federated_users: Option<Vec<UserRepresentation>>,
    pub groups: Option<Vec<GroupRepresentation>>,
    pub id: Option<String>,
    pub identity_provider_mappers: Option<Vec<IdentityProviderMapperRepresentation>>,
    pub identity_providers: Option<Vec<IdentityProviderRepresentation>>,
    pub internationalization_enabled: Option<bool>,
    pub keycloak_version: Option<String>,
    pub login_theme: Option<String>,
    pub login_with_email_allowed: Option<bool>,
    pub max_delta_time_seconds: Option<i32>,
    pub max_failure_wait_seconds: Option<i32>,
    pub minimum_quick_login_wait_seconds: Option<i32>,
    pub not_before: Option<i32>,
    pub o_auth2_device_code_lifespan: Option<i32>,
    pub o_auth2_device_polling_interval: Option<i32>,
    pub oauth2_device_code_lifespan: Option<i32>,
    pub oauth2_device_polling_interval: Option<i32>,
    pub offline_session_idle_timeout: Option<i32>,
    pub offline_session_max_lifespan: Option<i32>,
    pub offline_session_max_lifespan_enabled: Option<bool>,
    pub otp_policy_algorithm: Option<String>,
    pub otp_policy_code_reusable: Option<bool>,
    pub otp_policy_digits: Option<i32>,
    pub otp_policy_initial_counter: Option<i32>,
    pub otp_policy_look_ahead_window: Option<i32>,
    pub otp_policy_period: Option<i32>,
    pub otp_policy_type: Option<String>,
    pub otp_supported_applications: Option<Vec<String>>,
    pub password_policy: Option<String>,
    pub permanent_lockout: Option<bool>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation>>,
    pub quick_login_check_milli_seconds: Option<i64>,
    pub realm: Option<String>,
    pub refresh_token_max_reuse: Option<i32>,
    pub registration_allowed: Option<bool>,
    pub registration_email_as_username: Option<bool>,
    pub registration_flow: Option<String>,
    pub remember_me: Option<bool>,
    pub required_actions: Option<Vec<RequiredActionProviderRepresentation>>,
    pub reset_credentials_flow: Option<String>,
    pub reset_password_allowed: Option<bool>,
    pub revoke_refresh_token: Option<bool>,
    pub roles: Option<RolesRepresentation>,
    pub scope_mappings: Option<Vec<ScopeMappingRepresentation>>,
    pub smtp_server: Option<HashMap<String, Value>>,
    pub ssl_required: Option<String>,
    pub sso_session_idle_timeout: Option<i32>,
    pub sso_session_idle_timeout_remember_me: Option<i32>,
    pub sso_session_max_lifespan: Option<i32>,
    pub sso_session_max_lifespan_remember_me: Option<i32>,
    pub supported_locales: Option<Vec<String>>,
    pub user_federation_mappers: Option<Vec<UserFederationMapperRepresentation>>,
    pub user_federation_providers: Option<Vec<UserFederationProviderRepresentation>>,
    pub user_managed_access_allowed: Option<bool>,
    pub users: Option<Vec<UserRepresentation>>,
    pub verify_email: Option<bool>,
    pub wait_increment_seconds: Option<i32>,
    pub web_authn_policy_acceptable_aaguids: Option<Vec<String>>,
    pub web_authn_policy_attestation_conveyance_preference: Option<String>,
    pub web_authn_policy_authenticator_attachment: Option<String>,
    pub web_authn_policy_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_create_timeout: Option<i32>,
    pub web_authn_policy_passwordless_acceptable_aaguids: Option<Vec<String>>,
    pub web_authn_policy_passwordless_attestation_conveyance_preference: Option<String>,
    pub web_authn_policy_passwordless_authenticator_attachment: Option<String>,
    pub web_authn_policy_passwordless_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_passwordless_create_timeout: Option<i32>,
    pub web_authn_policy_passwordless_require_resident_key: Option<String>,
    pub web_authn_policy_passwordless_rp_entity_name: Option<String>,
    pub web_authn_policy_passwordless_rp_id: Option<String>,
    pub web_authn_policy_passwordless_signature_algorithms: Option<Vec<String>>,
    pub web_authn_policy_passwordless_user_verification_requirement: Option<String>,
    pub web_authn_policy_require_resident_key: Option<String>,
    pub web_authn_policy_rp_entity_name: Option<String>,
    pub web_authn_policy_rp_id: Option<String>,
    pub web_authn_policy_signature_algorithms: Option<Vec<String>>,
    pub web_authn_policy_user_verification_requirement: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RequiredActionProviderRepresentation {
    pub alias: Option<String>,
    pub config: Option<HashMap<String, Value>>,
    pub default_action: Option<bool>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub priority: Option<i32>,
    pub provider_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ResourceRepresentation {
    pub id: Option<String>,
    pub attributes: Option<HashMap<String, Value>>,
    pub display_name: Option<String>,
    #[serde(rename = "icon_uri")]
    pub icon_uri: Option<String>,
    pub name: Option<String>,
    pub owner_managed_access: Option<bool>,
    pub scopes: Option<Vec<ScopeRepresentation>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub uris: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerRepresentation {
    pub allow_remote_resource_management: Option<bool>,
    pub client_id: Option<String>,
    pub decision_strategy: Option<ResourceServerRepresentationDecisionStrategy>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub policies: Option<Vec<PolicyRepresentation>>,
    pub policy_enforcement_mode: Option<ResourceServerRepresentationPolicyEnforcementMode>,
    pub resources: Option<Vec<ResourceRepresentation>>,
    pub scopes: Option<Vec<ScopeRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RoleRepresentation {
    pub attributes: Option<HashMap<String, Value>>,
    pub client_role: Option<bool>,
    pub composite: Option<bool>,
    pub composites: Option<RoleRepresentationComposites>,
    pub container_id: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RoleRepresentationComposites {
    pub client: Option<HashMap<String, Value>>,
    pub realm: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RolesRepresentation {
    pub client: Option<HashMap<String, Value>>,
    pub realm: Option<Vec<RoleRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScopeMappingRepresentation {
    pub client: Option<String>,
    pub client_scope: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScopeRepresentation {
    pub display_name: Option<String>,
    pub icon_uri: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub policies: Option<Vec<PolicyRepresentation>>,
    pub resources: Option<Vec<ResourceRepresentation>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoRepresentation {
    pub builtin_protocol_mappers: Option<HashMap<String, Value>>,
    pub client_importers: Option<Vec<HashMap<String, Value>>>,
    pub client_installations: Option<HashMap<String, Value>>,
    pub component_types: Option<HashMap<String, Value>>,
    pub crypto_info: Option<CryptoInfoRepresentation>,
    pub enums: Option<HashMap<String, Value>>,
    pub identity_providers: Option<Vec<HashMap<String, Value>>>,
    pub memory_info: Option<MemoryInfoRepresentation>,
    pub password_policies: Option<Vec<PasswordPolicyTypeRepresentation>>,
    pub profile_info: Option<ProfileInfoRepresentation>,
    pub protocol_mapper_types: Option<HashMap<String, Value>>,
    pub providers: Option<HashMap<String, Value>>,
    pub social_providers: Option<Vec<HashMap<String, Value>>>,
    pub system_info: Option<SystemInfoRepresentation>,
    pub themes: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SpiInfoRepresentation {
    pub internal: Option<bool>,
    pub providers: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoRepresentation {
    pub file_encoding: Option<String>,
    pub java_home: Option<String>,
    pub java_runtime: Option<String>,
    pub java_vendor: Option<String>,
    pub java_version: Option<String>,
    pub java_vm: Option<String>,
    pub java_vm_version: Option<String>,
    pub os_architecture: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub server_time: Option<String>,
    pub uptime: Option<String>,
    pub uptime_millis: Option<i64>,
    pub user_dir: Option<String>,
    pub user_locale: Option<String>,
    pub user_name: Option<String>,
    pub user_timezone: Option<String>,
    pub version: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserConsentRepresentation {
    pub client_id: Option<String>,
    pub created_date: Option<i64>,
    pub granted_client_scopes: Option<Vec<String>>,
    pub last_updated_date: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserFederationMapperRepresentation {
    pub config: Option<HashMap<String, Value>>,
    pub federation_mapper_type: Option<String>,
    pub federation_provider_display_name: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserFederationProviderRepresentation {
    pub changed_sync_period: Option<i32>,
    pub config: Option<HashMap<String, Value>>,
    pub display_name: Option<String>,
    pub full_sync_period: Option<i32>,
    pub id: Option<String>,
    pub last_sync: Option<i32>,
    pub priority: Option<i32>,
    pub provider_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UserRepresentation {
    pub access: Option<HashMap<String, Value>>,
    pub attributes: Option<HashMap<String, Value>>,
    pub client_consents: Option<Vec<UserConsentRepresentation>>,
    pub client_roles: Option<HashMap<String, Value>>,
    pub created_timestamp: Option<i64>,
    pub credentials: Option<Vec<CredentialRepresentation>>,
    pub disableable_credential_types: Option<Vec<String>>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub enabled: Option<bool>,
    pub federated_identities: Option<Vec<FederatedIdentityRepresentation>>,
    pub federation_link: Option<String>,
    pub first_name: Option<String>,
    pub groups: Option<Vec<String>>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub not_before: Option<i32>,
    pub origin: Option<String>,
    pub realm_roles: Option<Vec<String>>,
    pub required_actions: Option<Vec<String>>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub service_account_client_id: Option<String>,
    pub username: Option<String>,
}
