use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::issues::Milestone;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MilestoneEvent {
    pub action: MilestoneAction,
    pub milestone: Milestone,
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
pub enum MilestoneAction {
    Created,
    Closed,
    Opened,
    Edited,
    Deleted,
}

impl DiscordTransform for MilestoneEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let milestone = &self.milestone;
        
        let (color, emoji, action_text) = match self.action {
            MilestoneAction::Created => (Colors::DULL_GREEN, "🎯", "created"),
            MilestoneAction::Closed => (Colors::GREEN, "✅", "closed"),
            MilestoneAction::Opened => (Colors::DULL_GREEN, "♻️", "reopened"),
            MilestoneAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            MilestoneAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };
        
        let title = format!("{} Milestone {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Milestone title
        fields.push(field("Milestone", &milestone.title, false));
        
        // State
        let state_emoji = if milestone.state == "open" { "🟢" } else { "✅" };
        fields.push(field("State", format!("{} {}", state_emoji, milestone.state), true));
        
        // Progress
        let total_issues = milestone.open_issues + milestone.closed_issues;
        let progress = if total_issues > 0 {
            let percentage = (milestone.closed_issues as f64 / total_issues as f64 * 100.0) as u32;
            format!("{}% ({}/{})", percentage, milestone.closed_issues, total_issues)
        } else {
            "No issues".to_string()
        };
        fields.push(field("Progress", progress, true));
        
        // Due date if set
        if let Some(due) = &milestone.due_on {
            fields.push(field("Due Date", &due[..10], true)); // Just the date part
        }
        
        // Description
        if let Some(desc) = &milestone.description {
            if !desc.is_empty() {
                fields.push(field("Description", truncate_string(desc, 500), false));
            }
        }
        
        // Issues count breakdown
        if total_issues > 0 {
            fields.push(field(
                "Issues",
                format!("🟢 {} open, ✅ {} closed", milestone.open_issues, milestone.closed_issues),
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
            "Milestone '{}' was {} by {}",
            milestone.title, action_text, self.sender.login
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(milestone.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Milestone".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}