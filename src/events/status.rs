use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StatusEvent {
    pub id: i64,
    pub sha: String,
    pub name: Option<String>,
    pub target_url: Option<String>,
    pub context: String,
    pub description: Option<String>,
    pub state: StatusState,
    pub commit: Option<StatusCommit>,
    pub branches: Vec<StatusBranch>,
    pub created_at: String,
    pub updated_at: String,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StatusState {
    Error,
    Failure,
    Pending,
    Success,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StatusCommit {
    pub sha: String,
    pub node_id: String,
    pub commit: CommitDetails,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub parents: Vec<CommitParent>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitDetails {
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub message: String,
    pub tree: CommitTree,
    pub url: String,
    pub comment_count: i64,
    pub verification: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitTree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitParent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StatusBranch {
    pub name: String,
    pub commit: BranchCommit,
    pub protected: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BranchCommit {
    pub sha: String,
    pub url: String,
}

impl DiscordTransform for StatusEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, state_text) = match self.state {
            StatusState::Success => (Colors::GREEN, "✅", "Success"),
            StatusState::Failure => (Colors::RED, "❌", "Failure"),
            StatusState::Error => (Colors::RED, "🚨", "Error"),
            StatusState::Pending => (Colors::YELLOW, "⏳", "Pending"),
        };
        
        let title = format!("{} Status: {}", emoji, state_text);
        
        let mut fields = vec![];
        
        // Context (CI/CD system name)
        fields.push(field("Context", &self.context, true));
        
        // State
        fields.push(field("State", state_text, true));
        
        // Commit SHA
        let short_sha = if self.sha.len() >= 7 {
            &self.sha[..7]
        } else {
            &self.sha
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // Description
        if let Some(desc) = &self.description {
            fields.push(field("Description", desc, false));
        }
        
        // Branches
        if !self.branches.is_empty() {
            let branch_names = self.branches.iter()
                .take(5)
                .map(|b| format!("`{}`", b.name))
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("Branches", branch_names, false));
        }
        
        // Commit message if available
        if let Some(commit) = &self.commit {
            let msg = if commit.commit.message.len() > 100 {
                format!("{}...", &commit.commit.message[..97])
            } else {
                commit.commit.message.clone()
            };
            fields.push(field("Commit Message", msg, false));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Build status {} for commit {} in {}",
            state_text.to_lowercase(), short_sha, self.repository.full_name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: self.target_url.clone(),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: format!("GitHub Status • {}", self.context),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}