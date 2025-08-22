use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecretScanningScanEvent {
    pub action: String,  // "completed"
    #[serde(rename = "type")]
    pub scan_type: String,  // "backfill", "push", etc.
    pub source: String,  // "git"
    pub started_at: String,
    pub completed_at: String,
    pub secret_types: Vec<String>,  // Don't log to Discord
    pub custom_pattern_name: Option<String>,  // Don't log to Discord
    pub custom_pattern_scope: Option<String>,  // Don't log to Discord
    pub repository: Repository,
    
    // Optional fields
    pub sender: Option<User>,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

impl DiscordTransform for SecretScanningScanEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "completed" => (Colors::GREEN, "✅"),
            _ => (Colors::YELLOW, "🔍"),
        };
        
        let title = format!("{} Secret scanning {} - {}", emoji, self.scan_type, self.action);
        
        let mut fields = vec![];
        
        // Scan type
        fields.push(field("Scan Type", &self.scan_type, true));
        
        // Source
        fields.push(field("Source", &self.source, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Duration calculation
        if let (Ok(start), Ok(end)) = (
            chrono::DateTime::parse_from_rfc3339(&self.started_at),
            chrono::DateTime::parse_from_rfc3339(&self.completed_at)
        ) {
            let duration = end.signed_duration_since(start);
            let duration_str = if duration.num_seconds() < 60 {
                format!("{}s", duration.num_seconds())
            } else if duration.num_minutes() < 60 {
                format!("{}m {}s", duration.num_minutes(), duration.num_seconds() % 60)
            } else {
                format!("{}h {}m", duration.num_hours(), duration.num_minutes() % 60)
            };
            fields.push(field("Duration", duration_str, true));
        }
        
        // Started and completed times
        fields.push(field("Started", &self.started_at.replace('T', " ").replace('Z', " UTC"), true));
        fields.push(field("Completed", &self.completed_at.replace('T', " ").replace('Z', " UTC"), true));
        
        let description = Some(format!(
            "Secret scanning {} completed for repository {}",
            self.scan_type,
            self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!("{}/security/secret-scanning", self.repository.html_url)),
            color,
            author: self.sender.as_ref().map(|sender| DiscordAuthor {
                name: sender.login.clone(),
                url: Some(sender.html_url.clone()),
                icon_url: Some(sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Secret Scanning".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}