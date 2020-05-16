use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccessTokenCategory {
    Internal,
    Access,
    Id,
    Admin,
    UserInfo,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PartialImportRepresentationPolicy {
    Skip,
    Overwrite,
    Fail,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyRepresentationDecisionStrategy {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyRepresentationLogic {
    Positive,
    Negative,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceServerRepresentationDecisionStrategy {
    Affirmative,
    Unanimous,
    Consensus,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceServerRepresentationPolicyEnforcementMode {
    Enforcing,
    Permissive,
    Disabled,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken<'a> {
    pub acr: Option<Cow<'a, str>>,
    pub address: Option<AddressClaimSet<'a>>,
    #[serde(rename = "allowed-origins")]
    pub allowed_origins: Option<Vec<Cow<'a, str>>>,
    pub at_hash: Option<Cow<'a, str>>,
    pub auth_time: Option<i64>,
    pub authorization: Option<AccessTokenAuthorization<'a>>,
    pub azp: Option<Cow<'a, str>>,
    pub birthdate: Option<Cow<'a, str>>,
    pub c_hash: Option<Cow<'a, str>>,
    pub category: Option<AccessTokenCategory>,
    pub claims_locales: Option<Cow<'a, str>>,
    pub cnf: Option<AccessTokenCertConf<'a>>,
    pub email: Option<Cow<'a, str>>,
    pub email_verified: Option<bool>,
    pub exp: Option<i64>,
    pub family_name: Option<Cow<'a, str>>,
    pub gender: Option<Cow<'a, str>>,
    pub given_name: Option<Cow<'a, str>>,
    pub iat: Option<i64>,
    pub iss: Option<Cow<'a, str>>,
    pub jti: Option<Cow<'a, str>>,
    pub locale: Option<Cow<'a, str>>,
    pub middle_name: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub nbf: Option<i64>,
    pub nickname: Option<Cow<'a, str>>,
    pub nonce: Option<Cow<'a, str>>,
    pub other_claims: Option<HashMap<Cow<'a, str>, Value>>,
    pub phone_number: Option<Cow<'a, str>>,
    pub phone_number_verified: Option<bool>,
    pub picture: Option<Cow<'a, str>>,
    pub preferred_username: Option<Cow<'a, str>>,
    pub profile: Option<Cow<'a, str>>,
    pub realm_access: Option<AccessTokenAccess<'a>>,
    pub s_hash: Option<Cow<'a, str>>,
    pub scope: Option<Cow<'a, str>>,
    pub session_state: Option<Cow<'a, str>>,
    pub sub: Option<Cow<'a, str>>,
    #[serde(rename = "trusted-certs")]
    pub trusted_certs: Option<Vec<Cow<'a, str>>>,
    pub typ: Option<Cow<'a, str>>,
    pub updated_at: Option<i64>,
    pub website: Option<Cow<'a, str>>,
    pub zoneinfo: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessTokenAccess<'a> {
    pub roles: Option<Vec<Cow<'a, str>>>,
    pub verify_caller: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessTokenAuthorization<'a> {
    pub permissions: Option<Vec<Permission<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessTokenCertConf<'a> {
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AddressClaimSet<'a> {
    pub country: Option<Cow<'a, str>>,
    pub formatted: Option<Cow<'a, str>>,
    pub locality: Option<Cow<'a, str>>,
    pub postal_code: Option<Cow<'a, str>>,
    pub region: Option<Cow<'a, str>>,
    pub street_address: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminEventRepresentation<'a> {
    pub auth_details: Option<AuthDetailsRepresentation<'a>>,
    pub error: Option<Cow<'a, str>>,
    pub operation_type: Option<Cow<'a, str>>,
    pub realm_id: Option<Cow<'a, str>>,
    pub representation: Option<Cow<'a, str>>,
    pub resource_path: Option<Cow<'a, str>>,
    pub resource_type: Option<Cow<'a, str>>,
    pub time: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthDetailsRepresentation<'a> {
    pub client_id: Option<Cow<'a, str>>,
    pub ip_address: Option<Cow<'a, str>>,
    pub realm_id: Option<Cow<'a, str>>,
    pub user_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionExportRepresentation<'a> {
    pub authenticator: Option<Cow<'a, str>>,
    pub authenticator_config: Option<Cow<'a, str>>,
    pub authenticator_flow: Option<bool>,
    pub autheticator_flow: Option<bool>,
    pub flow_alias: Option<Cow<'a, str>>,
    pub priority: Option<i32>,
    pub requirement: Option<Cow<'a, str>>,
    pub user_setup_allowed: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionInfoRepresentation<'a> {
    pub alias: Option<Cow<'a, str>>,
    pub authentication_config: Option<Cow<'a, str>>,
    pub authentication_flow: Option<bool>,
    pub configurable: Option<bool>,
    pub display_name: Option<Cow<'a, str>>,
    pub flow_id: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub index: Option<i32>,
    pub level: Option<i32>,
    pub provider_id: Option<Cow<'a, str>>,
    pub requirement: Option<Cow<'a, str>>,
    pub requirement_choices: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionRepresentation<'a> {
    pub authenticator: Option<Cow<'a, str>>,
    pub authenticator_config: Option<Cow<'a, str>>,
    pub authenticator_flow: Option<bool>,
    pub autheticator_flow: Option<bool>,
    pub flow_id: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub parent_flow: Option<Cow<'a, str>>,
    pub priority: Option<i32>,
    pub requirement: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationFlowRepresentation<'a> {
    pub alias: Option<Cow<'a, str>>,
    pub authentication_executions: Option<Vec<AuthenticationExecutionExportRepresentation<'a>>>,
    pub built_in: Option<bool>,
    pub description: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub provider_id: Option<Cow<'a, str>>,
    pub top_level: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorConfigInfoRepresentation<'a> {
    pub help_text: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub properties: Option<Vec<ConfigPropertyRepresentation<'a>>>,
    pub provider_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthenticatorConfigRepresentation<'a> {
    pub alias: Option<Cow<'a, str>>,
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateRepresentation<'a> {
    pub certificate: Option<Cow<'a, str>>,
    pub kid: Option<Cow<'a, str>>,
    pub private_key: Option<Cow<'a, str>>,
    pub public_key: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientInitialAccessCreatePresentation {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInitialAccessPresentation<'a> {
    pub count: Option<i32>,
    pub expiration: Option<i32>,
    pub id: Option<Cow<'a, str>>,
    pub remaining_count: Option<i32>,
    pub timestamp: Option<i32>,
    pub token: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ClientMappingsRepresentation<'a> {
    pub client: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub mappings: Option<Vec<RoleRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRepresentation<'a> {
    pub access: Option<HashMap<Cow<'a, str>, Value>>,
    pub admin_url: Option<Cow<'a, str>>,
    pub always_display_in_console: Option<bool>,
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub authentication_flow_binding_overrides: Option<HashMap<Cow<'a, str>, Value>>,
    pub authorization_services_enabled: Option<bool>,
    pub authorization_settings: Option<ResourceServerRepresentation<'a>>,
    pub base_url: Option<Cow<'a, str>>,
    pub bearer_only: Option<bool>,
    pub client_authenticator_type: Option<Cow<'a, str>>,
    pub client_id: Option<Cow<'a, str>>,
    pub consent_required: Option<bool>,
    pub default_client_scopes: Option<Vec<Cow<'a, str>>>,
    pub default_roles: Option<Vec<Cow<'a, str>>>,
    pub description: Option<Cow<'a, str>>,
    pub direct_access_grants_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub frontchannel_logout: Option<bool>,
    pub full_scope_allowed: Option<bool>,
    pub id: Option<Cow<'a, str>>,
    pub implicit_flow_enabled: Option<bool>,
    pub name: Option<Cow<'a, str>>,
    pub node_re_registration_timeout: Option<i32>,
    pub not_before: Option<i32>,
    pub optional_client_scopes: Option<Vec<Cow<'a, str>>>,
    pub origin: Option<Cow<'a, str>>,
    pub protocol: Option<Cow<'a, str>>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation<'a>>>,
    pub public_client: Option<bool>,
    pub redirect_uris: Option<Vec<Cow<'a, str>>>,
    pub registered_nodes: Option<HashMap<Cow<'a, str>, Value>>,
    pub registration_access_token: Option<Cow<'a, str>>,
    pub root_url: Option<Cow<'a, str>>,
    pub secret: Option<Cow<'a, str>>,
    pub service_accounts_enabled: Option<bool>,
    pub standard_flow_enabled: Option<bool>,
    pub surrogate_auth_required: Option<bool>,
    pub web_origins: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeEvaluateResourceProtocolMapperEvaluationRepresentation<'a> {
    pub container_id: Option<Cow<'a, str>>,
    pub container_name: Option<Cow<'a, str>>,
    pub container_type: Option<Cow<'a, str>>,
    pub mapper_id: Option<Cow<'a, str>>,
    pub mapper_name: Option<Cow<'a, str>>,
    pub protocol_mapper: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRepresentation<'a> {
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub description: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub protocol: Option<Cow<'a, str>>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentExportRepresentation<'a> {
    pub config: Option<MultivaluedHashMap>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub provider_id: Option<Cow<'a, str>>,
    pub sub_components: Option<MultivaluedHashMap>,
    pub sub_type: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRepresentation<'a> {
    pub config: Option<MultivaluedHashMap>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub parent_id: Option<Cow<'a, str>>,
    pub provider_id: Option<Cow<'a, str>>,
    pub provider_type: Option<Cow<'a, str>>,
    pub sub_type: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentTypeRepresentation<'a> {
    pub help_text: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub metadata: Option<HashMap<Cow<'a, str>, Value>>,
    pub properties: Option<Vec<ConfigPropertyRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigPropertyRepresentation<'a> {
    pub default_value: Option<Value>,
    pub help_text: Option<Cow<'a, str>>,
    pub label: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub options: Option<Vec<Cow<'a, str>>>,
    pub secret: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialRepresentation<'a> {
    pub created_date: Option<i64>,
    pub credential_data: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub priority: Option<i32>,
    pub secret_data: Option<Cow<'a, str>>,
    pub temporary: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
    pub user_label: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRepresentation<'a> {
    pub client_id: Option<Cow<'a, str>>,
    pub details: Option<HashMap<Cow<'a, str>, Value>>,
    pub error: Option<Cow<'a, str>>,
    pub ip_address: Option<Cow<'a, str>>,
    pub realm_id: Option<Cow<'a, str>>,
    pub session_id: Option<Cow<'a, str>>,
    pub time: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
    pub user_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FederatedIdentityRepresentation<'a> {
    pub identity_provider: Option<Cow<'a, str>>,
    pub user_id: Option<Cow<'a, str>>,
    pub user_name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalRequestResult<'a> {
    pub failed_requests: Option<Vec<Cow<'a, str>>>,
    pub success_requests: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation<'a> {
    pub access: Option<HashMap<Cow<'a, str>, Value>>,
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub client_roles: Option<HashMap<Cow<'a, str>, Value>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub path: Option<Cow<'a, str>>,
    pub realm_roles: Option<Vec<Cow<'a, str>>>,
    pub sub_groups: Option<Vec<GroupRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperRepresentation<'a> {
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub id: Option<Cow<'a, str>>,
    pub identity_provider_alias: Option<Cow<'a, str>>,
    pub identity_provider_mapper: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderRepresentation<'a> {
    pub add_read_token_role_on_create: Option<bool>,
    pub alias: Option<Cow<'a, str>>,
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub display_name: Option<Cow<'a, str>>,
    pub enabled: Option<bool>,
    pub first_broker_login_flow_alias: Option<Cow<'a, str>>,
    pub internal_id: Option<Cow<'a, str>>,
    pub link_only: Option<bool>,
    pub post_broker_login_flow_alias: Option<Cow<'a, str>>,
    pub provider_id: Option<Cow<'a, str>>,
    pub store_token: Option<bool>,
    pub trust_email: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyStoreConfig<'a> {
    pub format: Option<Cow<'a, str>>,
    pub key_alias: Option<Cow<'a, str>>,
    pub key_password: Option<Cow<'a, str>>,
    pub realm_alias: Option<Cow<'a, str>>,
    pub realm_certificate: Option<bool>,
    pub store_password: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeysMetadataRepresentation<'a> {
    pub active: Option<HashMap<Cow<'a, str>, Value>>,
    pub keys: Option<Vec<KeysMetadataRepresentationKeyMetadataRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeysMetadataRepresentationKeyMetadataRepresentation<'a> {
    pub algorithm: Option<Cow<'a, str>>,
    pub certificate: Option<Cow<'a, str>>,
    pub kid: Option<Cow<'a, str>>,
    pub provider_id: Option<Cow<'a, str>>,
    pub provider_priority: Option<i64>,
    pub public_key: Option<Cow<'a, str>>,
    pub status: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPermissionReference<'a> {
    pub enabled: Option<bool>,
    pub resource: Option<Cow<'a, str>>,
    pub scope_permissions: Option<HashMap<Cow<'a, str>, Value>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MappingsRepresentation<'a> {
    pub client_mappings: Option<HashMap<Cow<'a, str>, Value>>,
    pub realm_mappings: Option<Vec<RoleRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfoRepresentation<'a> {
    pub free: Option<i64>,
    pub free_formated: Option<Cow<'a, str>>,
    pub free_percentage: Option<i64>,
    pub total: Option<i64>,
    pub total_formated: Option<Cow<'a, str>>,
    pub used: Option<i64>,
    pub used_formated: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultivaluedHashMap {
    pub empty: Option<bool>,
    pub load_factor: Option<f32>,
    pub threshold: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialImportRepresentation<'a> {
    pub clients: Option<Vec<ClientRepresentation<'a>>>,
    pub groups: Option<Vec<GroupRepresentation<'a>>>,
    pub identity_providers: Option<Vec<IdentityProviderRepresentation<'a>>>,
    pub if_resource_exists: Option<Cow<'a, str>>,
    pub policy: Option<PartialImportRepresentationPolicy>,
    pub roles: Option<RolesRepresentation<'a>>,
    pub users: Option<Vec<UserRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordPolicyTypeRepresentation<'a> {
    pub config_type: Option<Cow<'a, str>>,
    pub default_value: Option<Cow<'a, str>>,
    pub display_name: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub multiple_supported: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Permission<'a> {
    pub claims: Option<HashMap<Cow<'a, str>, Value>>,
    pub rsid: Option<Cow<'a, str>>,
    pub rsname: Option<Cow<'a, str>>,
    pub scopes: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRepresentation<'a> {
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub decision_strategy: Option<PolicyRepresentationDecisionStrategy>,
    pub description: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub logic: Option<PolicyRepresentationLogic>,
    pub name: Option<Cow<'a, str>>,
    pub owner: Option<Cow<'a, str>>,
    pub policies: Option<Vec<Cow<'a, str>>>,
    pub resources: Option<Vec<Cow<'a, str>>>,
    pub resources_data: Option<Vec<ResourceRepresentation<'a>>>,
    pub scopes: Option<Vec<Cow<'a, str>>>,
    pub scopes_data: Option<Vec<ScopeRepresentation<'a>>>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoRepresentation<'a> {
    pub disabled_features: Option<Vec<Cow<'a, str>>>,
    pub experimental_features: Option<Vec<Cow<'a, str>>>,
    pub name: Option<Cow<'a, str>>,
    pub preview_features: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperRepresentation<'a> {
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub protocol: Option<Cow<'a, str>>,
    pub protocol_mapper: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRepresentation<'a> {
    pub operational_info: Option<HashMap<Cow<'a, str>, Value>>,
    pub order: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigRepresentation<'a> {
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub enabled_event_types: Option<Vec<Cow<'a, str>>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RealmRepresentation<'a> {
    pub access_code_lifespan: Option<i32>,
    pub access_code_lifespan_login: Option<i32>,
    pub access_code_lifespan_user_action: Option<i32>,
    pub access_token_lifespan: Option<i32>,
    pub access_token_lifespan_for_implicit_flow: Option<i32>,
    pub account_theme: Option<Cow<'a, str>>,
    pub action_token_generated_by_admin_lifespan: Option<i32>,
    pub action_token_generated_by_user_lifespan: Option<i32>,
    pub admin_events_details_enabled: Option<bool>,
    pub admin_events_enabled: Option<bool>,
    pub admin_theme: Option<Cow<'a, str>>,
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub authentication_flows: Option<Vec<AuthenticationFlowRepresentation<'a>>>,
    pub authenticator_config: Option<Vec<AuthenticatorConfigRepresentation<'a>>>,
    pub browser_flow: Option<Cow<'a, str>>,
    pub browser_security_headers: Option<HashMap<Cow<'a, str>, Value>>,
    pub brute_force_protected: Option<bool>,
    pub client_authentication_flow: Option<Cow<'a, str>>,
    pub client_scope_mappings: Option<HashMap<Cow<'a, str>, Value>>,
    pub client_scopes: Option<Vec<ClientScopeRepresentation<'a>>>,
    pub client_session_idle_timeout: Option<i32>,
    pub client_session_max_lifespan: Option<i32>,
    pub clients: Option<Vec<ClientRepresentation<'a>>>,
    pub components: Option<MultivaluedHashMap>,
    pub default_default_client_scopes: Option<Vec<Cow<'a, str>>>,
    pub default_groups: Option<Vec<Cow<'a, str>>>,
    pub default_locale: Option<Cow<'a, str>>,
    pub default_optional_client_scopes: Option<Vec<Cow<'a, str>>>,
    pub default_roles: Option<Vec<Cow<'a, str>>>,
    pub default_signature_algorithm: Option<Cow<'a, str>>,
    pub direct_grant_flow: Option<Cow<'a, str>>,
    pub display_name: Option<Cow<'a, str>>,
    pub display_name_html: Option<Cow<'a, str>>,
    pub docker_authentication_flow: Option<Cow<'a, str>>,
    pub duplicate_emails_allowed: Option<bool>,
    pub edit_username_allowed: Option<bool>,
    pub email_theme: Option<Cow<'a, str>>,
    pub enabled: Option<bool>,
    pub enabled_event_types: Option<Vec<Cow<'a, str>>>,
    pub events_enabled: Option<bool>,
    pub events_expiration: Option<i64>,
    pub events_listeners: Option<Vec<Cow<'a, str>>>,
    pub failure_factor: Option<i32>,
    pub federated_users: Option<Vec<UserRepresentation<'a>>>,
    pub groups: Option<Vec<GroupRepresentation<'a>>>,
    pub id: Option<Cow<'a, str>>,
    pub identity_provider_mappers: Option<Vec<IdentityProviderMapperRepresentation<'a>>>,
    pub identity_providers: Option<Vec<IdentityProviderRepresentation<'a>>>,
    pub internationalization_enabled: Option<bool>,
    pub keycloak_version: Option<Cow<'a, str>>,
    pub login_theme: Option<Cow<'a, str>>,
    pub login_with_email_allowed: Option<bool>,
    pub max_delta_time_seconds: Option<i32>,
    pub max_failure_wait_seconds: Option<i32>,
    pub minimum_quick_login_wait_seconds: Option<i32>,
    pub not_before: Option<i32>,
    pub offline_session_idle_timeout: Option<i32>,
    pub offline_session_max_lifespan: Option<i32>,
    pub offline_session_max_lifespan_enabled: Option<bool>,
    pub otp_policy_algorithm: Option<Cow<'a, str>>,
    pub otp_policy_digits: Option<i32>,
    pub otp_policy_initial_counter: Option<i32>,
    pub otp_policy_look_ahead_window: Option<i32>,
    pub otp_policy_period: Option<i32>,
    pub otp_policy_type: Option<Cow<'a, str>>,
    pub otp_supported_applications: Option<Vec<Cow<'a, str>>>,
    pub password_policy: Option<Cow<'a, str>>,
    pub permanent_lockout: Option<bool>,
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation<'a>>>,
    pub quick_login_check_milli_seconds: Option<i64>,
    pub realm: Option<Cow<'a, str>>,
    pub refresh_token_max_reuse: Option<i32>,
    pub registration_allowed: Option<bool>,
    pub registration_email_as_username: Option<bool>,
    pub registration_flow: Option<Cow<'a, str>>,
    pub remember_me: Option<bool>,
    pub required_actions: Option<Vec<RequiredActionProviderRepresentation<'a>>>,
    pub reset_credentials_flow: Option<Cow<'a, str>>,
    pub reset_password_allowed: Option<bool>,
    pub revoke_refresh_token: Option<bool>,
    pub roles: Option<RolesRepresentation<'a>>,
    pub scope_mappings: Option<Vec<ScopeMappingRepresentation<'a>>>,
    pub smtp_server: Option<HashMap<Cow<'a, str>, Value>>,
    pub ssl_required: Option<Cow<'a, str>>,
    pub sso_session_idle_timeout: Option<i32>,
    pub sso_session_idle_timeout_remember_me: Option<i32>,
    pub sso_session_max_lifespan: Option<i32>,
    pub sso_session_max_lifespan_remember_me: Option<i32>,
    pub supported_locales: Option<Vec<Cow<'a, str>>>,
    pub user_federation_mappers: Option<Vec<UserFederationMapperRepresentation<'a>>>,
    pub user_federation_providers: Option<Vec<UserFederationProviderRepresentation<'a>>>,
    pub user_managed_access_allowed: Option<bool>,
    pub users: Option<Vec<UserRepresentation<'a>>>,
    pub verify_email: Option<bool>,
    pub wait_increment_seconds: Option<i32>,
    pub web_authn_policy_acceptable_aaguids: Option<Vec<Cow<'a, str>>>,
    pub web_authn_policy_attestation_conveyance_preference: Option<Cow<'a, str>>,
    pub web_authn_policy_authenticator_attachment: Option<Cow<'a, str>>,
    pub web_authn_policy_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_create_timeout: Option<i32>,
    pub web_authn_policy_passwordless_acceptable_aaguids: Option<Vec<Cow<'a, str>>>,
    pub web_authn_policy_passwordless_attestation_conveyance_preference: Option<Cow<'a, str>>,
    pub web_authn_policy_passwordless_authenticator_attachment: Option<Cow<'a, str>>,
    pub web_authn_policy_passwordless_avoid_same_authenticator_register: Option<bool>,
    pub web_authn_policy_passwordless_create_timeout: Option<i32>,
    pub web_authn_policy_passwordless_require_resident_key: Option<Cow<'a, str>>,
    pub web_authn_policy_passwordless_rp_entity_name: Option<Cow<'a, str>>,
    pub web_authn_policy_passwordless_rp_id: Option<Cow<'a, str>>,
    pub web_authn_policy_passwordless_signature_algorithms: Option<Vec<Cow<'a, str>>>,
    pub web_authn_policy_passwordless_user_verification_requirement: Option<Cow<'a, str>>,
    pub web_authn_policy_require_resident_key: Option<Cow<'a, str>>,
    pub web_authn_policy_rp_entity_name: Option<Cow<'a, str>>,
    pub web_authn_policy_rp_id: Option<Cow<'a, str>>,
    pub web_authn_policy_signature_algorithms: Option<Vec<Cow<'a, str>>>,
    pub web_authn_policy_user_verification_requirement: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredActionProviderRepresentation<'a> {
    pub alias: Option<Cow<'a, str>>,
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub default_action: Option<bool>,
    pub enabled: Option<bool>,
    pub name: Option<Cow<'a, str>>,
    pub priority: Option<i32>,
    pub provider_id: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRepresentation<'a> {
    pub id: Cow<'a, str>,
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub display_name: Option<Cow<'a, str>>,
    pub icon_uri: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub owner_managed_access: Option<bool>,
    pub scopes: Option<Vec<ScopeRepresentation<'a>>>,
    #[serde(rename = "type")]
    pub type_: Option<Cow<'a, str>>,
    pub uris: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerRepresentation<'a> {
    pub allow_remote_resource_management: Option<bool>,
    pub client_id: Option<Cow<'a, str>>,
    pub decision_strategy: Option<ResourceServerRepresentationDecisionStrategy>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub policies: Option<Vec<PolicyRepresentation<'a>>>,
    pub policy_enforcement_mode: Option<ResourceServerRepresentationPolicyEnforcementMode>,
    pub resources: Option<Vec<ResourceRepresentation<'a>>>,
    pub scopes: Option<Vec<ScopeRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRepresentation<'a> {
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub client_role: Option<bool>,
    pub composite: Option<bool>,
    pub composites: Option<RoleRepresentationComposites<'a>>,
    pub container_id: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RoleRepresentationComposites<'a> {
    pub client: Option<HashMap<Cow<'a, str>, Value>>,
    pub realm: Option<Vec<Cow<'a, str>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RolesRepresentation<'a> {
    pub client: Option<HashMap<Cow<'a, str>, Value>>,
    pub realm: Option<Vec<RoleRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeMappingRepresentation<'a> {
    pub client: Option<Cow<'a, str>>,
    pub client_scope: Option<Cow<'a, str>>,
    pub roles: Option<Vec<Cow<'a, str>>>,
    #[serde(rename = "self")]
    pub self_: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeRepresentation<'a> {
    pub display_name: Option<Cow<'a, str>>,
    pub icon_uri: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
    pub policies: Option<Vec<PolicyRepresentation<'a>>>,
    pub resources: Option<Vec<ResourceRepresentation<'a>>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoRepresentation<'a> {
    pub builtin_protocol_mappers: Option<HashMap<Cow<'a, str>, Value>>,
    pub client_importers: Option<Vec<HashMap<Cow<'a, str>, Value>>>,
    pub client_installations: Option<HashMap<Cow<'a, str>, Value>>,
    pub component_types: Option<HashMap<Cow<'a, str>, Value>>,
    pub enums: Option<HashMap<Cow<'a, str>, Value>>,
    pub identity_providers: Option<Vec<HashMap<Cow<'a, str>, Value>>>,
    pub memory_info: Option<MemoryInfoRepresentation<'a>>,
    pub password_policies: Option<Vec<PasswordPolicyTypeRepresentation<'a>>>,
    pub profile_info: Option<ProfileInfoRepresentation<'a>>,
    pub protocol_mapper_types: Option<HashMap<Cow<'a, str>, Value>>,
    pub providers: Option<HashMap<Cow<'a, str>, Value>>,
    pub social_providers: Option<Vec<HashMap<Cow<'a, str>, Value>>>,
    pub system_info: Option<SystemInfoRepresentation<'a>>,
    pub themes: Option<HashMap<Cow<'a, str>, Value>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SpiInfoRepresentation<'a> {
    pub internal: Option<bool>,
    pub providers: Option<HashMap<Cow<'a, str>, Value>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SynchronizationResult<'a> {
    pub added: Option<i32>,
    pub failed: Option<i32>,
    pub ignored: Option<bool>,
    pub removed: Option<i32>,
    pub status: Option<Cow<'a, str>>,
    pub updated: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoRepresentation<'a> {
    pub file_encoding: Option<Cow<'a, str>>,
    pub java_home: Option<Cow<'a, str>>,
    pub java_runtime: Option<Cow<'a, str>>,
    pub java_vendor: Option<Cow<'a, str>>,
    pub java_version: Option<Cow<'a, str>>,
    pub java_vm: Option<Cow<'a, str>>,
    pub java_vm_version: Option<Cow<'a, str>>,
    pub os_architecture: Option<Cow<'a, str>>,
    pub os_name: Option<Cow<'a, str>>,
    pub os_version: Option<Cow<'a, str>>,
    pub server_time: Option<Cow<'a, str>>,
    pub uptime: Option<Cow<'a, str>>,
    pub uptime_millis: Option<i64>,
    pub user_dir: Option<Cow<'a, str>>,
    pub user_locale: Option<Cow<'a, str>>,
    pub user_name: Option<Cow<'a, str>>,
    pub user_timezone: Option<Cow<'a, str>>,
    pub version: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestLdapConnectionRepresentation<'a> {
    pub action: Option<Cow<'a, str>>,
    pub bind_credential: Option<Cow<'a, str>>,
    pub bind_dn: Option<Cow<'a, str>>,
    pub component_id: Option<Cow<'a, str>>,
    pub connection_timeout: Option<Cow<'a, str>>,
    pub connection_url: Option<Cow<'a, str>>,
    pub start_tls: Option<Cow<'a, str>>,
    pub use_truststore_spi: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConsentRepresentation<'a> {
    pub client_id: Option<Cow<'a, str>>,
    pub created_date: Option<i64>,
    pub granted_client_scopes: Option<Vec<Cow<'a, str>>>,
    pub last_updated_date: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFederationMapperRepresentation<'a> {
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub federation_mapper_type: Option<Cow<'a, str>>,
    pub federation_provider_display_name: Option<Cow<'a, str>>,
    pub id: Option<Cow<'a, str>>,
    pub name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFederationProviderRepresentation<'a> {
    pub changed_sync_period: Option<i32>,
    pub config: Option<HashMap<Cow<'a, str>, Value>>,
    pub display_name: Option<Cow<'a, str>>,
    pub full_sync_period: Option<i32>,
    pub id: Option<Cow<'a, str>>,
    pub last_sync: Option<i32>,
    pub priority: Option<i32>,
    pub provider_name: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRepresentation<'a> {
    pub access: Option<HashMap<Cow<'a, str>, Value>>,
    pub attributes: Option<HashMap<Cow<'a, str>, Value>>,
    pub client_consents: Option<Vec<UserConsentRepresentation<'a>>>,
    pub client_roles: Option<HashMap<Cow<'a, str>, Value>>,
    pub created_timestamp: Option<i64>,
    pub credentials: Option<Vec<CredentialRepresentation<'a>>>,
    pub disableable_credential_types: Option<Vec<Cow<'a, str>>>,
    pub email: Option<Cow<'a, str>>,
    pub email_verified: Option<bool>,
    pub enabled: Option<bool>,
    pub federated_identities: Option<Vec<FederatedIdentityRepresentation<'a>>>,
    pub federation_link: Option<Cow<'a, str>>,
    pub first_name: Option<Cow<'a, str>>,
    pub groups: Option<Vec<Cow<'a, str>>>,
    pub id: Option<Cow<'a, str>>,
    pub last_name: Option<Cow<'a, str>>,
    pub not_before: Option<i32>,
    pub origin: Option<Cow<'a, str>>,
    pub realm_roles: Option<Vec<Cow<'a, str>>>,
    pub required_actions: Option<Vec<Cow<'a, str>>>,
    #[serde(rename = "self")]
    pub self_: Option<Cow<'a, str>>,
    pub service_account_client_id: Option<Cow<'a, str>>,
    pub username: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSessionRepresentation<'a> {
    pub clients: Option<HashMap<Cow<'a, str>, Value>>,
    pub id: Option<Cow<'a, str>>,
    pub ip_address: Option<Cow<'a, str>>,
    pub last_access: Option<i64>,
    pub start: Option<i64>,
    pub user_id: Option<Cow<'a, str>>,
    pub username: Option<Cow<'a, str>>,
}
