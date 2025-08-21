use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PersonalAccessTokenRequestEvent {
    pub action: String,
    pub personal_access_token_request: PersonalAccessTokenRequest,
    pub organization: Organization,
    pub sender: User,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PersonalAccessTokenRequest {
    pub id: i64,
    pub owner: User,
    pub permissions_added: Option<serde_json::Value>,
    pub permissions_upgraded: Option<serde_json::Value>,
    pub permissions_result: Option<serde_json::Value>,
    pub repository_selection: String,
    pub repository_count: Option<i64>,
    pub repositories: Option<Vec<Repository>>,
    pub created_at: String,
    pub token_expired: bool,
    pub token_expires_at: Option<String>,
    pub token_last_used_at: Option<String>,
}

impl DiscordTransform for PersonalAccessTokenRequestEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let color = match self.action.as_str() {
            "approved" => Colors::GREEN,
            "denied" => Colors::RED,
            "created" => Colors::DULL_BLUE,
            _ => Colors::DULL_BLUE,
        };
        
        let emoji = match self.action.as_str() {
            "approved" => "✅",
            "denied" => "❌",
            "created" => "🔑",
            _ => "🔑",
        };
        
        let title = format!("{} Personal access token request {}", emoji, self.action);
        
        let mut fields = vec![];
        
        fields.push(field("Owner", &self.personal_access_token_request.owner.login, true));
        fields.push(field("Organization", &self.organization.login, true));
        fields.push(field("Repository Selection", &self.personal_access_token_request.repository_selection, true));
        
        if let Some(count) = self.personal_access_token_request.repository_count {
            fields.push(field("Repository Count", count.to_string(), true));
        }
        
        if self.personal_access_token_request.token_expired {
            fields.push(field("Status", "⚠️ Expired", true));
        }
        
        DiscordEmbed {
            title,
            description: Some(format!("PAT request {} for {}", self.action, self.personal_access_token_request.owner.login)),
            url: self.organization.html_url.as_ref()
                .map(|url| format!("{}/settings/personal-access-tokens/requests", url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub PAT Request".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
