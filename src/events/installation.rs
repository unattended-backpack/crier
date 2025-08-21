use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstallationEvent {
    pub action: InstallationAction,
    pub installation: Installation,
    pub repositories: Option<Vec<InstallationRepository>>,
    pub requester: Option<User>,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstallationAction {
    Created,
    Deleted,
    Suspend,
    Unsuspend,
    NewPermissionsAccepted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstallationRepository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
}

impl DiscordTransform for InstallationEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let installation = &self.installation;
        
        let (color, emoji, action_text) = match self.action {
            InstallationAction::Created => (Colors::GREEN, "🎉", "GitHub App installed"),
            InstallationAction::Deleted => (Colors::RED, "💀", "GitHub App uninstalled"),
            InstallationAction::Suspend => (Colors::YELLOW, "⏸️", "GitHub App suspended"),
            InstallationAction::Unsuspend => (Colors::DULL_GREEN, "▶️", "GitHub App unsuspended"),
            InstallationAction::NewPermissionsAccepted => (Colors::DULL_GREEN, "✅", "New permissions accepted"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Installation ID
        fields.push(field("Installation ID", installation.id.to_string(), true));
        
        // Account (user or org)
        fields.push(field(
            "Account",
            format!("[{}]({})", installation.account.login, installation.account.html_url),
            true
        ));
        
        // Account type
        let account_type_icon = match installation.account.account_type.as_str() {
            "User" => "👤",
            "Organization" => "🏢",
            _ => "❓",
        };
        fields.push(field(
            "Account Type",
            format!("{} {}", account_type_icon, installation.account.account_type),
            true
        ));
        
        // Repository selection
        let repo_selection = match installation.repository_selection.as_deref() {
            Some("all") => "📚 All repositories",
            Some("selected") => "📂 Selected repositories",
            _ => "Unknown",
        };
        fields.push(field("Access", repo_selection, true));
        
        // Repositories if provided
        if let Some(repos) = &self.repositories {
            if !repos.is_empty() {
                let repo_list = repos.iter()
                    .take(10)
                    .map(|r| {
                        let visibility = if r.private { "🔒" } else { "🌍" };
                        format!("{} {}", visibility, r.name)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let repo_field = if repos.len() > 10 {
                    format!("{}\n... and {} more", repo_list, repos.len() - 10)
                } else {
                    repo_list
                };
                
                fields.push(field("Repositories", repo_field, false));
            }
        }
        
        // Permissions if available
        if let Some(perms) = &installation.permissions {
            let perm_list = perms.as_object()
                .map(|obj| {
                    obj.iter()
                        .filter(|(_, v)| v.as_str() == Some("write") || v.as_str() == Some("admin"))
                        .take(5)
                        .map(|(k, v)| format!("• {}: {}", k, v.as_str().unwrap_or("unknown")))
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_default();
            
            if !perm_list.is_empty() {
                fields.push(field("Key Permissions", perm_list, false));
            }
        }
        
        // Events if available
        if let Some(events) = &installation.events {
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
                
                fields.push(field("Subscribed Events", event_field, false));
            }
        }
        
        // Requester if different from sender
        if let Some(requester) = &self.requester {
            if requester.login != self.sender.login {
                fields.push(field("Requested By", &requester.login, true));
            }
        }
        
        // App info
        if let Some(app_id) = installation.app_id {
            fields.push(field("App ID", app_id.to_string(), true));
        }
        
        if let Some(app_slug) = &installation.app_slug {
            fields.push(field("App", app_slug, true));
        }
        
        let description = Some(format!(
            "GitHub App {} for account {}",
            match self.action {
                InstallationAction::Created => "has been installed",
                InstallationAction::Deleted => "has been uninstalled",
                InstallationAction::Suspend => "has been suspended",
                InstallationAction::Unsuspend => "has been unsuspended",
                InstallationAction::NewPermissionsAccepted => "permissions have been updated",
            },
            installation.account.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(installation.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Apps".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}