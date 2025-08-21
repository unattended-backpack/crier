use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrgBlockEvent {
    pub action: OrgBlockAction,
    pub blocked_user: User,
    pub organization: Organization,
    pub sender: User,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrgBlockAction {
    Blocked,
    Unblocked,
}

impl DiscordTransform for OrgBlockEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            OrgBlockAction::Blocked => (Colors::RED, "🚫", "blocked"),
            OrgBlockAction::Unblocked => (Colors::GREEN, "✅", "unblocked"),
        };
        
        let title = format!("{} User {} from organization", emoji, action_text);
        
        let mut fields = vec![];
        
        // Blocked user
        fields.push(field(
            "User",
            format!("[{}]({})", self.blocked_user.login, self.blocked_user.html_url),
            true
        ));
        
        // Organization
        let org_url = self.organization.html_url.as_ref()
            .map(|url| format!("[{}]({})", self.organization.login, url))
            .unwrap_or_else(|| self.organization.login.clone());
        fields.push(field("Organization", org_url, true));
        
        // Action
        fields.push(field("Action", action_text, true));
        
        let description = Some(format!(
            "{} was {} from organization {}",
            self.blocked_user.login,
            action_text,
            self.organization.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: self.organization.html_url.as_ref()
                .map(|url| format!("{}/settings/blocked_users", url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Organization".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}