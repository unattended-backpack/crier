use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectColumnEvent {
    pub action: ProjectColumnAction,
    pub project_column: ProjectColumn,
    pub repository: Option<Repository>,
    pub sender: User,
    pub changes: Option<ProjectColumnChanges>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectColumnAction {
    Created,
    Edited,
    Moved,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectColumn {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub project_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub cards_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectColumnChanges {
    pub name: Option<ChangeFrom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChangeFrom {
    pub from: String,
}

impl DiscordTransform for ProjectColumnEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            ProjectColumnAction::Created => (Colors::DULL_GREEN, "➕", "created"),
            ProjectColumnAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            ProjectColumnAction::Moved => (Colors::BLUE, "↔️", "moved"),
            ProjectColumnAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };
        
        let title = format!("{} Project column {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Column name
        fields.push(field("Column", &self.project_column.name, true));
        
        // Column ID
        fields.push(field("Column ID", format!("#{}", self.project_column.id), true));
        
        // Changes if edited
        if self.action == ProjectColumnAction::Edited {
            if let Some(changes) = &self.changes {
                if let Some(name_change) = &changes.name {
                    fields.push(field("Previous Name", &name_change.from, true));
                }
            }
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
            "Project column '{}' {} by {}",
            self.project_column.name,
            action_text,
            self.sender.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.project_column.url.clone()),
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