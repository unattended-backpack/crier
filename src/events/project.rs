use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectEvent {
    pub action: ProjectAction,
    pub project: Project,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub sender: User,
    pub changes: Option<ProjectChanges>,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectAction {
    Created,
    Edited,
    Closed,
    Reopened,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Project {
    pub owner_url: String,
    pub url: String,
    pub html_url: String,
    pub columns_url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub body: Option<String>,
    pub number: i64,
    pub state: String,  // "open" or "closed"
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectChanges {
    pub name: Option<Change<String>>,
    pub body: Option<Change<Option<String>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Change<T> {
    pub from: T,
}

impl DiscordTransform for ProjectEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let project = &self.project;
        
        let (color, emoji, action_text) = match self.action {
            ProjectAction::Created => (Colors::DULL_GREEN, "📋", "created"),
            ProjectAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            ProjectAction::Closed => (Colors::DULL_YELLOW, "📕", "closed"),
            ProjectAction::Reopened => (Colors::DULL_GREEN, "📖", "reopened"),
            ProjectAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };
        
        let title = format!("{} Project {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Project name
        fields.push(field("Project", &project.name, true));
        
        // Project number
        fields.push(field("Number", format!("#{}", project.number), true));
        
        // State
        let state_icon = if project.state == "open" { "📖" } else { "📕" };
        fields.push(field("State", format!("{} {}", state_icon, project.state), true));
        
        // Description if available
        if let Some(body) = &project.body {
            if !body.is_empty() {
                let truncated = if body.len() > 500 {
                    format!("{}...", &body[..497])
                } else {
                    body.clone()
                };
                fields.push(field("Description", truncated, false));
            }
        }
        
        // Changes for edited action
        if self.action == ProjectAction::Edited {
            if let Some(changes) = &self.changes {
                let mut change_items = vec![];
                if let Some(name_change) = &changes.name {
                    change_items.push(format!("Name: {} → {}", name_change.from, project.name));
                }
                if changes.body.is_some() {
                    change_items.push("Description updated".to_string());
                }
                if !change_items.is_empty() {
                    fields.push(field("Changes", change_items.join("\n"), false));
                }
            }
        }
        
        // Context (repo or org)
        if let Some(repo) = &self.repository {
            fields.push(field(
                "Repository",
                format!("[{}]({})", repo.full_name, repo.html_url),
                true
            ));
        } else if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        // Creator
        fields.push(field("Creator", &project.creator.login, true));
        
        let description = Some(format!(
            "Project '{}' was {}{}",
            project.name,
            action_text,
            if let Some(repo) = &self.repository {
                format!(" in {}", repo.full_name)
            } else if let Some(org) = &self.organization {
                format!(" in organization {}", org.login)
            } else {
                String::new()
            }
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(project.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Projects (Classic)".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}