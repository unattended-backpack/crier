use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RegistryPackageEvent {
    pub action: RegistryPackageAction,
    pub registry_package: RegistryPackage,
    pub sender: User,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RegistryPackageAction {
    Published,
    Updated,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RegistryPackage {
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub description: Option<String>,
    pub ecosystem: String,
    pub package_type: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub owner: PackageOwner,
    pub package_version: RegistryPackageVersion,
    pub registry: Registry,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageOwner {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RegistryPackageVersion {
    pub id: i64,
    pub version: String,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub manifest: Option<serde_json::Value>,
    pub html_url: String,
    pub tag_name: Option<String>,
    pub draft: Option<bool>,
    pub prerelease: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
    pub package_files: Vec<PackageFile>,
    pub author: Option<User>,
    pub docker_metadata: Option<DockerMetadata>,
    pub container_metadata: Option<ContainerMetadata>,
    pub npm_metadata: Option<NpmMetadata>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PackageFile {
    pub download_url: String,
    pub id: i64,
    pub name: String,
    pub sha256: Option<String>,
    pub sha1: Option<String>,
    pub md5: Option<String>,
    pub content_type: String,
    pub size: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Registry {
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DockerMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ContainerMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct NpmMetadata {
    pub name: Option<String>,
    pub version: Option<String>,
    pub npm_user: Option<String>,
    pub author: Option<serde_json::Value>,
    pub bugs: Option<serde_json::Value>,
    pub description: Option<String>,
    pub dist: Option<serde_json::Value>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub main: Option<String>,
    pub repository: Option<serde_json::Value>,
    pub scripts: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
    pub dev_dependencies: Option<serde_json::Value>,
}

impl DiscordTransform for RegistryPackageEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let package = &self.registry_package;
        let version = &package.package_version;
        
        let (color, emoji, action_text) = match self.action {
            RegistryPackageAction::Published => (Colors::GREEN, "📦", "Registry package published"),
            RegistryPackageAction::Updated => (Colors::DULL_GREEN, "📦", "Registry package updated"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Package name with ecosystem
        let ecosystem_emoji = match package.ecosystem.as_str() {
            "npm" => "📜",
            "docker" | "container" => "🐋",
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
        
        // Registry
        fields.push(field("Registry", &package.registry.name, true));
        
        // Package type
        fields.push(field("Type", &package.package_type, true));
        
        // Pre-release/Draft status
        if version.prerelease.unwrap_or(false) {
            fields.push(field("Status", "🚧 Pre-release", true));
        } else if version.draft.unwrap_or(false) {
            fields.push(field("Status", "📝 Draft", true));
        }
        
        // Docker/Container tags if applicable
        if let Some(docker_meta) = &version.docker_metadata {
            if !docker_meta.tags.is_empty() {
                let tags = docker_meta.tags.iter()
                    .take(5)
                    .map(|t| format!("`{}`", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                fields.push(field("🐋 Docker Tags", tags, false));
            }
        }
        
        if let Some(container_meta) = &version.container_metadata {
            if !container_meta.tags.is_empty() {
                let tags = container_meta.tags.iter()
                    .take(5)
                    .map(|t| format!("`{}`", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                fields.push(field("📦 Container Tags", tags, false));
            }
        }
        
        // NPM specific metadata
        if let Some(npm_meta) = &version.npm_metadata {
            if let Some(license) = &npm_meta.license {
                fields.push(field("📜 License", license, true));
            }
            
            if let Some(deps) = &npm_meta.dependencies {
                if let Some(obj) = deps.as_object() {
                    fields.push(field("Dependencies", obj.len().to_string(), true));
                }
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
            if self.action == RegistryPackageAction::Published { "published" } else { "updated" },
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
                text: format!("GitHub Registry • {}", package.registry.name),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}