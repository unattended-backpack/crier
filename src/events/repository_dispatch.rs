use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryDispatchEvent {
    pub action: String,  // Custom event type
    pub branch: String,
    pub client_payload: Option<serde_json::Value>,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for RepositoryDispatchEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = format!("🎯 Custom event: {}", self.action);
        
        let mut fields = vec![];
        
        // Event type
        fields.push(field("Event Type", &self.action, true));
        
        // Branch
        fields.push(field("Branch", &self.branch, true));
        
        // Client payload if provided
        if let Some(payload) = &self.client_payload {
            if !payload.is_null() {
                let payload_str = if let Some(obj) = payload.as_object() {
                    // Format as key-value pairs
                    obj.iter()
                        .take(10)
                        .map(|(k, v)| {
                            let val_str = match v {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Null => "null".to_string(),
                                _ => format!("{} items", if v.is_array() { 
                                    v.as_array().unwrap().len() 
                                } else { 
                                    v.as_object().map_or(0, |o| o.len()) 
                                }),
                            };
                            format!("• `{}`: {}", k, val_str)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    serde_json::to_string_pretty(payload).unwrap_or_else(|_| "Invalid JSON".to_string())
                };
                
                if !payload_str.is_empty() {
                    fields.push(field("Payload", payload_str, false));
                }
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
            "Custom repository dispatch event '{}' triggered on branch {}",
            self.action, self.branch
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.repository.html_url.clone()),
            color: Colors::BLUE,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Repository Dispatch".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}