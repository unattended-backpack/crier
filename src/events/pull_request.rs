use crate::transform::{code_field, field, truncate_string, Colors};
use crate::types::{
    AuthorAssociation, DiscordTransform, Installation, Organization, Repository, User,
};
use crate::{DiscordAuthor, DiscordEmbed, DiscordFooter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: i64,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,

    // Optional fields based on action
    pub changes: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub label: Option<Label>,
    pub assignee: Option<User>,
    pub milestone: Option<Milestone>,
    pub requested_reviewer: Option<User>,
    pub requested_team: Option<Team>,

    // Optional context fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    AutoMergeDisabled,
    AutoMergeEnabled,
    Closed,
    ConvertedToDraft,
    Demilestoned,
    Dequeued,
    Edited,
    Enqueued,
    Labeled,
    Locked,
    Milestoned,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequestRemoved,
    ReviewRequested,
    Synchronize,
    Unassigned,
    Unlabeled,
    Unlocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub number: i64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: User,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub requested_teams: Vec<Team>,
    pub labels: Vec<Label>,
    pub milestone: Option<Milestone>,
    pub draft: bool,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub head: PullRequestRef,
    pub base: PullRequestRef,
    pub _links: PullRequestLinks,
    pub author_association: AuthorAssociation,
    pub auto_merge: Option<AutoMerge>,
    pub active_lock_reason: Option<String>,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: Option<User>,
    pub comments: i64,
    pub review_comments: i64,
    pub maintainer_can_modify: bool,
    pub commits: i64,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequestRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub sha: String,
    pub user: User,
    pub repo: Repository,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PullRequestLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub html: Link,
    pub issue: Link,
    pub comments: Link,
    pub review_comments: Link,
    pub review_comment: Link,
    pub commits: Link,
    pub statuses: Link,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Link {
    pub href: String,
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
pub struct Team {
    pub name: String,
    pub id: i64,
    pub node_id: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub notification_setting: Option<String>,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    pub parent: Option<Box<Team>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AutoMerge {
    pub enabled_by: User,
    pub merge_method: String,
    pub commit_title: Option<String>,
    pub commit_message: Option<String>,
}

impl DiscordTransform for PullRequestEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let pr = &self.pull_request;

        // Determine color and emoji based on action and state
        let (color, emoji) = match self.action {
            PullRequestAction::Opened => (Colors::DULL_GREEN, "📂"),
            PullRequestAction::Closed if pr.merged => (Colors::GREEN, "🎉"),
            PullRequestAction::Closed => (Colors::RED, "❌"),
            PullRequestAction::Reopened => (Colors::DULL_GREEN, "♻️"),
            PullRequestAction::ReadyForReview => (Colors::GREEN, "👀"),
            PullRequestAction::ConvertedToDraft => (Colors::DULL_BLUE, "📝"),
            PullRequestAction::Synchronize => (Colors::DULL_BLUE, "🔄"),
            PullRequestAction::Labeled => (Colors::DULL_BLUE, "🏷️"),
            PullRequestAction::ReviewRequested => (Colors::DULL_YELLOW, "👁️"),
            _ => (Colors::GRAY, "📋"),
        };

        let action_str = format!("{:?}", self.action)
            .split("::")
            .last()
            .unwrap_or("unknown")
            .chars()
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() && !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push(c.to_ascii_lowercase());
                acc
            });

        let title = format!("{} PR #{}: {}", emoji, pr.number, action_str);

        let mut fields = vec![];

        // PR title and branches
        fields.push(field("Title", &pr.title, false));
        fields.push(code_field(
            "Branches",
            format!("{} ← {}", pr.base.ref_, pr.head.ref_),
            true,
        ));

        // State and draft status
        let state_str = if pr.draft {
            format!("{} (draft)", pr.state)
        } else {
            pr.state.clone()
        };
        fields.push(code_field("State", state_str, true));

        // File changes
        if pr.additions > 0 || pr.deletions > 0 || pr.changed_files > 0 {
            fields.push(field(
                "Changes",
                format!(
                    "+{} / -{} in {} files",
                    pr.additions, pr.deletions, pr.changed_files
                ),
                true,
            ));
        }

        // Labels
        if !pr.labels.is_empty() {
            let labels = pr
                .labels
                .iter()
                .map(|l| format!("`{}`", l.name))
                .collect::<Vec<_>>()
                .join(" ");
            fields.push(field("Labels", truncate_string(&labels, 1024), false));
        }

        // Reviewers
        if !pr.requested_reviewers.is_empty() {
            let reviewers = pr
                .requested_reviewers
                .iter()
                .map(|r| r.login.clone())
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("Requested Reviewers", reviewers, true));
        }

        // Add specific fields based on action
        match &self.action {
            PullRequestAction::Labeled => {
                if let Some(label) = &self.label {
                    fields.push(field("Added Label", &label.name, true));
                }
            }
            PullRequestAction::ReviewRequested => {
                if let Some(reviewer) = &self.requested_reviewer {
                    fields.push(field("Reviewer Added", &reviewer.login, true));
                } else if let Some(team) = &self.requested_team {
                    fields.push(field("Team Reviewer Added", &team.name, true));
                }
            }
            PullRequestAction::Assigned => {
                if let Some(assignee) = &self.assignee {
                    fields.push(field("Assigned To", &assignee.login, true));
                }
            }
            _ => {}
        }

        // Repository
        fields.push(field(
            "Repository",
            format!(
                "[{}]({})",
                self.repository.full_name, self.repository.html_url
            ),
            true,
        ));

        // Build description
        let description = if let Some(body) = &pr.body {
            Some(truncate_string(body, 300))
        } else {
            None
        };

        DiscordEmbed {
            title,
            description,
            url: Some(pr.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Pull Request".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
