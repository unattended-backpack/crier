use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::team::Team;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TeamAddEvent {
    pub team: Team,
    pub repository: Repository,
    pub sender: User,
    pub organization: Organization,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for TeamAddEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let team = &self.team;
        
        let title = "➕ Team added to repository";
        
        let mut fields = vec![];
        
        // Team name
        fields.push(field("Team", &team.name, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Permission level
        fields.push(field("Permission", &team.permission, true));
        
        // Organization
        fields.push(field("Organization", &self.organization.login, true));
        
        // Team description if available
        if let Some(desc) = &team.description {
            if !desc.is_empty() {
                fields.push(field("Team Description", desc, false));
            }
        }
        
        // Team privacy
        let privacy_icon = if team.privacy == "secret" { "🔒" } else { "🔓" };
        fields.push(field(
            "Team Privacy",
            format!("{} {}", privacy_icon, team.privacy),
            true
        ));
        
        // Repository visibility
        let visibility_icon = if self.repository.private { "🔒" } else { "🌍" };
        fields.push(field(
            "Repository Visibility",
            format!("{} {}", 
                visibility_icon,
                if self.repository.private { "Private" } else { "Public" }
            ),
            true
        ));
        
        // Parent team if exists
        if let Some(parent) = &team.parent {
            fields.push(field("Parent Team", &parent.name, true));
        }
        
        // Member and repo counts if available
        if let Some(member_count) = team.members_count {
            fields.push(field("Team Members", member_count.to_string(), true));
        }
        
        if let Some(repo_count) = team.repos_count {
            fields.push(field("Team Repositories", repo_count.to_string(), true));
        }
        
        let description = Some(format!(
            "Team '{}' was granted {} access to repository {}",
            team.name, team.permission, self.repository.full_name
        ));
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: Some(format!(
                "{}/settings/access",
                self.repository.html_url
            )),
            color: Colors::DULL_GREEN,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Team Access".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}