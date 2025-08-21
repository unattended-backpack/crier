use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DeleteEvent {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub ref_type: String,  // "branch" or "tag"
    pub pusher_type: String,  // "user" or "deploy_key"
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for DeleteEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let emoji = if self.ref_type == "branch" { "🗑️" } else { "🏷️" };
        let item_type = if self.ref_type == "branch" { "Branch" } else { "Tag" };
        
        let title = format!("{} {} {} deleted", emoji, item_type, self.ref_);
        
        let mut fields = vec![];
        
        // Reference name and type
        fields.push(code_field(item_type, &self.ref_, true));
        
        // Deleted by type
        fields.push(field("Deleted By", &self.pusher_type, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "{} `{}` was deleted from {}",
            item_type, self.ref_, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.repository.html_url.clone()),
            color: Colors::DULL_RED,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: format!("GitHub {}", item_type),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}