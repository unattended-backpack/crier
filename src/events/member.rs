use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MemberEvent {
    pub action: MemberAction,
    pub member: User,
    pub repository: Repository,
    pub sender: User,
    pub changes: Option<MemberChanges>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemberAction {
    Added,
    Removed,
    Edited,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MemberChanges {
    pub permission: Option<PermissionChange>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PermissionChange {
    pub from: String,
    pub to: Option<String>,
}

impl DiscordTransform for MemberEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            MemberAction::Added => (Colors::DULL_GREEN, "➕", "added as collaborator"),
            MemberAction::Removed => (Colors::DULL_RED, "➖", "removed as collaborator"),
            MemberAction::Edited => (Colors::DULL_BLUE, "✏️", "permissions changed"),
        };
        
        let title = format!("{} Member {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Member
        fields.push(field(
            "Member",
            format!("[{}]({})", self.member.login, self.member.html_url),
            true
        ));
        
        // Action specific info
        if self.action == MemberAction::Edited {
            if let Some(changes) = &self.changes {
                if let Some(perm_change) = &changes.permission {
                    fields.push(field(
                        "Permission Changed",
                        format!("{} → {}", 
                            perm_change.from, 
                            perm_change.to.as_ref().unwrap_or(&"none".to_string())
                        ),
                        true
                    ));
                }
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Repository visibility
        let visibility_icon = if self.repository.private { "🔒" } else { "🌍" };
        fields.push(field(
            "Visibility",
            format!("{} {}", 
                visibility_icon,
                if self.repository.private { "Private" } else { "Public" }
            ),
            true
        ));
        
        // Repository stats
        fields.push(field("⭐ Stars", self.repository.stargazers_count.to_string(), true));
        fields.push(field("🍴 Forks", self.repository.forks_count.to_string(), true));
        
        let description = Some(format!(
            "{} was {} for repository {}",
            self.member.login, action_text, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!(
                "{}/settings/access",
                self.repository.html_url
            )),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Collaborators".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}