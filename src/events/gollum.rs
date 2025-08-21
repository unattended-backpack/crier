use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GollumEvent {
    pub pages: Vec<WikiPage>,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WikiPage {
    pub page_name: String,
    pub title: String,
    pub summary: Option<String>,
    pub action: String,  // "created" or "edited"
    pub sha: String,
    pub html_url: String,
}

impl DiscordTransform for GollumEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let page_count = self.pages.len();
        let emoji = "📚";
        
        let title = if page_count == 1 {
            format!("{} Wiki page {}", emoji, self.pages[0].action)
        } else {
            format!("{} {} wiki pages updated", emoji, page_count)
        };
        
        let mut fields = vec![];
        
        // List pages (up to 10)
        for page in self.pages.iter().take(10) {
            let action_emoji = if page.action == "created" { "✨" } else { "✏️" };
            let page_info = if let Some(summary) = &page.summary {
                format!("{} {} - {}", action_emoji, page.action, summary)
            } else {
                format!("{} {}", action_emoji, page.action)
            };
            
            fields.push(field(
                &page.title,
                format!("[{}]({})", page_info, page.html_url),
                false
            ));
        }
        
        if self.pages.len() > 10 {
            fields.push(field(
                "More Pages",
                format!("... and {} more", self.pages.len() - 10),
                false
            ));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "{} updated {} wiki page{} in {}",
            self.sender.login,
            page_count,
            if page_count == 1 { "" } else { "s" },
            self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!("{}/wiki", self.repository.html_url)),
            color: Colors::DULL_BLUE,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Wiki".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}