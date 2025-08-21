use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TeamEvent {
    pub action: TeamAction,
    pub team: Team,
    pub repository: Option<Repository>,
    pub sender: User,
    pub organization: Organization,
    pub changes: Option<serde_json::Value>,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TeamAction {
    Created,
    Deleted,
    Edited,
    AddedToRepository,
    RemovedFromRepository,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Team {
    pub name: String,
    pub id: i64,
    pub node_id: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,  // "secret" or "closed"
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    pub permission: String,
    pub parent: Option<Box<Team>>,
    pub members_count: Option<i64>,
    pub repos_count: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub organization: Option<Organization>,
}

impl DiscordTransform for TeamEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let team = &self.team;
        
        let (color, emoji, action_text) = match self.action {
            TeamAction::Created => (Colors::DULL_GREEN, "👥", "created"),
            TeamAction::Deleted => (Colors::DULL_RED, "💀", "deleted"),
            TeamAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            TeamAction::AddedToRepository => (Colors::DULL_GREEN, "➕", "added to repository"),
            TeamAction::RemovedFromRepository => (Colors::DULL_RED, "➖", "removed from repository"),
        };
        
        let title = format!("{} Team {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Team name
        fields.push(field("Team", &team.name, true));
        
        // Organization
        fields.push(field("Organization", &self.organization.login, true));
        
        // Privacy level
        let privacy_icon = if team.privacy == "secret" { "🔒" } else { "🔓" };
        fields.push(field("Privacy", format!("{} {}", privacy_icon, team.privacy), true));
        
        // Permission level
        fields.push(field("Permission", &team.permission, true));
        
        // Description
        if let Some(desc) = &team.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Parent team if exists
        if let Some(parent) = &team.parent {
            fields.push(field("Parent Team", &parent.name, true));
        }
        
        // Member and repo counts if available
        if let Some(member_count) = team.members_count {
            fields.push(field("Members", member_count.to_string(), true));
        }
        
        if let Some(repo_count) = team.repos_count {
            fields.push(field("Repositories", repo_count.to_string(), true));
        }
        
        // Repository context for add/remove actions
        if matches!(self.action, TeamAction::AddedToRepository | TeamAction::RemovedFromRepository) {
            if let Some(repo) = &self.repository {
                fields.push(field(
                    "Repository",
                    format!("[{}]({})", repo.full_name, repo.html_url),
                    false
                ));
            }
        }
        
        // Changes for edited action
        if self.action == TeamAction::Edited {
            if let Some(changes) = &self.changes {
                let mut change_items = vec![];
                if changes.get("name").is_some() {
                    change_items.push("name");
                }
                if changes.get("description").is_some() {
                    change_items.push("description");
                }
                if changes.get("privacy").is_some() {
                    change_items.push("privacy");
                }
                if !change_items.is_empty() {
                    fields.push(field("Changed", change_items.join(", "), true));
                }
            }
        }
        
        let description = Some(format!(
            "Team '{}' was {} in {}",
            team.name, action_text, self.organization.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(team.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Team".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}