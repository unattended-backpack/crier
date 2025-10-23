use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectsV2ItemEvent {
    pub action: ProjectsV2ItemAction,
    pub projects_v2_item: ProjectV2Item,
    pub organization: Option<Organization>,
    pub sender: User,
    pub changes: Option<ProjectV2ItemChanges>,

    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectsV2ItemAction {
    Created,
    Edited,
    Deleted,
    Archived,
    Restored,
    Converted,
    Reordered,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2Item {
    pub id: i64,
    pub node_id: String,
    pub project_node_id: String,
    pub content_node_id: Option<String>,
    pub content_type: Option<String>, // "Issue", "PullRequest", "DraftIssue"
    pub creator: User,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub content: Option<ProjectV2ItemContent>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2ItemContent {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub number: Option<i64>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub state: Option<String>,
    pub user: Option<User>,
    #[serde(rename = "type")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2ItemChanges {
    pub field_value: Option<FieldValueChange>,
    pub archived_at: Option<ProjectV2ItemChange<Option<String>>>,
    pub body: Option<ProjectV2ItemChange<Option<String>>>,
    pub title: Option<ProjectV2ItemChange<Option<String>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FieldValueChange {
    pub field_node_id: String,
    pub field_type: String,
    pub field_name: Option<String>,
    pub project_number: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2ItemChange<T> {
    pub from: T,
    pub to: Option<T>,
}

impl DiscordTransform for ProjectsV2ItemEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let item = &self.projects_v2_item;

        let (color, emoji, action_text) = match self.action {
            ProjectsV2ItemAction::Created => (Colors::DULL_GREEN, "➕", "added to project"),
            ProjectsV2ItemAction::Edited => (Colors::DULL_BLUE, "✏️", "edited in project"),
            ProjectsV2ItemAction::Deleted => (Colors::DULL_RED, "❌", "removed from project"),
            ProjectsV2ItemAction::Archived => (Colors::DULL_YELLOW, "📦", "archived in project"),
            ProjectsV2ItemAction::Restored => (Colors::DULL_GREEN, "♻️", "restored in project"),
            ProjectsV2ItemAction::Converted => (Colors::DULL_BLUE, "🔄", "converted in project"),
            ProjectsV2ItemAction::Reordered => (Colors::GRAY, "↕️", "reordered in project"),
        };

        let title = format!("{} Project Item {}", emoji, action_text);

        let mut fields = vec![];

        // Content type and details
        if let Some(content_type) = &item.content_type {
            let type_icon = match content_type.as_str() {
                "Issue" => "🐛",
                "PullRequest" => "🔀",
                "DraftIssue" => "📝",
                _ => "📋",
            };
            fields.push(field("Item Type", format!("{} {}", type_icon, content_type), true));
        }

        // Content details (issue/PR title and number)
        if let Some(content) = &item.content {
            if let Some(number) = content.number {
                fields.push(field("Number", format!("#{}", number), true));
            }

            if let Some(state) = &content.state {
                let state_icon = match state.as_str() {
                    "open" => "🟢",
                    "closed" => "🔴",
                    "merged" => "🟣",
                    _ => "⚪",
                };
                fields.push(field("State", format!("{} {}", state_icon, state), true));
            }

            if let Some(title) = &content.title {
                let truncated = if title.len() > 200 {
                    format!("{}...", &title[..197])
                } else {
                    title.clone()
                };
                fields.push(field("Title", truncated, false));
            }

            if let Some(body) = &content.body {
                if !body.is_empty() {
                    let truncated = if body.len() > 300 {
                        format!("{}...", &body[..297])
                    } else {
                        body.clone()
                    };
                    fields.push(field("Description", truncated, false));
                }
            }

            if let Some(user) = &content.user {
                fields.push(field("Author", format!("@{}", user.login), true));
            }
        }

        // Archived status
        if item.archived_at.is_some() {
            fields.push(field("Status", "📦 Archived", true));
        }

        // Changes for edited action
        if self.action == ProjectsV2ItemAction::Edited {
            if let Some(changes) = &self.changes {
                let mut change_items = vec![];

                if let Some(title_change) = &changes.title {
                    let from = title_change.from.as_deref().unwrap_or("(none)");
                    let to = title_change.to.as_ref().and_then(|t| t.as_deref()).unwrap_or("(none)");
                    change_items.push(format!("**Title:** {} → {}",
                        if from.len() > 50 { format!("{}...", &from[..47]) } else { from.to_string() },
                        if to.len() > 50 { format!("{}...", &to[..47]) } else { to.to_string() }
                    ));
                }

                if let Some(body_change) = &changes.body {
                    let from = body_change.from.as_deref().unwrap_or("(none)");
                    let to = body_change.to.as_ref().and_then(|t| t.as_deref()).unwrap_or("(none)");
                    change_items.push(format!("**Body:** {} → {}",
                        if from.len() > 50 { format!("{}...", &from[..47]) } else { from.to_string() },
                        if to.len() > 50 { format!("{}...", &to[..47]) } else { to.to_string() }
                    ));
                }

                if let Some(field_value) = &changes.field_value {
                    let field_name = field_value.field_name.as_deref().unwrap_or("Unknown field");
                    change_items.push(format!("**{}** updated ({})", field_name, field_value.field_type));
                }

                if let Some(archived_change) = &changes.archived_at {
                    let was_archived = archived_change.from.is_some();
                    let is_archived = archived_change.to.as_ref().and_then(|t| t.as_ref()).is_some();

                    match (was_archived, is_archived) {
                        (false, true) => change_items.push("**Archived**".to_string()),
                        (true, false) => change_items.push("**Restored from archive**".to_string()),
                        _ => {}
                    }
                }

                if !change_items.is_empty() {
                    fields.push(field("Changes", change_items.join("\n"), false));
                }
            }
        }

        // Creator
        fields.push(field("Added By", &item.creator.login, true));

        // Organization context
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }

        let description = if let Some(content) = &item.content {
            if let (Some(_title), Some(number)) = (&content.title, content.number) {
                Some(format!(
                    "{} #{} was {}{}",
                    item.content_type.as_deref().unwrap_or("Item"),
                    number,
                    action_text,
                    if let Some(org) = &self.organization {
                        format!(" in {}'s project", org.login)
                    } else {
                        String::new()
                    }
                ))
            } else {
                Some(format!(
                    "Project item was {}{}",
                    action_text,
                    if let Some(org) = &self.organization {
                        format!(" in {}'s project", org.login)
                    } else {
                        String::new()
                    }
                ))
            }
        } else {
            Some(format!(
                "Draft item was {}{}",
                action_text,
                if let Some(org) = &self.organization {
                    format!(" in {}'s project", org.login)
                } else {
                    String::new()
                }
            ))
        };

        // Use the content URL if available
        let url = item.content.as_ref()
            .and_then(|c| c.html_url.clone())
            .or_else(|| item.content.as_ref().and_then(|c| c.url.clone()));

        DiscordEmbed {
            title,
            description,
            url,
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Projects V2 Item".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
