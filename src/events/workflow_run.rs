use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::pull_request::PullRequest;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowRunEvent {
    pub action: WorkflowRunAction,
    pub workflow_run: WorkflowRun,
    pub workflow: Workflow,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunAction {
    Requested,
    Completed,
    InProgress,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowRun {
    pub id: i64,
    pub name: Option<String>,
    pub node_id: String,
    pub head_branch: String,
    pub head_sha: String,
    pub path: String,
    pub display_title: String,
    pub run_number: i64,
    pub event: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub workflow_id: i64,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    pub url: String,
    pub html_url: String,
    pub pull_requests: Vec<PullRequest>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub actor: Option<User>,
    pub run_attempt: i64,
    pub referenced_workflows: Option<Vec<serde_json::Value>>,
    pub run_started_at: String,
    pub triggering_actor: User,
    pub jobs_url: String,
    pub logs_url: String,
    pub check_suite_url: String,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub rerun_url: String,
    pub previous_attempt_url: Option<String>,
    pub workflow_url: String,
    pub head_commit: HeadCommit,
    pub repository: Repository,
    pub head_repository: Repository,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub html_url: String,
    pub badge_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HeadCommit {
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

impl DiscordTransform for WorkflowRunEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let run = &self.workflow_run;
        let workflow = &self.workflow;
        
        let (color, emoji, status_text) = match self.action {
            WorkflowRunAction::Requested => (Colors::DULL_BLUE, "🔄", "requested"),
            WorkflowRunAction::InProgress => (Colors::YELLOW, "⚡", "in progress"),
            WorkflowRunAction::Completed => {
                match run.conclusion.as_deref() {
                    Some("success") => (Colors::GREEN, "✅", "completed successfully"),
                    Some("failure") => (Colors::RED, "❌", "failed"),
                    Some("cancelled") => (Colors::DULL_YELLOW, "🚫", "cancelled"),
                    Some("skipped") => (Colors::GRAY, "⏭️", "skipped"),
                    Some("timed_out") => (Colors::RED, "⏰", "timed out"),
                    Some("action_required") => (Colors::YELLOW, "⚠️", "action required"),
                    Some("neutral") => (Colors::GRAY, "➖", "completed (neutral)"),
                    Some("stale") => (Colors::DULL_YELLOW, "📅", "stale"),
                    _ => (Colors::GRAY, "❓", "completed"),
                }
            }
        };
        
        let title = format!("{} Workflow: {}", emoji, workflow.name);
        
        let mut fields = vec![];
        
        // Workflow name and run number
        fields.push(field("Run", format!("#{}", run.run_number), true));
        
        // Status and conclusion
        fields.push(field("Status", status_text, true));
        
        // Trigger event
        fields.push(field("Triggered By", &run.event, true));
        
        // Branch
        fields.push(code_field("Branch", &run.head_branch, true));
        
        // Commit
        let short_sha = if run.head_sha.len() >= 7 {
            &run.head_sha[..7]
        } else {
            &run.head_sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Run attempt
        if run.run_attempt > 1 {
            fields.push(field("Attempt", format!("#{}", run.run_attempt), true));
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
        
        // Commit message
        let commit_msg = if run.head_commit.message.len() > 100 {
            format!("{}...", &run.head_commit.message[..97])
        } else {
            run.head_commit.message.clone()
        };
        fields.push(field("Commit Message", commit_msg, false));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Actor (who triggered it)
        if let Some(actor) = &run.actor {
            fields.push(field("Triggered By", &actor.login, true));
        }
        
        let description = Some(format!(
            "Workflow '{}' {} for commit {} on branch {}",
            workflow.name, status_text, short_sha, run.head_branch
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(run.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Actions".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}