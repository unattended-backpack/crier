use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowJobEvent {
    pub action: WorkflowJobAction,
    pub workflow_job: WorkflowJob,
    pub repository: Repository,
    pub sender: User,
    pub deployment: Option<serde_json::Value>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowJobAction {
    Queued,
    InProgress,
    Completed,
    Waiting,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowJob {
    pub id: i64,
    pub run_id: i64,
    pub workflow_name: Option<String>,
    pub head_branch: Option<String>,
    pub run_url: String,
    pub run_attempt: i64,
    pub node_id: String,
    pub head_sha: String,
    pub url: String,
    pub html_url: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub name: String,
    pub steps: Option<Vec<JobStep>>,
    pub check_run_url: String,
    pub labels: Vec<String>,
    pub runner_id: Option<i64>,
    pub runner_name: Option<String>,
    pub runner_group_id: Option<i64>,
    pub runner_group_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct JobStep {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub number: i64,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

impl DiscordTransform for WorkflowJobEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let job = &self.workflow_job;
        
        let (color, emoji, status_text) = match self.action {
            WorkflowJobAction::Queued => (Colors::DULL_BLUE, "📋", "queued"),
            WorkflowJobAction::Waiting => (Colors::DULL_YELLOW, "⏳", "waiting"),
            WorkflowJobAction::InProgress => (Colors::YELLOW, "⚡", "in progress"),
            WorkflowJobAction::Completed => {
                match job.conclusion.as_deref() {
                    Some("success") => (Colors::GREEN, "✅", "completed successfully"),
                    Some("failure") => (Colors::RED, "❌", "failed"),
                    Some("cancelled") => (Colors::DULL_YELLOW, "🚫", "cancelled"),
                    Some("skipped") => (Colors::GRAY, "⏭️", "skipped"),
                    _ => (Colors::GRAY, "❓", "completed"),
                }
            }
        };
        
        let title = format!("{} Job: {}", emoji, job.name);
        
        let mut fields = vec![];
        
        // Job name and status
        fields.push(field("Status", status_text, true));
        
        // Run ID
        fields.push(field("Run ID", format!("#{}", job.run_id), true));
        
        // Attempt number
        if job.run_attempt > 1 {
            fields.push(field("Attempt", format!("#{}", job.run_attempt), true));
        }
        
        // Workflow name if available
        if let Some(workflow_name) = &job.workflow_name {
            fields.push(field("Workflow", workflow_name, true));
        }
        
        // Branch
        if let Some(branch) = &job.head_branch {
            fields.push(code_field("Branch", branch, true));
        }
        
        // Commit
        let short_sha = if job.head_sha.len() >= 7 {
            &job.head_sha[..7]
        } else {
            &job.head_sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Runner info
        if let Some(runner_name) = &job.runner_name {
            let runner_info = if let Some(group_name) = &job.runner_group_name {
                format!("{} ({})", runner_name, group_name)
            } else {
                runner_name.clone()
            };
            fields.push(field("Runner", runner_info, true));
        }
        
        // Labels
        if !job.labels.is_empty() {
            let labels_str = job.labels.iter()
                .take(5)
                .map(|l| format!("`{}`", l))
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("Labels", labels_str, false));
        }
        
        // Steps summary (if completed and has steps)
        if self.action == WorkflowJobAction::Completed {
            if let Some(steps) = &job.steps {
                if !steps.is_empty() {
                    let failed_steps: Vec<_> = steps.iter()
                        .filter(|s| s.conclusion.as_deref() == Some("failure"))
                        .collect();
                    
                    if !failed_steps.is_empty() {
                        let failed_names = failed_steps.iter()
                            .take(3)
                            .map(|s| format!("• {}", s.name))
                            .collect::<Vec<_>>()
                            .join("\n");
                        fields.push(field("Failed Steps", failed_names, false));
                    }
                    
                    let total = steps.len();
                    let completed = steps.iter()
                        .filter(|s| s.status == "completed")
                        .count();
                    fields.push(field("Steps", format!("{}/{} completed", completed, total), true));
                }
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Job '{}' {} for workflow run #{}",
            job.name, status_text, job.run_id
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(job.html_url.clone()),
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