use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ForkEvent {
    pub forkee: Repository,  // The new fork
    pub repository: Repository,  // The original repository
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for ForkEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let title = format!("🍴 Repository forked");
        
        let mut fields = vec![];
        
        // Original repository
        fields.push(field(
            "Original Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            false
        ));
        
        // New fork
        fields.push(field(
            "New Fork",
            format!("[{}]({})", self.forkee.full_name, self.forkee.html_url),
            false
        ));
        
        // Fork visibility
        let visibility = if self.forkee.private { "Private" } else { "Public" };
        fields.push(field("Visibility", visibility, true));
        
        // Fork owner
        fields.push(field("Fork Owner", &self.forkee.owner.login, true));
        
        // Statistics from original repo
        if self.repository.forks_count > 0 {
            fields.push(field(
                "Total Forks",
                format!("{}", self.repository.forks_count + 1),
                true
            ));
        }
        
        let description = Some(format!(
            "{} forked {} to {}",
            self.sender.login, self.repository.full_name, self.forkee.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.forkee.html_url.clone()),
            color: Colors::DULL_GREEN,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Fork".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}