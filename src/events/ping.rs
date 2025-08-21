use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PingEvent {
    pub zen: String,
    pub hook_id: i64,
    pub hook: Webhook,
    pub repository: Option<Repository>,
    pub sender: Option<User>,
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Webhook {
    #[serde(rename = "type")]
    pub hook_type: String,
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub events: Vec<String>,
    pub config: WebhookConfig,
    pub updated_at: String,
    pub created_at: String,
    pub url: String,
    pub test_url: Option<String>,
    pub ping_url: Option<String>,
    pub deliveries_url: Option<String>,
    pub last_response: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WebhookConfig {
    pub url: String,
    pub content_type: Option<String>,
    pub insecure_ssl: Option<String>,
    pub secret: Option<String>,
}

impl DiscordTransform for PingEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = "🏓 Webhook ping received";
        
        let mut fields = vec![];
        
        // Zen message
        fields.push(field("Zen", format!("_{}_", self.zen), false));
        
        // Webhook info
        fields.push(field("Webhook ID", self.hook_id.to_string(), true));
        fields.push(field("Webhook Type", &self.hook.hook_type, true));
        
        // Status
        let status = if self.hook.active { "✅ Active" } else { "❌ Inactive" };
        fields.push(field("Status", status, true));
        
        // Events listening for
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
            
            fields.push(field("Subscribed Events", events_str, false));
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
            "Webhook connection test successful! This ping confirms that GitHub can successfully deliver events to your webhook endpoint.".to_string()
        );
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: None,
            color: Colors::DULL_YELLOW,
            author: self.sender.as_ref().map(|s| DiscordAuthor {
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