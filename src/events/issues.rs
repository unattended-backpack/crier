use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform, AuthorAssociation};
use crate::transform::{Colors, field, code_field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct IssuesEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields based on action
    pub changes: Option<serde_json::Value>,
    pub assignee: Option<User>,
    pub label: Option<Label>,
    pub milestone: Option<Milestone>,
    
    // Optional context fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssueAction {
    Opened,
    Edited,
    Deleted,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Unlabeled,
    Locked,
    Unlocked,
    Transferred,
    Milestoned,
    Demilestoned,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub milestone: Option<Milestone>,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: AuthorAssociation,
    pub active_lock_reason: Option<String>,
    pub body: Option<String>,
    pub closed_by: Option<User>,
    pub reactions: Option<Reactions>,
    pub timeline_url: Option<String>,
    pub performed_via_github_app: Option<serde_json::Value>,
    pub state_reason: Option<String>,
    pub draft: Option<bool>,
    pub pull_request: Option<IssuePullRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct IssuePullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub merged_at: Option<String>,
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
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub description: Option<String>,
    pub creator: User,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Reactions {
    pub url: String,
    pub total_count: i64,
    #[serde(rename = "+1")]
    pub plus_one: i64,
    #[serde(rename = "-1")]
    pub minus_one: i64,
    pub laugh: i64,
    pub hooray: i64,
    pub confused: i64,
    pub heart: i64,
    pub rocket: i64,
    pub eyes: i64,
}

impl DiscordTransform for IssuesEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let issue = &self.issue;
        
        let (color, emoji, action_text) = match self.action {
            IssueAction::Opened => (Colors::DULL_GREEN, "📋", "opened"),
            IssueAction::Closed => (Colors::GREEN, "✅", "closed"),
            IssueAction::Reopened => (Colors::DULL_GREEN, "♻️", "reopened"),
            IssueAction::Edited => (Colors::DULL_BLUE, "✏️", "edited"),
            IssueAction::Deleted => (Colors::DULL_RED, "🗑️", "deleted"),
            IssueAction::Pinned => (Colors::BLUE, "📌", "pinned"),
            IssueAction::Unpinned => (Colors::DULL_BLUE, "📌", "unpinned"),
            IssueAction::Assigned => (Colors::DULL_BLUE, "👤", "assigned"),
            IssueAction::Unassigned => (Colors::DULL_BLUE, "👤", "unassigned"),
            IssueAction::Labeled => (Colors::DULL_BLUE, "🏷️", "labeled"),
            IssueAction::Unlabeled => (Colors::DULL_BLUE, "🏷️", "unlabeled"),
            IssueAction::Locked => (Colors::YELLOW, "🔒", "locked"),
            IssueAction::Unlocked => (Colors::DULL_BLUE, "🔓", "unlocked"),
            IssueAction::Transferred => (Colors::DULL_BLUE, "➡️", "transferred"),
            IssueAction::Milestoned => (Colors::DULL_BLUE, "🎯", "milestoned"),
            IssueAction::Demilestoned => (Colors::DULL_BLUE, "🎯", "demilestoned"),
        };
        
        let title = format!("{} Issue #{}: {}", emoji, issue.number, action_text);
        
        let mut fields = vec![];
        
        // Issue title
        fields.push(field("Title", &issue.title, false));
        
        // State
        let state_emoji = if issue.state == "open" { "🟢" } else { "🔴" };
        fields.push(field("State", format!("{} {}", state_emoji, issue.state), true));
        
        // Comments
        if issue.comments > 0 {
            fields.push(field("Comments", format!("💬 {}", issue.comments), true));
        }
        
        // Assignees
        if !issue.assignees.is_empty() {
            let assignees = issue.assignees.iter()
                .map(|a| a.login.clone())
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("Assignees", assignees, true));
        }
        
        // Labels
        if !issue.labels.is_empty() {
            let labels = issue.labels.iter()
                .map(|l| format!("`{}`", l.name))
                .collect::<Vec<_>>()
                .join(" ");
            fields.push(field("Labels", truncate_string(&labels, 1024), false));
        }
        
        // Milestone
        if let Some(milestone) = &issue.milestone {
            fields.push(field(
                "Milestone",
                format!("{} ({}/{})", 
                    milestone.title,
                    milestone.closed_issues,
                    milestone.open_issues + milestone.closed_issues
                ),
                true
            ));
        }
        
        // Action-specific fields
        match &self.action {
            IssueAction::Assigned => {
                if let Some(assignee) = &self.assignee {
                    fields.push(field("Assigned To", &assignee.login, true));
                }
            }
            IssueAction::Labeled => {
                if let Some(label) = &self.label {
                    fields.push(field("Added Label", &label.name, true));
                }
            }
            IssueAction::Unlabeled => {
                if let Some(label) = &self.label {
                    fields.push(field("Removed Label", &label.name, true));
                }
            }
            IssueAction::Milestoned => {
                if let Some(milestone) = &self.milestone {
                    fields.push(field("Added to Milestone", &milestone.title, true));
                }
            }
            IssueAction::Closed => {
                if let Some(reason) = &issue.state_reason {
                    fields.push(field("Close Reason", reason, true));
                }
            }
            _ => {}
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Build description
        let description = if self.action == IssueAction::Opened {
            issue.body.as_ref().map(|b| truncate_string(b, 300))
        } else {
            Some(format!(
                "Issue #{} was {} by {}",
                issue.number, action_text, self.sender.login
            ))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(issue.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Issues".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}