use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WatchEvent {
    pub action: WatchAction,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WatchAction {
    Started,
}

impl DiscordTransform for WatchEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = "👀 Repository watched";
        
        let mut fields = vec![];
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            false
        ));
        
        // Watcher count
        fields.push(field("Total Watchers", format!("👀 {}", self.repository.watchers_count), true));
        
        // Stars for context
        fields.push(field("Stars", format!("⭐ {}", self.repository.stargazers_count), true));
        
        // Repository description
        if let Some(desc) = &self.repository.description {
            if !desc.is_empty() {
                let truncated = if desc.len() > 200 {
                    format!("{}...", &desc[..197])
                } else {
                    desc.clone()
                };
                fields.push(field("Description", truncated, false));
            }
        }
        
        let description = Some(format!(
            "{} started watching {}",
            self.sender.login, self.repository.full_name
        ));
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: Some(self.repository.html_url.clone()),
            color: Colors::DULL_BLUE,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Watch".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}