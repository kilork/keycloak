use reqwest::header::CONTENT_LENGTH;
use serde_json::Value;

use super::{url_enc::encode_url_param as p, *};

/// Attack Detection
#[cfg(feature = "tag-attack-detection")]
pub mod attack_detection;
/// Authentication Management
#[cfg(feature = "tag-authentication-management")]
pub mod authentication_management;
/// Client Attribute Certificate
#[cfg(feature = "tag-client-attribute-certificate")]
pub mod client_attribute_certificate;
/// Client Initial Access
#[cfg(feature = "tag-client-initial-access")]
pub mod client_initial_access;
/// Client Registration Policy
#[cfg(feature = "tag-client-registration-policy")]
pub mod client_registration_policy;
/// Client Role Mappings
#[cfg(feature = "tag-client-role-mappings")]
pub mod client_role_mappings;
/// Client Scopes
#[cfg(feature = "tag-client-scopes")]
pub mod client_scopes;
/// Clients
#[cfg(feature = "tag-clients")]
pub mod clients;
/// Component
#[cfg(feature = "tag-component")]
pub mod component;
/// Groups
#[cfg(feature = "tag-groups")]
pub mod groups;
/// Identity Providers
#[cfg(feature = "tag-identity-providers")]
pub mod identity_providers;
/// Key
#[cfg(feature = "tag-key")]
pub mod key;
/// Organizations
#[cfg(feature = "tag-organizations")]
pub mod organizations;
/// Other (non tagged) methods
#[cfg(feature = "tag-none")]
pub mod other_methods;
/// Protocol Mappers
#[cfg(feature = "tag-protocol-mappers")]
pub mod protocol_mappers;
/// Realms Admin
#[cfg(feature = "tag-realms-admin")]
pub mod realms_admin;
/// Role Mapper
#[cfg(feature = "tag-role-mapper")]
pub mod role_mapper;
/// Roles
#[cfg(feature = "tag-roles")]
pub mod roles;
/// Roles (by ID)
#[cfg(feature = "tag-roles-by-id")]
pub mod roles_by_id;
/// Scope Mappings
#[cfg(feature = "tag-scope-mappings")]
pub mod scope_mappings;
/// Users
#[cfg(feature = "tag-users")]
pub mod users;
/// Workflow Steps
#[cfg(feature = "tag-workflow-steps")]
pub mod workflow_steps;
