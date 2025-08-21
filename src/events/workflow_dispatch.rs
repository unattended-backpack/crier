use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WorkflowDispatchEvent {
    pub inputs: Option<serde_json::Map<String, serde_json::Value>>,
    pub workflow: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for WorkflowDispatchEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = "🚀 Workflow manually triggered";
        
        let mut fields = vec![];
        
        // Workflow path
        fields.push(code_field("Workflow", &self.workflow, false));
        
        // Branch/tag
        let ref_type = if self.ref_.starts_with("refs/heads/") {
            "Branch"
        } else if self.ref_.starts_with("refs/tags/") {
            "Tag"
        } else {
            "Ref"
        };
        
        let ref_name = self.ref_
            .replace("refs/heads/", "")
            .replace("refs/tags/", "");
        
        fields.push(code_field(ref_type, &ref_name, true));
        
        // Inputs if provided
        if let Some(inputs) = &self.inputs {
            if !inputs.is_empty() {
                let inputs_str = inputs.iter()
                    .take(10)
                    .map(|(key, value)| {
                        let val_str = match value {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            serde_json::Value::Number(n) => n.to_string(),
                            _ => value.to_string(),
                        };
                        format!("• `{}`: {}", key, val_str)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                fields.push(field("Inputs", inputs_str, false));
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Triggered by
        fields.push(field("Triggered By", &self.sender.login, true));
        
        let description = Some(format!(
            "{} manually triggered workflow '{}' on {} {}",
            self.sender.login, self.workflow, ref_type.to_lowercase(), ref_name
        ));
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: Some(format!(
                "{}/actions/workflows/{}",
                self.repository.html_url,
                self.workflow.split('/').last().unwrap_or(&self.workflow)
            )),
            color: Colors::BLUE,
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