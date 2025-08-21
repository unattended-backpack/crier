use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckRunEvent {
    pub action: CheckRunAction,
    pub check_run: CheckRun,
    pub repository: Repository,
    pub sender: User,
    pub requested_action: Option<RequestedAction>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunAction {
    Created,
    Completed,
    Rerequested,
    RequestedAction,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckRun {
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub head_sha: String,
    pub external_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub details_url: Option<String>,
    pub status: String,  // "queued", "in_progress", "completed"
    pub conclusion: Option<String>,  // "success", "failure", "neutral", "cancelled", "skipped", "timed_out", "action_required"
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub output: CheckRunOutput,
    pub check_suite: CheckSuite,
    pub app: App,
    pub pull_requests: Vec<CheckRunPullRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckRunOutput {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub annotations_count: i64,
    pub annotations_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckSuite {
    pub id: i64,
    pub node_id: String,
    pub head_branch: String,
    pub head_sha: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub url: String,
    pub before: String,
    pub after: String,
    pub pull_requests: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct App {
    pub id: i64,
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: User,
    pub name: String,
    pub description: Option<String>,
    pub external_url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CheckRunPullRequest {
    pub url: String,
    pub id: i64,
    pub number: i64,
    pub head: PRRef,
    pub base: PRRef,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PRRef {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
    pub repo: Repository,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RequestedAction {
    pub identifier: String,
}

impl DiscordTransform for CheckRunEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let run = &self.check_run;
        
        let (color, emoji, status_text) = match self.action {
            CheckRunAction::Created => (Colors::DULL_BLUE, "🔄", "created"),
            CheckRunAction::Rerequested => (Colors::YELLOW, "🔁", "re-requested"),
            CheckRunAction::RequestedAction => (Colors::YELLOW, "⚡", "action requested"),
            CheckRunAction::Completed => {
                match run.conclusion.as_deref() {
                    Some("success") => (Colors::GREEN, "✅", "completed successfully"),
                    Some("failure") => (Colors::RED, "❌", "failed"),
                    Some("neutral") => (Colors::GRAY, "➖", "completed (neutral)"),
                    Some("cancelled") => (Colors::DULL_YELLOW, "🚫", "cancelled"),
                    Some("skipped") => (Colors::GRAY, "⏭️", "skipped"),
                    Some("timed_out") => (Colors::RED, "⏰", "timed out"),
                    Some("action_required") => (Colors::YELLOW, "⚠️", "action required"),
                    _ => (Colors::GRAY, "❓", "completed"),
                }
            }
        };
        
        let title = format!("{} Check: {}", emoji, run.name);
        
        let mut fields = vec![];
        
        // Status
        fields.push(field("Status", status_text, true));
        
        // App that created the check
        fields.push(field("App", &run.app.name, true));
        
        // Branch
        fields.push(code_field("Branch", &run.check_suite.head_branch, true));
        
        // Commit
        let short_sha = if run.head_sha.len() >= 7 {
            &run.head_sha[..7]
        } else {
            &run.head_sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Output summary if available
        if let Some(summary) = &run.output.summary {
            if !summary.is_empty() {
                let truncated = if summary.len() > 500 {
                    format!("{}...", &summary[..497])
                } else {
                    summary.clone()
                };
                fields.push(field("Summary", truncated, false));
            }
        }
        
        // Annotations count
        if run.output.annotations_count > 0 {
            fields.push(field(
                "Annotations",
                format!("⚠️ {}", run.output.annotations_count),
                true
            ));
        }
        
        // Associated PRs
        if !run.pull_requests.is_empty() {
            let pr_nums = run.pull_requests.iter()
                .take(5)
                .map(|pr| format!("#{}", pr.number))
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("Pull Requests", pr_nums, true));
        }
        
        // Requested action if applicable
        if let Some(action) = &self.requested_action {
            fields.push(field("Requested Action", &action.identifier, true));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = run.output.title.clone().or_else(|| 
            Some(format!("Check run '{}' {}", run.name, status_text))
        );
        
        DiscordEmbed {
            title,
            description,
            url: run.details_url.clone().or_else(|| Some(run.html_url.clone())),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: format!("GitHub Checks • {}", run.app.name),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}