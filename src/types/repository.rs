use serde::{Deserialize, Serialize};
use super::{User, Visibility};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: User,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    
    // Timestamps
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: Option<String>,
    
    // Repository stats
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_discussions: Option<bool>,
    pub forks_count: i64,
    pub mirror_url: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub license: Option<License>,
    pub allow_forking: Option<bool>,
    pub is_template: Option<bool>,
    pub web_commit_signoff_required: Option<bool>,
    pub topics: Option<Vec<String>>,
    pub visibility: Option<Visibility>,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
    
    // Optional fields for different contexts
    pub permissions: Option<Permissions>,
    pub temp_clone_token: Option<String>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub allow_update_branch: Option<bool>,
    pub use_squash_pr_title_as_default: Option<bool>,
    pub squash_merge_commit_message: Option<String>,
    pub squash_merge_commit_title: Option<String>,
    pub merge_commit_message: Option<String>,
    pub merge_commit_title: Option<String>,
    pub custom_properties: Option<serde_json::Value>,
    pub organization: Option<String>,
    pub security_and_analysis: Option<SecurityAndAnalysis>,
    pub network_count: Option<i64>,
    pub subscribers_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Permissions {
    pub admin: Option<bool>,
    pub maintain: Option<bool>,
    pub push: Option<bool>,
    pub triage: Option<bool>,
    pub pull: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityAndAnalysis {
    pub advanced_security: Option<SecurityFeature>,
    pub dependabot_security_updates: Option<SecurityFeature>,
    pub secret_scanning: Option<SecurityFeature>,
    pub secret_scanning_push_protection: Option<SecurityFeature>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityFeature {
    pub status: String,
}