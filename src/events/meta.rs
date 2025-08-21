use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordFooter};
use super::ping::Webhook;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MetaEvent {
    pub action: MetaAction,
    pub hook_id: i64,
    pub hook: Webhook,
    pub repository: Option<Repository>,
    pub sender: Option<User>,
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetaAction {
    Deleted,
}

impl DiscordTransform for MetaEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = "🗑️ Webhook deleted";
        
        let mut fields = vec![];
        
        // Webhook info
        fields.push(field("Webhook ID", self.hook_id.to_string(), true));
        fields.push(field("Webhook Type", &self.hook.hook_type, true));
        
        // Events it was listening for
        if !self.hook.events.is_empty() {
            let events_list = self.hook.events.iter()
                .take(10)
                .map(|e| format!("`{}`", e))
                .collect::<Vec<_>>()
                .join(", ");
            
            let events_str = if self.hook.events.len() > 10 {
                format!("{} ... and {} more", events_list, self.hook.events.len() - 10)
            } else {
                events_list
            };
            
            fields.push(field("Was Listening For", events_str, false));
        }
        
        // Webhook URL (redacted)
        let webhook_url = &self.hook.config.url;
        let redacted_url = if webhook_url.contains("discord.com") {
            "Discord Webhook (redacted)".to_string()
        } else {
            let url_parts: Vec<&str> = webhook_url.split('/').collect();
            if url_parts.len() > 3 {
                format!("{}//{}/...", url_parts[0], url_parts[2])
            } else {
                "Webhook URL (redacted)".to_string()
            }
        };
        fields.push(code_field("Webhook URL", redacted_url, false));
        
        // Repository if available
        if let Some(repo) = &self.repository {
            fields.push(field(
                "Repository",
                format!("[{}]({})", repo.full_name, repo.html_url),
                true
            ));
        }
        
        // Organization if available
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        let description = Some(
            "This webhook has been deleted and will no longer receive events. This is the final event from this webhook.".to_string()
        );
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: None,
            color: Colors::DULL_RED,
            author: self.sender.as_ref().map(|s| crate::DiscordAuthor {
                name: s.login.clone(),
                url: Some(s.html_url.clone()),
                icon_url: Some(s.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Webhook".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}