use std::fmt::Display;

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    CustomResource, Default, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, JsonSchema,
)]
#[kube(
    group = "baarsgaard.dev",
    kind = "PasswordState",
    plural = "passwordstates",
    shortname = "pstate",
    version = "v1alpha1",
    namespaced,
    doc = "",
    derive = "PartialEq",
    derive = "Default",
    printcolumn = r#"{"name":"Url", "type":"string", "description":"Url of a Password State instance", "jsonPath": ".spec.url"}"#
)]
pub struct PasswordStateSpec {
    pub url: String,
    /// Default Resync Period in seconds
    // XXX Consider implementing "OFF" state for no resync unless 'last_sync' annotation is set to zero.
    // Allows force syncing when necessary.
    pub resync_period: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct ApiKeySecretRef {
    pub name: String,
    pub namespace: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct ReplicationStatus {
    last_reconciled: String,
    result: ReplicationResult,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum ReplicationResult {
    Success,
    Failure(String),
}
impl Display for ReplicationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ReplicationResult::Success => "Reconciled".to_string(),
            ReplicationResult::Failure(e) => format!("Error: {}", e),
        };
        write!(f, "{}", res)
    }
}

#[derive(
    CustomResource, Default, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, JsonSchema,
)]
#[kube(
    group = "baarsgaard.dev",
    kind = "PasswordStateSecret",
    plural = "passwordstatesecrets",
    shortname = "psecrets",
    version = "v1alpha1",
    namespaced,
    doc = "",
    status = "ReplicationStatus",
    derive = "PartialEq",
    derive = "Default",
    printcolumn = r#"{"name":"Url", "type":"string", "description":"Url of a Password State instance", "jsonPath": ".spec.url"}"#
)]
pub struct SecretSpec {
    /// Mount point of the kv1 engine. Default: kv/
    /// title is the name of each entry in the list lowercased and spaces replaced with underscores.
    pub vault_mount_path: String,
    /// Secret ID
    pub id: u32,
    pub fields: Option<FieldsConfigSpec>,
    /// Secret containting data.apiKey: xxxxxx
    pub api_key_secret_ref: Option<ApiKeySecretRef>,
    /// Resync Period in seconds
    pub resync_period: String,
    pub password_state_ref: PasswordStateRef,
}
#[derive(
    CustomResource, Default, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, JsonSchema,
)]
pub struct ListSpec {
    /// Mount point of the kv1 engine.
    /// Default: kv/{{ metadata.name }}
    /// The Path will be the title of each Secret normalised to match the following regex
    /// [A-Za-z0-9][_-\.A-Za-z0-9]*[A-Za-z0-9]
    pub vault_mount_path: String,
    /// List ID
    pub id: u32,
    pub r#type: FieldsConfigSpec,
    /// Secret containting data.apiKey: xxxxxx
    pub api_key_secret_ref: Option<ApiKeySecretRef>,
    /// Resync Period in seconds
    pub resync_period: String,
    pub password_state_ref: PasswordStateRef,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum FieldsConfigSpec {
    /// Only UserName & Password
    Minimal,
    /// All Fields
    All,
    /// List of Strings matching fieleds to store in Vault
    Custom(Vec<String>),
}
impl Default for FieldsConfigSpec {
    fn default() -> Self {
        Self::Minimal
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct PasswordStateRef {
    pub namespace: Option<String>,
    pub name: String,
}
