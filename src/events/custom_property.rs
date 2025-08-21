use serde::{Deserialize, Serialize};
use crate::types::{Organization, User, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CustomPropertyEvent {
    pub action: String,
    pub definition: Option<serde_json::Value>,
    pub organization: Option<Organization>,
    pub sender: User,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for CustomPropertyEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "created" => (Colors::DULL_GREEN, "➕"),
            "updated" => (Colors::DULL_BLUE, "✏️"),
            "deleted" => (Colors::DULL_RED, "🗑️"),
            _ => (Colors::DULL_BLUE, "🏷️"),
        };
        
        let title = format!("{} Custom property {}", emoji, self.action);
        
        let mut fields = vec![];
        
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        DiscordEmbed {
            title,
            description: Some(format!("Custom property was {}", self.action)),
            url: None,
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Custom Properties".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
