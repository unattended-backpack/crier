use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstallationRepositoriesEvent {
    pub action: InstallationRepositoriesAction,
    pub installation: Installation,
    pub repository_selection: String,  // "all" or "selected"
    pub repositories_added: Vec<InstallationRepository>,
    pub repositories_removed: Vec<InstallationRepository>,
    pub requester: Option<User>,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstallationRepositoriesAction {
    Added,
    Removed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct InstallationRepository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
}

impl DiscordTransform for InstallationRepositoriesEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let installation = &self.installation;
        
        let (color, emoji, action_text) = match self.action {
            InstallationRepositoriesAction::Added => (Colors::DULL_GREEN, "➕", "Repositories added to app"),
            InstallationRepositoriesAction::Removed => (Colors::DULL_RED, "➖", "Repositories removed from app"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Installation info
        if let Some(app_slug) = &installation.app_slug {
            fields.push(field("App", app_slug, true));
        }
        
        // Account
        fields.push(field(
            "Account",
            format!("[{}]({})", installation.account.login, installation.account.html_url),
            true
        ));
        
        // Repository selection type
        let selection_icon = if self.repository_selection == "all" { "📚" } else { "📂" };
        fields.push(field(
            "Selection Type",
            format!("{} {}", selection_icon, self.repository_selection),
            true
        ));
        
        // Added repositories
        if !self.repositories_added.is_empty() {
            let added_list = self.repositories_added.iter()
                .take(15)
                .map(|r| {
                    let visibility = if r.private { "🔒" } else { "🌍" };
                    format!("+ {} {}", visibility, r.name)
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            let added_field = if self.repositories_added.len() > 15 {
                format!("{}\n... and {} more", added_list, self.repositories_added.len() - 15)
            } else {
                added_list
            };
            
            fields.push(field(
                &format!("➕ Added ({})", self.repositories_added.len()),
                added_field,
                false
            ));
        }
        
        // Removed repositories
        if !self.repositories_removed.is_empty() {
            let removed_list = self.repositories_removed.iter()
                .take(15)
                .map(|r| {
                    let visibility = if r.private { "🔒" } else { "🌍" };
                    format!("- {} {}", visibility, r.name)
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            let removed_field = if self.repositories_removed.len() > 15 {
                format!("{}\n... and {} more", removed_list, self.repositories_removed.len() - 15)
            } else {
                removed_list
            };
            
            fields.push(field(
                &format!("➖ Removed ({})", self.repositories_removed.len()),
                removed_field,
                false
            ));
        }
        
        // Summary counts
        let total_added = self.repositories_added.len();
        let total_removed = self.repositories_removed.len();
        
        if total_added > 0 || total_removed > 0 {
            fields.push(field(
                "Summary",
                format!("+{} / -{}", total_added, total_removed),
                true
            ));
        }
        
        // Requester if different from sender
        if let Some(requester) = &self.requester {
            if requester.login != self.sender.login {
                fields.push(field("Requested By", &requester.login, true));
            }
        }
        
        let description = Some(format!(
            "{} repositories {} for GitHub App installation",
            if total_added > 0 && total_removed > 0 {
                format!("{} added, {} removed", total_added, total_removed)
            } else if total_added > 0 {
                format!("{} added", total_added)
            } else {
                format!("{} removed", total_removed)
            },
            match self.action {
                InstallationRepositoriesAction::Added => "were added to",
                InstallationRepositoriesAction::Removed => "were removed from",
            }
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