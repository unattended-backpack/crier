use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::deployment::{Deployment, Workflow, WorkflowRun};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DeploymentStatusEvent {
    pub deployment_status: DeploymentStatus,
    pub deployment: Deployment,
    pub repository: Repository,
    pub sender: User,
    pub workflow: Option<Workflow>,
    pub workflow_run: Option<WorkflowRun>,
    pub check_run: Option<serde_json::Value>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DeploymentStatus {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub state: String,
    pub creator: User,
    pub description: Option<String>,
    pub environment: Option<String>,
    pub deployment_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub target_url: Option<String>,
    pub log_url: Option<String>,
    pub environment_url: Option<String>,
    pub performed_via_github_app: Option<serde_json::Value>,
}

impl DiscordTransform for DeploymentStatusEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let status = &self.deployment_status;
        let deployment = &self.deployment;
        
        let (color, emoji, state_text) = match status.state.as_str() {
            "error" => (Colors::RED, "🚨", "Error"),
            "failure" => (Colors::RED, "❌", "Failed"),
            "inactive" => (Colors::GRAY, "💤", "Inactive"),
            "in_progress" => (Colors::YELLOW, "⚡", "In Progress"),
            "queued" => (Colors::DULL_BLUE, "📋", "Queued"),
            "pending" => (Colors::YELLOW, "⏳", "Pending"),
            "success" => (Colors::GREEN, "✅", "Success"),
            "waiting" => (Colors::DULL_YELLOW, "⏸️", "Waiting"),
            _ => (Colors::GRAY, "❓", status.state.as_str()),
        };
        
        let env_type = if deployment.production_environment == Some(true) {
            " (Production)"
        } else {
            ""
        };
        
        let title = format!(
            "{} Deployment Status: {}{}",
            emoji, state_text, env_type
        );
        
        let mut fields = vec![];
        
        // Environment
        let environment = status.environment.as_ref()
            .unwrap_or(&deployment.environment);
        fields.push(field("Environment", environment, true));
        
        // Status
        fields.push(field("Status", state_text, true));
        
        // Ref (branch/tag)
        let ref_name = deployment.ref_
            .replace("refs/heads/", "")
            .replace("refs/tags/", "");
        fields.push(code_field("Ref", &ref_name, true));
        
        // Commit SHA
        let short_sha = if deployment.sha.len() >= 7 {
            &deployment.sha[..7]
        } else {
            &deployment.sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Description
        if let Some(desc) = &status.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // URLs
        if let Some(url) = &status.target_url {
            fields.push(field("Target URL", format!("[View Deployment]({})", url), true));
        }
        
        if let Some(url) = &status.environment_url {
            fields.push(field("Environment URL", format!("[View Environment]({})", url), true));
        }
        
        // Environment flags
        if deployment.production_environment == Some(true) {
            fields.push(field("Type", "🔴 Production", true));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Creator
        fields.push(field("Updated By", &status.creator.login, true));
        
        let description = Some(format!(
            "Deployment to {} is now {}",
            environment, state_text.to_lowercase()
        ));
        
        DiscordEmbed {
            title,
            description,
            url: status.target_url.clone().or_else(|| Some(format!(
                "{}/deployments/{}",
                self.repository.html_url, deployment.id
            ))),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Deployments".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}