use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ReleaseEvent {
    pub action: ReleaseAction,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub changes: Option<serde_json::Value>,
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseAction {
    Published,
    Unpublished,
    Created,
    Edited,
    Deleted,
    Prereleased,
    Released,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    pub assets: Vec<Asset>,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub body: Option<String>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub mentions_count: Option<i64>,
    pub discussion_url: Option<String>,
    pub reactions: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: User,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl DiscordTransform for ReleaseEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            ReleaseAction::Published => (Colors::GREEN, "🚀", "published"),
            ReleaseAction::Created => (Colors::DULL_GREEN, "📦", "created"),
            ReleaseAction::Prereleased => (Colors::YELLOW, "🔬", "pre-released"),
            ReleaseAction::Released => (Colors::GREEN, "✅", "released"),
            ReleaseAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            ReleaseAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
            ReleaseAction::Unpublished => (Colors::DULL_RED, "📤", "unpublished"),
        };
        
        let release_type = if self.release.prerelease { 
            "Pre-release" 
        } else if self.release.draft { 
            "Draft" 
        } else { 
            "Release" 
        };
        
        let title = format!(
            "{} {} {} {}",
            emoji,
            release_type,
            self.release.tag_name,
            action_text
        );
        
        let mut fields = vec![];
        
        // Release name and tag
        if let Some(name) = &self.release.name {
            if name != &self.release.tag_name {
                fields.push(field("Release Name", name, false));
            }
        }
        fields.push(code_field("Tag", &self.release.tag_name, true));
        
        // Target branch
        fields.push(code_field("Target", &self.release.target_commitish, true));
        
        // Release type badges
        let mut badges = vec![];
        if self.release.prerelease {
            badges.push("🔬 Pre-release");
        }
        if self.release.draft {
            badges.push("📝 Draft");
        }
        if !badges.is_empty() {
            fields.push(field("Type", badges.join(" "), true));
        }
        
        // Assets
        if !self.release.assets.is_empty() {
            let asset_info: Vec<String> = self.release.assets.iter()
                .take(5)
                .map(|a| {
                    let size_mb = a.size as f64 / 1_048_576.0;
                    format!(
                        "• [`{}`]({}) ({:.1} MB, {} downloads)",
                        a.name, a.browser_download_url, size_mb, a.download_count
                    )
                })
                .collect();
            
            let mut assets_text = asset_info.join("\n");
            if self.release.assets.len() > 5 {
                assets_text.push_str(&format!("\n... and {} more", self.release.assets.len() - 5));
            }
            fields.push(field("Assets", assets_text, false));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Author
        fields.push(field("Author", &self.release.author.login, true));
        
        // Build description from release body
        let description = if let Some(body) = &self.release.body {
            Some(truncate_string(body, 500))
        } else {
            Some(format!(
                "{} {} {} in {}",
                release_type, self.release.tag_name, action_text, self.repository.full_name
            ))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.release.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Release".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}