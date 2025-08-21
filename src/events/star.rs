use crate::transform::{field, Colors};
use crate::types::{DiscordTransform, Installation, Organization, Repository, User};
use crate::{DiscordAuthor, DiscordEmbed, DiscordFooter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StarEvent {
    pub action: StarAction,
    pub starred_at: Option<String>,
    pub repository: Repository,
    pub sender: User,

    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}

impl DiscordTransform for StarEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (emoji, action_text, color) = match self.action {
            StarAction::Created => ("⭐", "starred", Colors::GOLD),
            StarAction::Deleted => ("💔", "unstarred", Colors::DULL_RED),
        };

        let title = format!("{} Repository {}", emoji, action_text);

        let mut fields = vec![];

        // Repository info
        fields.push(field(
            "Repository",
            format!(
                "[{}]({})",
                self.repository.full_name, self.repository.html_url
            ),
            false,
        ));

        // Current star count
        let star_count = match self.action {
            StarAction::Created => self.repository.stargazers_count,
            StarAction::Deleted => self.repository.stargazers_count.saturating_sub(1),
        };
        fields.push(field("Total Stars", format!("⭐ {}", star_count), true));

        // Language if available
        if let Some(lang) = &self.repository.language {
            fields.push(field("Language", lang, true));
        }

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

        // Topics if any
        if let Some(topics) = &self.repository.topics {
            if !topics.is_empty() {
                let topics_str = topics
                    .iter()
                    .take(5)
                    .map(|t| format!("`{}`", t))
                    .collect::<Vec<_>>()
                    .join(" ");
                fields.push(field("Topics", topics_str, false));
            }
        }

        let description = Some(format!(
            "{} {} {}",
            self.sender.login, action_text, self.repository.full_name
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
                text: "GitHub Star".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
