use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecretScanningAlertLocationEvent {
    pub action: String,
    pub alert: serde_json::Value,
    pub location: serde_json::Value,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for SecretScanningAlertLocationEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "created" => (Colors::RED, "🚨"),
            _ => (Colors::YELLOW, "⚠️"),
        };
        
        let title = format!("{} Secret scanning alert location {}", emoji, self.action);
        
        let mut fields = vec![];
        
        fields.push(field("Repository", format!("[{}]({})", self.repository.full_name, self.repository.html_url), true));
        fields.push(field("Action", &self.action, true));
        
        DiscordEmbed {
            title,
            description: Some(format!("New location found for secret scanning alert")),
            url: Some(format!("{}/security/secret-scanning", self.repository.html_url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Secret Scanning".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
