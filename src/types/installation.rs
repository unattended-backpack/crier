use serde::{Deserialize, Serialize};
use super::User;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub account_type: String,  // "User" or "Organization"
    pub site_admin: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Installation {
    pub id: i64,
    #[serde(default)]
    pub node_id: String,
    pub account: Account,
    pub repository_selection: Option<String>,
    pub access_tokens_url: String,
    pub repositories_url: String,
    pub html_url: String,
    pub app_id: Option<i64>,
    pub app_slug: Option<String>,
    pub target_id: Option<i64>,
    pub target_type: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub events: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
    pub single_file_name: Option<String>,
    pub has_multiple_single_files: Option<bool>,
    pub single_file_paths: Option<Vec<String>>,
    pub suspended_by: Option<User>,
    pub suspended_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstallationLite {
    pub id: i64,
    pub node_id: String,
}