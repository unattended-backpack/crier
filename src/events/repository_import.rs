use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryImportEvent {
    pub status: String,  // "success" or "failure"
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for RepositoryImportEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, status_text) = match self.status.as_str() {
            "success" => (Colors::GREEN, "✅", "succeeded"),
            "failure" => (Colors::RED, "❌", "failed"),
            _ => (Colors::DULL_BLUE, "📦", self.status.as_str()),
        };
        
        let title = format!("{} Repository import {}", emoji, status_text);
        
        let mut fields = vec![];
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Status
        fields.push(field("Status", &self.status, true));
        
        // Language if present
        if let Some(lang) = &self.repository.language {
            fields.push(field("Language", lang, true));
        }
        
        // Size
        fields.push(field("Size", format!("{} KB", self.repository.size), true));
        
        let description = Some(format!(
            "Import {} for repository {}",
            status_text,
            self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.repository.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Repository Import".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}