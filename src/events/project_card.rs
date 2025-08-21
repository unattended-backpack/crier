use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectCardEvent {
    pub action: ProjectCardAction,
    pub project_card: ProjectCard,
    pub repository: Option<Repository>,
    pub sender: User,
    pub changes: Option<ProjectCardChanges>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectCardAction {
    Created,
    Edited,
    Moved,
    Converted,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectCard {
    pub id: i64,
    pub node_id: String,
    pub note: Option<String>,
    pub archived: bool,
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub project_url: String,
    pub column_url: String,
    pub url: String,
    pub content_url: Option<String>,
    pub column_id: i64,
    pub after_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectCardChanges {
    pub note: Option<ChangeFrom>,
    pub column_id: Option<ChangeFromI64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChangeFrom {
    pub from: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChangeFromI64 {
    pub from: i64,
}

impl DiscordTransform for ProjectCardEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            ProjectCardAction::Created => (Colors::DULL_GREEN, "➕", "created"),
            ProjectCardAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            ProjectCardAction::Moved => (Colors::BLUE, "↔️", "moved"),
            ProjectCardAction::Converted => (Colors::BLUE, "🔄", "converted"),
            ProjectCardAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };
        
        let title = format!("{} Project card {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Card ID
        fields.push(field("Card ID", format!("#{}", self.project_card.id), true));
        
        // Note if present
        if let Some(note) = &self.project_card.note {
            fields.push(field("Note", note, false));
        }
        
        // Changes if edited
        if self.action == ProjectCardAction::Edited {
            if let Some(changes) = &self.changes {
                if let Some(note_change) = &changes.note {
                    fields.push(field("Previous Note", &note_change.from, false));
                }
                if let Some(column_change) = &changes.column_id {
                    fields.push(field("Moved From Column", format!("#{}", column_change.from), true));
                }
            }
        }
        
        // Archived status
        if self.project_card.archived {
            fields.push(field("Status", "🗂️ Archived", true));
        }
        
        // Repository if present
        if let Some(repo) = &self.repository {
            fields.push(field(
                "Repository",
                format!("[{}]({})", repo.full_name, repo.html_url),
                true
            ));
        }
        
        let description = Some(format!(
            "Project card {} by {}",
            action_text,
            self.sender.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.project_card.url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Projects".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}