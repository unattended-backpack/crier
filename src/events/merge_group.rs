use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MergeGroupEvent {
    pub action: MergeGroupAction,
    pub merge_group: MergeGroup,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MergeGroupAction {
    ChecksRequested,
    Destroyed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MergeGroup {
    pub head_sha: String,
    pub head_ref: String,
    pub base_sha: String,
    pub base_ref: String,
    pub created_at: String,
}

impl DiscordTransform for MergeGroupEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let group = &self.merge_group;
        
        let (color, emoji, action_text) = match self.action {
            MergeGroupAction::ChecksRequested => (Colors::YELLOW, "🔄", "checks requested"),
            MergeGroupAction::Destroyed => (Colors::DULL_RED, "💥", "destroyed"),
        };
        
        let title = format!("{} Merge group {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Base branch
        fields.push(code_field("Base Branch", &group.base_ref, true));
        
        // Head branch
        fields.push(code_field("Head Branch", &group.head_ref, true));
        
        // Commits
        let base_short = if group.base_sha.len() >= 7 {
            &group.base_sha[..7]
        } else {
            &group.base_sha
        };
        let head_short = if group.head_sha.len() >= 7 {
            &group.head_sha[..7]
        } else {
            &group.head_sha
        };
        
        fields.push(code_field(
            "Commits",
            format!("{}...{}", base_short, head_short),
            true
        ));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Merge group {} for {} → {}",
            action_text,
            group.head_ref,
            group.base_ref
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!(
                "{}/compare/{}...{}",
                self.repository.html_url,
                group.base_sha,
                group.head_sha
            )),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Merge Queue".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}