use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectsV2Event {
    pub action: ProjectsV2Action,
    pub projects_v2: ProjectV2,
    pub organization: Option<Organization>,
    pub sender: User,
    pub changes: Option<ProjectV2Changes>,

    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectsV2Action {
    Created,
    Edited,
    Deleted,
    Closed,
    Reopened,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2 {
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub public: bool,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub deleted_at: Option<String>,
    pub creator: User,
    pub deleted_by: Option<User>,
    pub owner: ProjectV2Owner,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2Owner {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub owner_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2Changes {
    pub title: Option<ProjectV2Change<String>>,
    pub short_description: Option<ProjectV2Change<Option<String>>>,
    pub description: Option<ProjectV2Change<Option<String>>>,
    pub public: Option<ProjectV2Change<bool>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProjectV2Change<T> {
    pub from: T,
    pub to: Option<T>,
}

impl DiscordTransform for ProjectsV2Event {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let project = &self.projects_v2;

        let (color, emoji, action_text) = match self.action {
            ProjectsV2Action::Created => (Colors::DULL_GREEN, "📋", "created"),
            ProjectsV2Action::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            ProjectsV2Action::Closed => (Colors::DULL_YELLOW, "📕", "closed"),
            ProjectsV2Action::Reopened => (Colors::DULL_GREEN, "📖", "reopened"),
            ProjectsV2Action::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };

        let title = format!("{} Project V2 {}", emoji, action_text);

        let mut fields = vec![];

        // Project title
        fields.push(field("Project", &project.title, true));

        // Project number
        fields.push(field("Number", format!("#{}", project.number), true));

        // Visibility
        let visibility = if project.public { "🌐 Public" } else { "🔒 Private" };
        fields.push(field("Visibility", visibility, true));

        // Short description if available
        if let Some(short_desc) = &project.short_description {
            if !short_desc.is_empty() {
                let truncated = if short_desc.len() > 300 {
                    format!("{}...", &short_desc[..297])
                } else {
                    short_desc.clone()
                };
                fields.push(field("Short Description", truncated, false));
            }
        }

        // Full description if available
        if let Some(desc) = &project.description {
            if !desc.is_empty() {
                let truncated = if desc.len() > 500 {
                    format!("{}...", &desc[..497])
                } else {
                    desc.clone()
                };
                fields.push(field("Description", truncated, false));
            }
        }

        // Changes for edited action
        if self.action == ProjectsV2Action::Edited {
            if let Some(changes) = &self.changes {
                let mut change_items = vec![];

                if let Some(title_change) = &changes.title {
                    change_items.push(format!("**Title:** {} → {}",
                        title_change.from,
                        title_change.to.as_ref().unwrap_or(&project.title)));
                }

                if let Some(short_desc_change) = &changes.short_description {
                    let from = short_desc_change.from.as_ref().map(|s| s.as_str()).unwrap_or("(none)");
                    let to = short_desc_change.to.as_ref()
                        .and_then(|opt| opt.as_ref())
                        .map(|s| s.as_str())
                        .unwrap_or("(none)");
                    if from != to {
                        change_items.push(format!("**Short Description:** {} → {}",
                            if from.len() > 50 { format!("{}...", &from[..47]) } else { from.to_string() },
                            if to.len() > 50 { format!("{}...", &to[..47]) } else { to.to_string() }
                        ));
                    }
                }

                if let Some(_desc_change) = &changes.description {
                    change_items.push("**Description** updated".to_string());
                }

                if let Some(public_change) = &changes.public {
                    let from_vis = if public_change.from { "Public" } else { "Private" };
                    let to_vis = if public_change.to.unwrap_or(project.public) { "Public" } else { "Private" };
                    change_items.push(format!("**Visibility:** {} → {}", from_vis, to_vis));
                }

                if !change_items.is_empty() {
                    fields.push(field("Changes", change_items.join("\n"), false));
                }
            }
        }

        // Owner (organization or user)
        fields.push(field(
            if project.owner.owner_type == "Organization" { "Organization" } else { "Owner" },
            &project.owner.login,
            true
        ));

        // Creator
        fields.push(field("Creator", &project.creator.login, true));

        // Status
        if project.closed_at.is_some() {
            fields.push(field("Status", "📕 Closed", true));
        } else if project.deleted_at.is_some() {
            fields.push(field("Status", "🗑️ Deleted", true));
        }

        let description = Some(format!(
            "Project '{}' was {}{}",
            project.title,
            action_text,
            if let Some(org) = &self.organization {
                format!(" in organization {}", org.login)
            } else {
                String::new()
            }
        ));

        DiscordEmbed {
            title,
            description,
            url: None, // Projects V2 don't have direct HTML URLs in the webhook payload
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Projects V2".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
