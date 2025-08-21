use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryAdvisoryEvent {
    pub action: String,
    pub repository: Repository,
    pub repository_advisory: RepositoryAdvisory,
    pub sender: User,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RepositoryAdvisory {
    pub ghsa_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub severity: String,
    pub cvss: Option<serde_json::Value>,
}

impl DiscordTransform for RepositoryAdvisoryEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji) = match self.action.as_str() {
            "published" => (Colors::RED, "🚨"),
            "updated" => (Colors::YELLOW, "⚠️"),
            "reported" => (Colors::YELLOW, "📢"),
            _ => (Colors::DULL_BLUE, "🔒"),
        };
        
        let severity_emoji = match self.repository_advisory.severity.as_str() {
            "critical" => "🔴",
            "high" => "🟠",
            "moderate" | "medium" => "🟡",
            "low" => "🟢",
            _ => "⚪",
        };
        
        let title = format!("{} {} Repository advisory {}", severity_emoji, emoji, self.action);
        
        let mut fields = vec![];
        
        fields.push(field("Advisory", &self.repository_advisory.ghsa_id, true));
        fields.push(field("Severity", format!("{} {}", severity_emoji, self.repository_advisory.severity.to_uppercase()), true));
        fields.push(field("Repository", format!("[{}]({})", self.repository.full_name, self.repository.html_url), true));
        
        DiscordEmbed {
            title,
            description: Some(self.repository_advisory.summary.clone()),
            url: Some(format!("{}/security/advisories", self.repository.html_url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Security Advisory".to_string(),
                icon_url: Some("https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png".to_string()),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
