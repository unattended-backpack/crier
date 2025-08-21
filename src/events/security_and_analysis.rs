use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityAndAnalysisEvent {
    pub changes: serde_json::Value,
    pub repository: Repository,
    pub sender: User,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for SecurityAndAnalysisEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = "🔒 Security and analysis settings changed";
        
        let mut fields = vec![];
        
        fields.push(field("Repository", format!("[{}]({})", self.repository.full_name, self.repository.html_url), true));
        
        DiscordEmbed {
            title: title.to_string(),
            description: Some("Security and analysis settings were updated".to_string()),
            url: Some(format!("{}/settings/security_analysis", self.repository.html_url)),
            color: Colors::YELLOW,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Security".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
