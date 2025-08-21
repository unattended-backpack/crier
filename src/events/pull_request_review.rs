use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform, AuthorAssociation};
use crate::transform::{Colors, field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};
use super::pull_request::PullRequest;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequestReviewEvent {
    pub action: PullRequestReviewAction,
    pub review: Review,
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
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Review {
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub body: Option<String>,
    pub commit_id: String,
    pub submitted_at: Option<String>,
    pub state: String,  // "approved", "changes_requested", "commented", "dismissed", "pending"
    pub html_url: String,
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    pub _links: ReviewLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ReviewLinks {
    pub html: Link,
    pub pull_request: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Link {
    pub href: String,
}

impl DiscordTransform for PullRequestReviewEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let review = &self.review;
        let pr = &self.pull_request;
        
        let (color, emoji, action_text) = match self.action {
            PullRequestReviewAction::Submitted => {
                match review.state.as_str() {
                    "approved" => (Colors::GREEN, "✅", "approved"),
                    "changes_requested" => (Colors::YELLOW, "🔄", "requested changes"),
                    "commented" => (Colors::DULL_BLUE, "💬", "commented on"),
                    "dismissed" => (Colors::DULL_RED, "🚫", "dismissed review on"),
                    _ => (Colors::DULL_BLUE, "👀", "reviewed"),
                }
            }
            PullRequestReviewAction::Edited => (Colors::DULL_BLUE, "✏️", "edited review on"),
            PullRequestReviewAction::Dismissed => (Colors::DULL_RED, "🚫", "dismissed review on"),
        };
        
        let title = format!("{} {} PR #{}", emoji, action_text, pr.number);
        
        let mut fields = vec![];
        
        // PR title
        fields.push(field("Pull Request", &pr.title, false));
        
        // Review state
        let state_display = match review.state.as_str() {
            "approved" => "✅ Approved",
            "changes_requested" => "🔄 Changes Requested",
            "commented" => "💬 Commented",
            "dismissed" => "🚫 Dismissed",
            "pending" => "⏳ Pending",
            _ => &review.state,
        };
        fields.push(field("Review State", state_display, true));
        
        // Author association
        let author_badge = match review.author_association {
            AuthorAssociation::Owner => "👑 Owner",
            AuthorAssociation::Member => "👥 Member",
            AuthorAssociation::Collaborator => "🤝 Collaborator",
            AuthorAssociation::Contributor => "✨ Contributor",
            AuthorAssociation::FirstTimeContributor => "🌟 First Time Contributor",
            AuthorAssociation::FirstTimer => "🎉 First Timer",
            AuthorAssociation::Mannequin => "🤖 Mannequin",
            AuthorAssociation::None => "👤 None",
        };
        fields.push(field("Reviewer", author_badge, true));
        
        // PR stats
        fields.push(field(
            "Changes",
            format!("+{} / -{} in {} files", 
                pr.additions, pr.deletions, pr.changed_files),
            true
        ));
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Build description
        let description = if let Some(body) = &review.body {
            if !body.is_empty() {
                Some(truncate_string(body, 500))
            } else {
                Some(format!(
                    "{} {} pull request #{}",
                    review.user.login, action_text, pr.number
                ))
            }
        } else {
            Some(format!(
                "{} {} pull request #{}",
                review.user.login, action_text, pr.number
            ))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(review.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Pull Request Review".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}