use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PageBuildEvent {
    pub id: i64,
    pub build: PageBuild,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PageBuild {
    pub url: String,
    pub status: String,  // "built", "building", "errored"
    pub error: Option<PageBuildError>,
    pub pusher: User,
    pub commit: String,
    pub duration: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PageBuildError {
    pub message: Option<String>,
}

impl DiscordTransform for PageBuildEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let build = &self.build;
        
        let (color, emoji, status_text) = match build.status.as_str() {
            "built" => (Colors::GREEN, "✅", "built successfully"),
            "building" => (Colors::YELLOW, "🔨", "building"),
            "errored" => (Colors::RED, "❌", "failed"),
            _ => (Colors::GRAY, "❓", build.status.as_str()),
        };
        
        let title = format!("{} GitHub Pages {}", emoji, status_text);
        
        let mut fields = vec![];
        
        // Status
        fields.push(field("Status", status_text, true));
        
        // Duration
        let duration_seconds = build.duration / 1000;
        let duration_text = if duration_seconds > 60 {
            format!("{}m {}s", duration_seconds / 60, duration_seconds % 60)
        } else {
            format!("{}s", duration_seconds)
        };
        fields.push(field("Build Time", duration_text, true));
        
        // Commit
        let short_commit = if build.commit.len() >= 7 {
            &build.commit[..7]
        } else {
            &build.commit
        };
        fields.push(code_field("Commit", short_commit, true));
        
        // Pusher
        fields.push(field("Triggered By", &build.pusher.login, true));
        
        // Error message if failed
        if let Some(error) = &build.error {
            if let Some(msg) = &error.message {
                fields.push(field("Error", msg, false));
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Pages URL
        let pages_url = format!("https://{}.github.io/{}/", 
            self.repository.owner.login,
            self.repository.name
        );
        fields.push(field("Pages URL", format!("[View Site]({})", pages_url), true));
        
        let description = Some(format!(
            "GitHub Pages build {} for {}",
            status_text, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(pages_url),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Pages".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}