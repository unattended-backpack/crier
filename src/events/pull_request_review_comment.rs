use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform, AuthorAssociation};
use crate::transform::{Colors, field, code_field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::pull_request::PullRequest;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequestReviewCommentEvent {
    pub action: PullRequestReviewCommentAction,
    pub comment: ReviewComment,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ReviewComment {
    pub url: String,
    pub pull_request_review_id: Option<i64>,
    pub id: i64,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: Option<i64>,
    pub original_position: Option<i64>,
    pub commit_id: String,
    pub original_commit_id: String,
    pub user: User,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    pub _links: ReviewCommentLinks,
    pub reactions: Option<super::issues::Reactions>,
    pub start_line: Option<i64>,
    pub original_start_line: Option<i64>,
    pub start_side: Option<String>,
    pub line: Option<i64>,
    pub original_line: Option<i64>,
    pub side: Option<String>,
    pub in_reply_to_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ReviewCommentLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub html: Link,
    pub pull_request: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Link {
    pub href: String,
}

impl DiscordTransform for PullRequestReviewCommentEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let comment = &self.comment;
        let pr = &self.pull_request;
        
        let (color, emoji, action_text) = match self.action {
            PullRequestReviewCommentAction::Created => (Colors::DULL_BLUE, "💬", "commented on"),
            PullRequestReviewCommentAction::Edited => (Colors::DULL_BLUE, "✏️", "edited comment on"),
            PullRequestReviewCommentAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted comment on"),
        };
        
        let title = format!("{} {} code in PR #{}", emoji, action_text, pr.number);
        
        let mut fields = vec![];
        
        // PR title
        fields.push(field("Pull Request", &pr.title, false));
        
        // File and line
        let location = if let Some(line) = comment.line {
            format!("{}:{}", comment.path, line)
        } else if let Some(line) = comment.original_line {
            format!("{}:{}", comment.path, line)
        } else {
            comment.path.clone()
        };
        fields.push(code_field("Location", location, false));
        
        // Side (for split diffs)
        if let Some(side) = &comment.side {
            let side_text = if side == "LEFT" { "⬅️ Left (old)" } else { "➡️ Right (new)" };
            fields.push(field("Side", side_text, true));
        }
        
        // Is this a reply?
        if comment.in_reply_to_id.is_some() {
            fields.push(field("Type", "↩️ Reply", true));
        }
        
        // Author association
        let author_badge = match comment.author_association {
            AuthorAssociation::Owner => "👑 Owner",
            AuthorAssociation::Member => "👥 Member",
            AuthorAssociation::Collaborator => "🤝 Collaborator",
            AuthorAssociation::Contributor => "✨ Contributor",
            AuthorAssociation::FirstTimeContributor => "🌟 First Time Contributor",
            AuthorAssociation::FirstTimer => "🎉 First Timer",
            AuthorAssociation::Mannequin => "🤖 Mannequin",
            AuthorAssociation::None => "👤 None",
        };
        fields.push(field("Author", author_badge, true));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Build description
        let description = match self.action {
            PullRequestReviewCommentAction::Created | PullRequestReviewCommentAction::Edited => {
                Some(truncate_string(&comment.body, 500))
            }
            PullRequestReviewCommentAction::Deleted => {
                Some(format!(
                    "Comment on {} in PR #{} was deleted",
                    comment.path, pr.number
                ))
            }
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(comment.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Code Review".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}