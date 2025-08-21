use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GitHubAppAuthorizationEvent {
    pub action: GitHubAppAuthorizationAction,
    pub sender: User,
    pub app: App,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GitHubAppAuthorizationAction {
    Revoked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct App {
    pub id: i64,
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: User,
    pub name: String,
    pub description: Option<String>,
    pub external_url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Option<serde_json::Value>,
    pub events: Option<Vec<String>>,
    pub client_id: Option<String>,
}

impl DiscordTransform for GitHubAppAuthorizationEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let app = &self.app;
        
        let (color, emoji, action_text) = match self.action {
            GitHubAppAuthorizationAction::Revoked => (Colors::DULL_RED, "🔐", "GitHub App authorization revoked"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // App name
        fields.push(field("App", &app.name, true));
        
        // App owner
        fields.push(field(
            "App Owner",
            format!("[{}]({})", app.owner.login, app.owner.html_url),
            true
        ));
        
        // User who revoked
        fields.push(field(
            "Revoked By",
            format!("[{}]({})", self.sender.login, self.sender.html_url),
            true
        ));
        
        // App description if available
        if let Some(desc) = &app.description {
            if !desc.is_empty() {
                let truncated = if desc.len() > 300 {
                    format!("{}...", &desc[..297])
                } else {
                    desc.clone()
                };
                fields.push(field("App Description", truncated, false));
            }
        }
        
        // Permissions if available
        if let Some(perms) = &app.permissions {
            if let Some(obj) = perms.as_object() {
                let perm_list = obj.iter()
                    .filter(|(_, v)| v.as_str() == Some("write") || v.as_str() == Some("admin"))
                    .take(5)
                    .map(|(k, v)| format!("• {}: {}", k, v.as_str().unwrap_or("unknown")))
                    .collect::<Vec<_>>();
                
                if !perm_list.is_empty() {
                    fields.push(field(
                        "Permissions Revoked",
                        perm_list.join("\n"),
                        false
                    ));
                }
            }
        }
        
        // Events if available
        if let Some(events) = &app.events {
            if !events.is_empty() {
                let event_list = events.iter()
                    .take(10)
                    .map(|e| format!("`{}`", e))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                let event_field = if events.len() > 10 {
                    format!("{} ... and {} more", event_list, events.len() - 10)
                } else {
                    event_list
                };
                
                fields.push(field("Events Access Revoked", event_field, false));
            }
        }
        
        // Organization if available
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }
        
        // External URL
        if !app.external_url.is_empty() {
            fields.push(field("App URL", &app.external_url, false));
        }
        
        let description = Some(format!(
            "Authorization for GitHub App '{}' has been revoked by {}. The app no longer has access to the account.",
            app.name, self.sender.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(app.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub App Authorization".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}