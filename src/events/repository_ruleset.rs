use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryRulesetEvent {
    pub action: String,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub sender: User,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for RepositoryRulesetEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "created" => (Colors::GREEN, "➕"),
            "edited" => (Colors::DULL_BLUE, "✏️"),
            "deleted" => (Colors::RED, "🗑️"),
            _ => (Colors::DULL_BLUE, "📋"),
        };
        
        let title = format!("{} Repository ruleset {}", emoji, self.action);
        
        let mut fields = vec![];
        
        if let Some(repo) = &self.repository {
            fields.push(field("Repository", format!("[{}]({})", repo.full_name, repo.html_url), true));
        }
        
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        DiscordEmbed {
            title,
            description: Some(format!("Repository ruleset was {}", self.action)),
            url: self.repository.as_ref().map(|r| format!("{}/settings/rules", r.html_url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Repository Rules".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
