use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BranchProtectionConfigurationEvent {
    pub action: String,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for BranchProtectionConfigurationEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "disabled" => (Colors::RED, "⚠️"),
            "enabled" => (Colors::GREEN, "🛡️"),
            _ => (Colors::DULL_BLUE, "🔒"),
        };
        
        let title = format!("{} Branch protection configuration {}", emoji, self.action);
        
        let mut fields = vec![];
        
        fields.push(field("Repository", format!("[{}]({})", self.repository.full_name, self.repository.html_url), true));
        fields.push(field("Action", &self.action, true));
        
        DiscordEmbed {
            title,
            description: Some(format!("Branch protection was {} for {}", self.action, self.repository.full_name)),
            url: Some(format!("{}/settings/branches", self.repository.html_url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Branch Protection".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
