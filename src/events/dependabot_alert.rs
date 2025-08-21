use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DependabotAlertEvent {
    pub action: DependabotAlertAction,
    pub alert: DependabotAlert,
    pub repository: Repository,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DependabotAlertAction {
    Created,
    Dismissed,
    Fixed,
    Reintroduced,
    Reopened,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DependabotAlert {
    pub number: i64,
    pub state: String,  // "open", "dismissed", "fixed"
    pub dependency: DependabotDependency,
    pub security_advisory: SecurityAdvisory,
    pub security_vulnerability: SecurityVulnerability,
    pub url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub dismissed_at: Option<String>,
    pub dismissed_by: Option<User>,
    pub dismissed_reason: Option<String>,
    pub dismissed_comment: Option<String>,
    pub fixed_at: Option<String>,
    pub auto_dismissed_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DependabotDependency {
    pub package: DependabotPackage,
    pub manifest_path: String,
    pub scope: Option<String>,  // "development" or "runtime"
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DependabotPackage {
    pub ecosystem: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub summary: String,
    pub description: String,
    pub severity: String,  // "low", "moderate", "high", "critical"
    pub identifiers: Vec<AdvisoryIdentifier>,
    pub references: Vec<AdvisoryReference>,
    pub published_at: String,
    pub updated_at: String,
    pub withdrawn_at: Option<String>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub cvss: Option<CVSS>,
    pub cwes: Vec<CWE>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AdvisoryIdentifier {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AdvisoryReference {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vulnerability {
    pub package: DependabotPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FirstPatchedVersion {
    pub identifier: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CVSS {
    pub vector_string: Option<String>,
    pub score: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CWE {
    pub cwe_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityVulnerability {
    pub package: DependabotPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

impl DiscordTransform for DependabotAlertEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let alert = &self.alert;
        let advisory = &alert.security_advisory;
        
        let (color, emoji, action_text) = match self.action {
            DependabotAlertAction::Created => (Colors::RED, "🚨", "Vulnerability detected"),
            DependabotAlertAction::Fixed => (Colors::GREEN, "✅", "Vulnerability fixed"),
            DependabotAlertAction::Dismissed => (Colors::DULL_GREEN, "👋", "Alert dismissed"),
            DependabotAlertAction::Reopened => (Colors::YELLOW, "🔄", "Alert reopened"),
            DependabotAlertAction::Reintroduced => (Colors::RED, "⚠️", "Vulnerability reintroduced"),
        };
        
        // Add severity emphasis
        let severity_emoji = match advisory.severity.as_str() {
            "critical" => "🔴🔴🔴 ",
            "high" => "🔴 ",
            "moderate" => "🟡 ",
            "low" => "🟢 ",
            _ => "",
        };
        
        let title = format!("{}{}{}", severity_emoji, emoji, action_text);
        
        let mut fields = vec![];
        
        // Alert number
        fields.push(field("Alert", format!("#{}", alert.number), true));
        
        // Severity with emphasis
        let severity_display = match advisory.severity.as_str() {
            "critical" => "🔴 CRITICAL ⚠️",
            "high" => "🔴 HIGH",
            "moderate" => "🟡 MODERATE",
            "low" => "🟢 LOW",
            _ => &advisory.severity,
        };
        fields.push(field("Severity", severity_display, true));
        
        // Package
        fields.push(field(
            "Package",
            format!("{} ({})",
                alert.dependency.package.name,
                alert.dependency.package.ecosystem
            ),
            true
        ));
        
        // Vulnerable version
        fields.push(field(
            "Vulnerable Versions",
            &alert.security_vulnerability.vulnerable_version_range,
            true
        ));
        
        // Fixed version
        if let Some(fixed) = &alert.security_vulnerability.first_patched_version {
            fields.push(field("Fixed Version", &fixed.identifier, true));
        }
        
        // CVSS Score if available
        if let Some(cvss) = &advisory.cvss {
            fields.push(field("CVSS Score", format!("{:.1}", cvss.score), true));
        }
        
        // Identifiers (GHSA, CVE)
        fields.push(field("GHSA", &advisory.ghsa_id, true));
        if let Some(cve) = &advisory.cve_id {
            fields.push(field("CVE", cve, true));
        }
        
        // Manifest path
        fields.push(code_field("Manifest", &alert.dependency.manifest_path, true));
        
        // Scope
        if let Some(scope) = &alert.dependency.scope {
            let scope_icon = if scope == "runtime" { "⚠️" } else { "🛠️" };
            fields.push(field("Scope", format!("{} {}", scope_icon, scope), true));
        }
        
        // State
        let state_display = match alert.state.as_str() {
            "open" => "🔓 Open",
            "dismissed" => "👋 Dismissed",
            "fixed" => "✅ Fixed",
            _ => &alert.state,
        };
        fields.push(field("State", state_display, true));
        
        // Dismissal info if applicable
        if let Some(dismissed_by) = &alert.dismissed_by {
            fields.push(field("Dismissed By", &dismissed_by.login, true));
            if let Some(reason) = &alert.dismissed_reason {
                fields.push(field("Dismiss Reason", reason, true));
            }
        }
        
        // CWEs if available
        if !advisory.cwes.is_empty() {
            let cwe_list = advisory.cwes.iter()
                .take(3)
                .map(|cwe| format!("{}: {}", cwe.cwe_id, cwe.name))
                .collect::<Vec<_>>()
                .join("\n");
            fields.push(field("Weaknesses", cwe_list, false));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "**{}**\n\n{}",
            advisory.summary,
            if advisory.description.len() > 300 {
                format!("{}...", &advisory.description[..297])
            } else {
                advisory.description.clone()
            }
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(alert.html_url.clone()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Dependabot • Security Vulnerability".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}