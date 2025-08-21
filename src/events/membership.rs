use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::team::Team;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MembershipEvent {
    pub action: MembershipAction,
    pub scope: String,  // "team" or "organization"
    pub member: User,
    pub team: Team,
    pub organization: Organization,
    pub sender: User,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MembershipAction {
    Added,
    Removed,
}

impl DiscordTransform for MembershipEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            MembershipAction::Added => (Colors::DULL_GREEN, "➕", "added to"),
            MembershipAction::Removed => (Colors::DULL_RED, "➖", "removed from"),
        };
        
        let scope_text = if self.scope == "team" {
            format!("team '{}'", self.team.name)
        } else {
            format!("{} organization", self.scope)
        };
        
        let title = format!("{} Member {} {}", emoji, action_text, self.scope);
        
        let mut fields = vec![];
        
        // Member
        fields.push(field(
            "Member",
            format!("[{}]({})", self.member.login, self.member.html_url),
            true
        ));
        
        // Team
        fields.push(field("Team", &self.team.name, true));
        
        // Team description
        if let Some(desc) = &self.team.description {
            if !desc.is_empty() {
                fields.push(field("Team Description", desc, false));
            }
        }
        
        // Team privacy
        let privacy_icon = if self.team.privacy == "secret" { "🔒" } else { "🔓" };
        fields.push(field(
            "Team Privacy",
            format!("{} {}", privacy_icon, self.team.privacy),
            true
        ));
        
        // Team permission level
        fields.push(field("Permission Level", &self.team.permission, true));
        
        // Organization
        fields.push(field("Organization", &self.organization.login, true));
        
        // Parent team if exists
        if let Some(parent) = &self.team.parent {
            fields.push(field("Parent Team", &parent.name, true));
        }
        
        // Member and repo counts if available
        if let Some(member_count) = self.team.members_count {
            fields.push(field("Team Members", member_count.to_string(), true));
        }
        
        if let Some(repo_count) = self.team.repos_count {
            fields.push(field("Team Repositories", repo_count.to_string(), true));
        }
        
        let description = Some(format!(
            "{} was {} {}",
            self.member.login, action_text, scope_text
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.team.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Team Membership".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}