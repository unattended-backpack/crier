use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionEvent {
    pub action: DiscussionAction,
    pub discussion: Discussion,
    pub repository: Repository,
    pub sender: User,
    pub label: Option<Label>,
    pub changes: Option<DiscussionChanges>,
    pub answer: Option<DiscussionComment>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionAction {
    Created,
    Edited,
    Deleted,
    Pinned,
    Unpinned,
    Locked,
    Unlocked,
    Transferred,
    CategoryChanged,
    Answered,
    Unanswered,
    Labeled,
    Unlabeled,
    Closed,
    Reopened,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Discussion {
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: User,
    pub state: String,  // "open", "closed"
    pub state_reason: Option<String>,  // "resolved", "outdated", "duplicate", "reopened"
    pub locked: bool,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    pub category: DiscussionCategory,
    pub answer_html_url: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionCategory {
    pub id: i64,
    pub node_id: String,
    pub repository_id: i64,
    pub emoji: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub slug: String,
    pub is_answerable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionComment {
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub parent_id: Option<i64>,
    pub child_comment_count: i64,
    pub repository_url: String,
    pub discussion_id: i64,
    pub author_association: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionChanges {
    pub title: Option<Change<String>>,
    pub body: Option<Change<String>>,
    pub category: Option<CategoryChange>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Change<T> {
    pub from: T,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CategoryChange {
    pub from: DiscussionCategory,
}

impl DiscordTransform for DiscussionEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let discussion = &self.discussion;
        
        let (color, emoji, action_text) = match self.action {
            DiscussionAction::Created => (Colors::DULL_BLUE, "💬", "created"),
            DiscussionAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            DiscussionAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
            DiscussionAction::Pinned => (Colors::DULL_GREEN, "📌", "pinned"),
            DiscussionAction::Unpinned => (Colors::GRAY, "📌", "unpinned"),
            DiscussionAction::Locked => (Colors::DULL_YELLOW, "🔒", "locked"),
            DiscussionAction::Unlocked => (Colors::DULL_GREEN, "🔓", "unlocked"),
            DiscussionAction::Transferred => (Colors::DULL_BLUE, "➡️", "transferred"),
            DiscussionAction::CategoryChanged => (Colors::DULL_BLUE, "🏷️", "category changed"),
            DiscussionAction::Answered => (Colors::GREEN, "✅", "marked as answered"),
            DiscussionAction::Unanswered => (Colors::DULL_YELLOW, "❓", "unmarked as answered"),
            DiscussionAction::Labeled => (Colors::DULL_BLUE, "🏷️", "labeled"),
            DiscussionAction::Unlabeled => (Colors::GRAY, "🏷️", "unlabeled"),
            DiscussionAction::Closed => (Colors::DULL_RED, "🔴", "closed"),
            DiscussionAction::Reopened => (Colors::DULL_GREEN, "🟢", "reopened"),
        };
        
        let title = format!("{} Discussion {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Discussion number and title
        fields.push(field(
            "Discussion",
            format!("#{} {}", discussion.number, discussion.title),
            false
        ));
        
        // Category with emoji
        fields.push(field(
            "Category",
            format!("{} {}", discussion.category.emoji, discussion.category.name),
            true
        ));
        
        // State
        let state_icon = match discussion.state.as_str() {
            "open" => "🟢",
            "closed" => "🔴",
            _ => "⚪",
        };
        fields.push(field("State", format!("{} {}", state_icon, discussion.state), true));
        
        // Comments count
        if discussion.comments > 0 {
            fields.push(field("💬 Comments", discussion.comments.to_string(), true));
        }
        
        // Lock status
        if discussion.locked {
            let lock_reason = discussion.active_lock_reason.as_deref().unwrap_or("locked");
            fields.push(field("🔒 Locked", lock_reason, true));
        }
        
        // Answer info for answered/unanswered actions
        if matches!(self.action, DiscussionAction::Answered | DiscussionAction::Unanswered) {
            if let Some(answer) = &self.answer {
                let answer_preview = if answer.body.len() > 100 {
                    format!("{}...", &answer.body[..97])
                } else {
                    answer.body.clone()
                };
                fields.push(field("Answer", answer_preview, false));
                fields.push(field("Answer By", &answer.user.login, true));
            }
        }
        
        // Label info for label actions
        if matches!(self.action, DiscussionAction::Labeled | DiscussionAction::Unlabeled) {
            if let Some(label) = &self.label {
                fields.push(field(
                    "Label",
                    format!("🏷️ {}", label.name),
                    true
                ));
            }
        }
        
        // Changes for edited or category changed
        if let Some(changes) = &self.changes {
            let mut change_items = vec![];
            if let Some(title_change) = &changes.title {
                change_items.push(format!("Title: {} → {}", title_change.from, discussion.title));
            }
            if changes.body.is_some() {
                change_items.push("Body updated".to_string());
            }
            if let Some(cat_change) = &changes.category {
                change_items.push(format!(
                    "Category: {} {} → {} {}",
                    cat_change.from.emoji, cat_change.from.name,
                    discussion.category.emoji, discussion.category.name
                ));
            }
            if !change_items.is_empty() {
                fields.push(field("Changes", change_items.join("\n"), false));
            }
        }
        
        // State reason if closed
        if let Some(reason) = &discussion.state_reason {
            let reason_icon = match reason.as_str() {
                "resolved" => "✅",
                "outdated" => "📅",
                "duplicate" => "👥",
                _ => "📝",
            };
            fields.push(field("Reason", format!("{} {}", reason_icon, reason), true));
        }
        
        // Author
        fields.push(field("Author", &discussion.user.login, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = if let Some(body) = &discussion.body {
            let truncated = if body.len() > 300 {
                format!("{}...", &body[..297])
            } else {
                body.clone()
            };
            Some(truncated)
        } else {
            Some(format!("Discussion #{} was {}", discussion.number, action_text))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(discussion.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Discussions".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}