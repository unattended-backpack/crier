use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::check_run::App;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckSuiteEvent {
    pub action: CheckSuiteAction,
    pub check_suite: CheckSuite,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteAction {
    Completed,
    Requested,
    Rerequested,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckSuite {
    pub id: i64,
    pub node_id: String,
    pub head_branch: String,
    pub head_sha: String,
    pub status: String,  // "queued", "in_progress", "completed"
    pub conclusion: Option<String>,  // "success", "failure", "neutral", "cancelled", "timed_out", "action_required", "stale"
    pub url: String,
    pub before: String,
    pub after: String,
    pub pull_requests: Vec<serde_json::Value>,
    pub app: App,
    pub created_at: String,
    pub updated_at: String,
    pub latest_check_runs_count: i64,
    pub check_runs_url: String,
    pub head_commit: Option<CheckSuiteCommit>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckSuiteCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
}

impl DiscordTransform for CheckSuiteEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let suite = &self.check_suite;
        
        let (color, emoji, status_text) = match self.action {
            CheckSuiteAction::Requested => (Colors::DULL_BLUE, "📋", "requested"),
            CheckSuiteAction::Rerequested => (Colors::YELLOW, "🔁", "re-requested"),
            CheckSuiteAction::Completed => {
                match suite.conclusion.as_deref() {
                    Some("success") => (Colors::GREEN, "✅", "completed successfully"),
                    Some("failure") => (Colors::RED, "❌", "failed"),
                    Some("neutral") => (Colors::GRAY, "➖", "completed (neutral)"),
                    Some("cancelled") => (Colors::DULL_YELLOW, "🚫", "cancelled"),
                    Some("timed_out") => (Colors::RED, "⏰", "timed out"),
                    Some("action_required") => (Colors::YELLOW, "⚠️", "action required"),
                    Some("stale") => (Colors::DULL_YELLOW, "📅", "stale"),
                    _ => (Colors::GRAY, "❓", "completed"),
                }
            }
        };
        
        let title = format!("{} Check Suite {}", emoji, status_text);
        
        let mut fields = vec![];
        
        // Status
        fields.push(field("Status", status_text, true));
        
        // App that created the suite
        fields.push(field("App", &suite.app.name, true));
        
        // Check runs count
        if suite.latest_check_runs_count > 0 {
            fields.push(field(
                "Check Runs",
                suite.latest_check_runs_count.to_string(),
                true
            ));
        }
        
        // Branch
        fields.push(code_field("Branch", &suite.head_branch, true));
        
        // Commit
        let short_sha = if suite.head_sha.len() >= 7 {
            &suite.head_sha[..7]
        } else {
            &suite.head_sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Commit message if available
        if let Some(commit) = &suite.head_commit {
            let msg = if commit.message.len() > 100 {
                format!("{}...", &commit.message[..97])
            } else {
                commit.message.clone()
            };
            fields.push(field("Commit Message", msg, false));
        }
        
        // Associated PRs
        if !suite.pull_requests.is_empty() {
            fields.push(field(
                "Pull Requests",
                format!("{} PR(s)", suite.pull_requests.len()),
                true
            ));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Check suite {} for commit {} on {}",
            status_text, short_sha, suite.head_branch
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!(
                "{}/commit/{}/checks",
                self.repository.html_url, suite.head_sha
            )),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: format!("GitHub Checks • {}", suite.app.name),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}