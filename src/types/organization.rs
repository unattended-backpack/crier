use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Organization {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: Option<String>,
    
    // Optional fields
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub twitter_username: Option<String>,
    pub is_verified: Option<bool>,
    pub has_organization_projects: Option<bool>,
    pub has_repository_projects: Option<bool>,
    pub public_repos: Option<i64>,
    pub public_gists: Option<i64>,
    pub followers: Option<i64>,
    pub following: Option<i64>,
    pub html_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(rename = "type")]
    pub org_type: Option<String>,
}