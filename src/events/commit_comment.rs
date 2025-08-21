use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform, AuthorAssociation};
use crate::transform::{Colors, field, code_field, truncate_string};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommitComment {
    pub url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub position: Option<i64>,
    pub line: Option<i64>,
    pub path: Option<String>,
    pub commit_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: AuthorAssociation,
    pub body: String,
    pub reactions: Option<super::issues::Reactions>,
}

impl DiscordTransform for CommitCommentEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let comment = &self.comment;
        
        let title = "💬 Comment on commit";
        
        let mut fields = vec![];
        
        // Commit SHA
        let short_sha = if comment.commit_id.len() >= 7 {
            &comment.commit_id[..7]
        } else {
            &comment.commit_id
        };
        fields.push(code_field("Commit", short_sha, true));
        
        // File and line if available
        if let Some(path) = &comment.path {
            let location = if let Some(line) = comment.line {
                format!("{}:{}", path, line)
            } else {
                path.clone()
            };
            fields.push(code_field("Location", location, false));
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
        
        let description = Some(truncate_string(&comment.body, 500));
        
        DiscordEmbed {
            title: title.to_string(),
            description,
            url: Some(comment.html_url.clone()),
            color: Colors::DULL_BLUE,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Commit Comment".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}