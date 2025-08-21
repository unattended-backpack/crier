use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PublicEvent {
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for PublicEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let repo = &self.repository;
        
        let title = "🌍 Repository made public";
        
        let mut fields = vec![];
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", repo.full_name, repo.html_url),
            false
        ));
        
        // Stats
        fields.push(field("Stars", format!("⭐ {}", repo.stargazers_count), true));
        fields.push(field("Forks", format!("🍴 {}", repo.forks_count), true));
        fields.push(field("Watchers", format!("👀 {}", repo.watchers_count), true));
        
        // Language
        if let Some(lang) = &repo.language {
            fields.push(field("Language", lang, true));
        }
        
        // Description
        if let Some(desc) = &repo.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Topics
        if let Some(topics) = &repo.topics {
            if !topics.is_empty() {
                let topics_str = topics.iter()
                    .take(10)
                    .map(|t| format!("`{}`", t))
                    .collect::<Vec<_>>()
                    .join(" ");
                fields.push(field("Topics", topics_str, false));
            }
        }
        
        let description = Some(format!(
            "🎉 {} is now publicly accessible! The repository was made public by {}.",
            repo.full_name, self.sender.login
        ));
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: Some(repo.html_url.clone()),
            color: Colors::GREEN,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Repository".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}