use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateEvent {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub ref_type: String,  // "branch" or "tag"
    pub pusher_type: String,  // "user" or "deploy_key"
    pub repository: Repository,
    pub sender: User,
    pub master_branch: String,
    pub description: Option<String>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for CreateEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let emoji = if self.ref_type == "branch" { "🌿" } else { "🏷️" };
        let item_type = if self.ref_type == "branch" { "Branch" } else { "Tag" };
        
        let title = format!("{} {} {} created", emoji, item_type, self.ref_);
        
        let mut fields = vec![];
        
        // Reference name and type
        fields.push(code_field(item_type, &self.ref_, true));
        
        // Default branch for context
        if self.ref_type == "branch" && self.master_branch != self.ref_ {
            fields.push(code_field("Default Branch", &self.master_branch, true));
        }
        
        // Pusher type
        fields.push(field("Created By", &self.pusher_type, true));
        
        // Description if provided
        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "{} `{}` was created in {}",
            item_type, self.ref_, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!(
                "{}/tree/{}",
                self.repository.html_url,
                urlencoding::encode(&self.ref_)
            )),
            color: Colors::DULL_GREEN,
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