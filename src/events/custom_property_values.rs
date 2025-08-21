use serde::{Deserialize, Serialize};
use crate::types::{Repository, Organization, User, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CustomPropertyValuesEvent {
    pub action: String,
    pub repository_id: Option<i64>,
    pub custom_property_values: Vec<CustomPropertyValue>,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub sender: User,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CustomPropertyValue {
    pub property_name: String,
    pub value: serde_json::Value,
}

impl DiscordTransform for CustomPropertyValuesEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "updated" => (Colors::DULL_BLUE, "✏️"),
            _ => (Colors::DULL_BLUE, "🏷️"),
        };
        
        let title = format!("{} Custom property values {}", emoji, self.action);
        
        let mut fields = vec![];
        
        if let Some(repo) = &self.repository {
            fields.push(field("Repository", format!("[{}]({})", repo.full_name, repo.html_url), true));
        }
        
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        DiscordEmbed {
            title,
            description: Some(format!("Custom property values were {}", self.action)),
            url: self.repository.as_ref().map(|r| r.html_url.clone()),
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
