use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionCommentEvent {
    pub action: DiscussionCommentAction,
    pub comment: DiscussionComment,
    pub discussion: Discussion,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionCommentAction {
    Created,
    Edited,
    Deleted,
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
pub struct Discussion {
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: User,
    pub state: String,
    pub locked: bool,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub repository_url: String,
    pub category: DiscussionCategory,
    
    // Additional fields from the actual API
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub comments: i64,
    #[serde(default)]
    pub author_association: String,
    #[serde(default)]
    pub active_lock_reason: Option<String>,
    #[serde(default)]
    pub answer_chosen_at: Option<String>,
    #[serde(default)]
    pub answer_chosen_by: Option<User>,
    #[serde(default)]
    pub answer_html_url: Option<String>,
    #[serde(default)]
    pub reactions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DiscussionCategory {
    pub id: i64,
    pub node_id: String,
    pub repository_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub slug: String,
    pub is_answerable: bool,
}

impl DiscordTransform for DiscussionCommentEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            DiscussionCommentAction::Created => (Colors::DULL_BLUE, "💬", "New comment"),
            DiscussionCommentAction::Edited => (Colors::DULL_BLUE, "✏️", "Comment edited"),
            DiscussionCommentAction::Deleted => (Colors::DULL_RED, "🗑️", "Comment deleted"),
        };
        
        let title = format!("{} {} on discussion #{}", emoji, action_text, self.discussion.number);
        
        let mut fields = vec![];
        
        // Discussion title
        fields.push(field(
            "Discussion",
            format!("[{}]({})", self.discussion.title, self.discussion.html_url),
            false
        ));
        
        // Category
        let category_display = if let Some(emoji) = &self.discussion.category.emoji {
            format!("{} {}", emoji, self.discussion.category.name)
        } else {
            self.discussion.category.name.clone()
        };
        fields.push(field("Category", category_display, true));
        
        // Comment type
        if self.comment.parent_id.is_some() {
            fields.push(field("Type", "Reply", true));
        } else {
            fields.push(field("Type", "Top-level comment", true));
        }
        
        // Reply count
        if self.comment.child_comment_count > 0 {
            fields.push(field("Replies", self.comment.child_comment_count.to_string(), true));
        }
        
        // Author association
        fields.push(field("Author Association", &self.comment.author_association, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = if self.action != DiscussionCommentAction::Deleted {
            Some(truncate_string(&self.comment.body, 500))
        } else {
            Some("Comment was deleted".to_string())
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(self.comment.html_url.clone()),
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