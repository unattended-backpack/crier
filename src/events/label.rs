use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::issues::Label;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LabelEvent {
    pub action: LabelAction,
    pub label: Label,
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
pub enum LabelAction {
    Created,
    Edited,
    Deleted,
}

impl DiscordTransform for LabelEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let label = &self.label;
        
        let (color, emoji, action_text) = match self.action {
            LabelAction::Created => (Colors::DULL_GREEN, "🏷️", "created"),
            LabelAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            LabelAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
        };
        
        let title = format!("{} Label {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Label name with color preview
        let label_color = u32::from_str_radix(&label.color, 16).unwrap_or(0x808080);
        fields.push(field("Label", format!("`{}`", label.name), true));
        
        // Color (as hex)
        fields.push(field("Color", format!("#{}", label.color), true));
        
        // Default label indicator
        if label.default {
            fields.push(field("Type", "📌 Default Label", true));
        }
        
        // Description
        if let Some(desc) = &label.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Changes if edited
        if self.action == LabelAction::Edited {
            if let Some(changes) = &self.changes {
                if let Some(name_change) = changes.get("name") {
                    if let Some(from) = name_change.get("from").and_then(|v| v.as_str()) {
                        fields.push(field("Previous Name", from, true));
                    }
                }
                if let Some(color_change) = changes.get("color") {
                    if let Some(from) = color_change.get("from").and_then(|v| v.as_str()) {
                        fields.push(field("Previous Color", format!("#{}", from), true));
                    }
                }
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Label '{}' was {} in {}",
            label.name, action_text, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!("{}/labels/{}", self.repository.html_url, urlencoding::encode(&label.name))),
            color: label_color, // Use the label's color for the embed
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Label".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}