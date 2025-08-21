use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageEvent {
    pub action: PackageAction,
    pub package: Package,
    pub repository: Option<Repository>,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PackageAction {
    Published,
    Updated,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Package {
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub description: Option<String>,
    pub ecosystem: String,  // "npm", "docker", "maven", "nuget", "rubygems", "cargo"
    pub package_type: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub owner: PackageOwner,
    pub package_version: PackageVersion,
    pub registry: PackageRegistry,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageOwner {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub type_: String,  // "User" or "Organization"
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageVersion {
    pub id: i64,
    pub version: String,
    pub summary: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub body: Option<String>,
    pub body_html: Option<String>,
    pub release: Option<PackageRelease>,
    pub manifest: Option<serde_json::Value>,
    pub html_url: String,
    pub tag_name: Option<String>,
    pub target_commitish: Option<String>,
    pub target_oid: Option<String>,
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
    pub metadata: Option<Vec<PackageMetadata>>,
    pub container_metadata: Option<ContainerMetadata>,
    pub docker_metadata: Option<Vec<DockerMetadata>>,
    pub package_files: Vec<PackageFile>,
    pub author: Option<User>,
    pub source_url: Option<String>,
    pub installation_command: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageRelease {
    pub url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub author: User,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageMetadata {
    pub package_type: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ContainerMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DockerMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageFile {
    pub download_url: String,
    pub id: i64,
    pub name: String,
    pub sha256: String,
    pub sha1: String,
    pub md5: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageRegistry {
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub vendor: String,
}

impl DiscordTransform for PackageEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let package = &self.package;
        let version = &package.package_version;
        
        let (color, emoji, action_text) = match self.action {
            PackageAction::Published => (Colors::GREEN, "📦", "Package published"),
            PackageAction::Updated => (Colors::DULL_GREEN, "📦", "Package updated"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Package name with ecosystem
        let ecosystem_emoji = match package.ecosystem.as_str() {
            "npm" => "📜",
            "docker" => "🐋",
            "maven" => "☕",
            "nuget" => "🔷",
            "rubygems" => "💎",
            "cargo" => "🦀",
            _ => "📦",
        };
        fields.push(field(
            "Package",
            format!("{} {} ({})", ecosystem_emoji, package.name, package.ecosystem),
            false
        ));
        
        // Version
        fields.push(field("Version", &version.version, true));
        
        // Prerelease/Draft status
        if version.prerelease.unwrap_or(false) {
            fields.push(field("Status", "🚧 Pre-release", true));
        } else if version.draft.unwrap_or(false) {
            fields.push(field("Status", "📝 Draft", true));
        }
        
        // Registry
        fields.push(field("Registry", &package.registry.name, true));
        
        // Installation command if available
        if let Some(cmd) = &version.installation_command {
            fields.push(code_field("Install", cmd, false));
        }
        
        // Container/Docker tags if applicable
        if let Some(container_meta) = &version.container_metadata {
            if !container_meta.tags.is_empty() {
                let tags = container_meta.tags.iter()
                    .take(5)
                    .map(|t| format!("`{}`", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                fields.push(field("🐋 Tags", tags, false));
            }
        }
        
        // Package files
        if !version.package_files.is_empty() {
            let total_size: i64 = version.package_files.iter().map(|f| f.size).sum();
            let size_mb = total_size as f64 / 1_048_576.0;
            fields.push(field(
                "Files",
                format!("{} files ({:.2} MB)", version.package_files.len(), size_mb),
                true
            ));
        }
        
        // Description
        if let Some(desc) = &package.description {
            if !desc.is_empty() {
                let truncated = if desc.len() > 300 {
                    format!("{}...", &desc[..297])
                } else {
                    desc.clone()
                };
                fields.push(field("Description", truncated, false));
            }
        }
        
        // Author
        if let Some(author) = &version.author {
            fields.push(field("Author", &author.login, true));
        }
        
        // Repository if linked
        if let Some(repo) = &self.repository {
            fields.push(field(
                "Repository",
                format!("[{}]({})", repo.full_name, repo.html_url),
                true
            ));
        }
        
        // Owner
        let owner_type_icon = if package.owner.type_ == "Organization" { "🏢" } else { "👤" };
        fields.push(field(
            "Owner",
            format!("{} [{}]({})",
                owner_type_icon,
                package.owner.login,
                package.owner.html_url
            ),
            true
        ));
        
        let description = Some(format!(
            "{} version {} has been {} to {}",
            package.name,
            version.version,
            if self.action == PackageAction::Published { "published" } else { "updated" },
            package.registry.name
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(package.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: format!("GitHub Packages • {}", package.registry.name),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}