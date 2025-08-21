use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DeploymentEvent {
    pub deployment: Deployment,
    pub repository: Repository,
    pub sender: User,
    pub workflow: Option<Workflow>,
    pub workflow_run: Option<WorkflowRun>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Deployment {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub task: String,
    pub payload: serde_json::Value,
    pub original_environment: Option<String>,
    pub environment: String,
    pub description: Option<String>,
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub statuses_url: String,
    pub repository_url: String,
    pub transient_environment: Option<bool>,
    pub production_environment: Option<bool>,
    pub performed_via_github_app: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowRun {
    pub id: i64,
    pub name: Option<String>,
    pub head_branch: Option<String>,
}

impl DiscordTransform for DeploymentEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let deployment = &self.deployment;
        
        let emoji = if deployment.production_environment == Some(true) {
            "🚀"
        } else {
            "📦"
        };
        
        let env_type = if deployment.production_environment == Some(true) {
            " (Production)"
        } else if deployment.transient_environment == Some(true) {
            " (Transient)"
        } else {
            ""
        };
        
        let title = format!("{} Deployment to {}{}", emoji, deployment.environment, env_type);
        
        let mut fields = vec![];
        
        // Environment
        fields.push(field("Environment", &deployment.environment, true));
        
        // Task
        if !deployment.task.is_empty() && deployment.task != "deploy" {
            fields.push(field("Task", &deployment.task, true));
        }
        
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
        
        // Workflow info if available
        if let Some(workflow) = &self.workflow {
            fields.push(field("Workflow", &workflow.name, true));
        }
        
        if let Some(run) = &self.workflow_run {
            if let Some(name) = &run.name {
                fields.push(field("Run", format!("{} (#{})", name, run.id), true));
            }
        }
        
        // Description
        if let Some(desc) = &deployment.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Environment flags
        let mut env_flags = vec![];
        if deployment.production_environment == Some(true) {
            env_flags.push("🔴 Production");
        }
        if deployment.transient_environment == Some(true) {
            env_flags.push("⏰ Transient");
        }
        if !env_flags.is_empty() {
            fields.push(field("Flags", env_flags.join(" "), true));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Deployer
        fields.push(field("Deployed By", &deployment.creator.login, true));
        
        let description = Some(format!(
            "Deploying {} to {} environment",
            short_sha, deployment.environment
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!(
                "{}/deployments/{}",
                self.repository.html_url, deployment.id
            )),
            color: if deployment.production_environment == Some(true) {
                Colors::YELLOW  // Production deployments get yellow for attention
            } else {
                Colors::BLUE
            },
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