use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform, AuthorAssociation};
use crate::transform::{Colors, field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::issues::{Issue};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct IssueCommentEvent {
    pub action: IssueCommentAction,
    pub issue: Issue,
    pub comment: IssueComment,
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
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct IssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: AuthorAssociation,
    pub body: String,
    pub reactions: Option<super::issues::Reactions>,
    pub performed_via_github_app: Option<serde_json::Value>,
}

impl DiscordTransform for IssueCommentEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let issue = &self.issue;
        let comment = &self.comment;
        
        let (color, emoji, action_text) = match self.action {
            IssueCommentAction::Created => (Colors::DULL_BLUE, "💬", "commented on"),
            IssueCommentAction::Edited => (Colors::DULL_BLUE, "✏️", "edited comment on"),
            IssueCommentAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted comment on"),
        };
        
        let issue_or_pr = if issue.pull_request.is_some() {
            "Pull Request"
        } else {
            "Issue"
        };
        
        let title = format!("{} {} {} #{}", emoji, action_text, issue_or_pr, issue.number);
        
        let mut fields = vec![];
        
        // Issue/PR title
        fields.push(field(
            &format!("{} Title", issue_or_pr),
            &issue.title,
            false
        ));
        
        // Issue/PR state
        let state_emoji = if issue.state == "open" { "🟢" } else { "🔴" };
        fields.push(field("State", format!("{} {}", state_emoji, issue.state), true));
        
        // Comment count
        fields.push(field("Total Comments", format!("💬 {}", issue.comments), true));
        
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
            IssueCommentAction::Created | IssueCommentAction::Edited => {
                Some(truncate_string(&comment.body, 500))
            }
            IssueCommentAction::Deleted => {
                Some(format!(
                    "Comment by {} was deleted from {} #{}",
                    comment.user.login, issue_or_pr.to_lowercase(), issue.number
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
                text: format!("GitHub {}", issue_or_pr),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}