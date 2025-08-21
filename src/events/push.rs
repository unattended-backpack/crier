use crate::transform::{code_field, field, format_commit_message, Colors};
use crate::types::{DiscordTransform, Installation, Organization, Repository, User};
use crate::{DiscordAuthor, DiscordEmbed, DiscordFooter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: User,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: Option<Commit>,

    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}

impl DiscordTransform for PushEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let branch = self.ref_.replace("refs/heads/", "");
        let commit_count = self.commits.len();

        // Determine color based on action
        let color = if self.deleted {
            Colors::DULL_RED
        } else if self.created {
            Colors::DULL_GREEN
        } else if self.forced {
            Colors::RED
        } else {
            Colors::DULL_BLUE
        };

        // Build title with appropriate emoji
        let emoji = if self.deleted {
            "🗑️"
        } else if self.created {
            "🌱"
        } else if self.forced {
            "⚠️"
        } else {
            "📤"
        };

        let title = if self.deleted {
            format!("{} Branch {} deleted", emoji, branch)
        } else if self.created {
            format!("{} Branch {} created", emoji, branch)
        } else {
            format!(
                "{} {} commit{} to {}",
                emoji,
                commit_count,
                if commit_count == 1 { "" } else { "s" },
                branch
            )
        };

        let mut fields = vec![];

        // Add branch field
        fields.push(code_field("Branch", &branch, true));

        // Add pusher info
        fields.push(field("Pusher", &self.pusher.name, true));

        // Add commit messages (up to 5)
        if !self.commits.is_empty() && !self.deleted {
            let messages: Vec<String> = self
                .commits
                .iter()
                .take(5)
                .map(|c| {
                    let msg = format_commit_message(&c.message);
                    let author = c.author.username.as_ref().unwrap_or(&c.author.name);
                    format!(
                        "• [`{}`]({}) - {} ({})",
                        if c.id.len() >= 7 { &c.id[..7] } else { &c.id },
                        c.url,
                        msg,
                        author
                    )
                })
                .collect();

            if !messages.is_empty() {
                let mut value = messages.join("\n");
                if self.commits.len() > 5 {
                    value.push_str(&format!("\n... and {} more", self.commits.len() - 5));
                }
                fields.push(field("Commits", value, false));
            }
        }

        // Add file statistics if we have commits
        if !self.commits.is_empty() && !self.deleted {
            let mut total_added = 0;
            let mut total_removed = 0;
            let mut total_modified = 0;

            for commit in &self.commits {
                total_added += commit.added.len();
                total_removed += commit.removed.len();
                total_modified += commit.modified.len();
            }

            if total_added > 0 || total_removed > 0 || total_modified > 0 {
                fields.push(field(
                    "Files Changed",
                    format!(
                        "+{} added, -{} removed, ~{} modified",
                        total_added, total_removed, total_modified
                    ),
                    false,
                ));
            }
        }

        // Add repository field
        fields.push(field(
            "Repository",
            format!(
                "[{}]({})",
                self.repository.full_name, self.repository.html_url
            ),
            true,
        ));

        // Build description
        let description = if self.deleted {
            Some(format!(
                "Branch `{}` was deleted from {}",
                branch, self.repository.full_name
            ))
        } else if self.created {
            Some(format!(
                "Branch `{}` was created in {}",
                branch, self.repository.full_name
            ))
        } else if self.forced {
            Some(format!(
                "⚠️ Force push to `{}` in {}",
                branch, self.repository.full_name
            ))
        } else if let Some(head) = &self.head_commit {
            Some(format_commit_message(&head.message))
        } else {
            None
        };

        DiscordEmbed {
            title,
            description,
            url: Some(self.compare.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Push".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
