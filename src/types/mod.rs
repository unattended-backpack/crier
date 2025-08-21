// Common types shared across GitHub webhook events
// Based on github-webhooks.schema.json

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod user;
pub mod repository;
pub mod organization;
pub mod installation;

pub use user::User;
pub use repository::Repository;
pub use organization::Organization;
pub use installation::{Installation, InstallationLite, Account};

// Re-export common enums
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimeContributor,
    Mannequin,
    Member,
    None,
    Owner,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

// Common trait for events that can be transformed to Discord
pub trait DiscordTransform {
    fn to_discord_embed(&self, event_type: &str) -> crate::DiscordEmbed;
}

// For forward compatibility - capture unknown fields
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ExtraFields {
    #[serde(flatten)]
    pub extra: Value,
}