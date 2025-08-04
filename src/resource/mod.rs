use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde_json::Value;

use crate::{
    types::*, DefaultResponse, KeycloakError, KeycloakRealmAdmin, KeycloakRealmAdminMethod,
    KeycloakTokenSupplier,
};

/// Attack Detection
#[cfg(feature = "tag-attack-detection")]
mod attack_detection;
/// Authentication Management
#[cfg(feature = "tag-authentication-management")]
mod authentication_management;
/// Client Attribute Certificate
#[cfg(feature = "tag-client-attribute-certificate")]
mod client_attribute_certificate;
/// Client Initial Access
#[cfg(feature = "tag-client-initial-access")]
mod client_initial_access;
/// Client Registration Policy
#[cfg(feature = "tag-client-registration-policy")]
mod client_registration_policy;
/// Client Role Mappings
#[cfg(feature = "tag-client-role-mappings")]
mod client_role_mappings;
/// Client Scopes
#[cfg(feature = "tag-client-scopes")]
mod client_scopes;
/// Clients
#[cfg(feature = "tag-clients")]
mod clients;
/// Component
#[cfg(feature = "tag-component")]
mod component;
/// Groups
#[cfg(feature = "tag-groups")]
mod groups;
/// Identity Providers
#[cfg(feature = "tag-identity-providers")]
mod identity_providers;
/// Key
#[cfg(feature = "tag-key")]
mod key;
/// Organizations
#[cfg(feature = "tag-organizations")]
mod organizations;
/// Other (non tagged) methods
#[cfg(feature = "tag-none")]
mod other_methods;
/// Protocol Mappers
#[cfg(feature = "tag-protocol-mappers")]
mod protocol_mappers;
/// Realms Admin
#[cfg(feature = "tag-realms-admin")]
mod realms_admin;
/// Role Mapper
#[cfg(feature = "tag-role-mapper")]
mod role_mapper;
/// Roles
#[cfg(feature = "tag-roles")]
mod roles;
/// Roles (by ID)
#[cfg(feature = "tag-roles-by-id")]
mod roles_by_id;
/// Scope Mappings
#[cfg(feature = "tag-scope-mappings")]
mod scope_mappings;
/// Users
#[cfg(feature = "tag-users")]
mod users;
