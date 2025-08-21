use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryEvent {
    pub action: RepositoryAction,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub changes: Option<serde_json::Value>,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAction {
    Created,
    Deleted,
    Archived,
    Unarchived,
    Edited,
    Renamed,
    Transferred,
    Publicized,
    Privatized,
}

impl DiscordTransform for RepositoryEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let repo = &self.repository;
        
        let (color, emoji, action_text) = match self.action {
            RepositoryAction::Created => (Colors::DULL_GREEN, "🎉", "created"),
            RepositoryAction::Deleted => (Colors::RED, "💀", "deleted"),
            RepositoryAction::Archived => (Colors::YELLOW, "📦", "archived"),
            RepositoryAction::Unarchived => (Colors::DULL_GREEN, "📂", "unarchived"),
            RepositoryAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            RepositoryAction::Renamed => (Colors::DULL_BLUE, "🏷️", "renamed"),
            RepositoryAction::Transferred => (Colors::BLUE, "➡️", "transferred"),
            RepositoryAction::Publicized => (Colors::GREEN, "🌍", "made public"),
            RepositoryAction::Privatized => (Colors::YELLOW, "🔒", "made private"),
        };
        
        let title = format!("{} Repository {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Repository name
        fields.push(field("Repository", &repo.full_name, false));
        
        // Visibility
        let visibility = if repo.private { "🔒 Private" } else { "🌍 Public" };
        fields.push(field("Visibility", visibility, true));
        
        // Stats
        fields.push(field("Stars", format!("⭐ {}", repo.stargazers_count), true));
        fields.push(field("Forks", format!("🍴 {}", repo.forks_count), true));
        
        // Language
        if let Some(lang) = &repo.language {
            fields.push(field("Language", lang, true));
        }
        
        // Default branch
        fields.push(code_field("Default Branch", &repo.default_branch, true));
        
        // Archive status
        if repo.archived {
            fields.push(field("Status", "📦 Archived", true));
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
        
        // License
        if let Some(license) = &repo.license {
            fields.push(field("License", &license.name, true));
        }
        
        // Owner (for transfers)
        if self.action == RepositoryAction::Transferred {
            fields.push(field("New Owner", &repo.owner.login, true));
        }
        
        let description = Some(format!(
            "Repository {} was {} by {}",
            repo.full_name, action_text, self.sender.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(repo.html_url.clone()),
            color,
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