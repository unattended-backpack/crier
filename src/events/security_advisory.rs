use crate::transform::{field, Colors};
use crate::types::{DiscordTransform, Organization, Repository, User};
use crate::{DiscordAuthor, DiscordEmbed, DiscordFooter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityAdvisoryEvent {
    pub action: SecurityAdvisoryAction,
    pub security_advisory: SecurityAdvisory,
    pub sender: Option<User>,
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SecurityAdvisoryAction {
    Published,
    Updated,
    Withdrawn,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecurityAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub summary: String,
    pub description: String,
    pub severity: String, // "low", "moderate", "high", "critical"
    pub author: Option<User>,
    pub publisher: Option<User>,
    pub identifiers: Vec<Identifier>,
    pub state: String, // "published", "withdrawn", "draft"
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub closed_at: Option<String>,
    pub withdrawn_at: Option<String>,
    pub submission: Option<Submission>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub cvss: Option<CVSS>,
    pub cwes: Vec<CWE>,
    pub cwe_ids: Vec<String>,
    pub credits: Vec<Credit>,
    pub credits_detailed: Vec<CreditDetailed>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Identifier {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Submission {
    pub accepted: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vulnerability {
    pub package: VulnerablePackage,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct VulnerablePackage {
    pub ecosystem: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FirstPatchedVersion {
    pub identifier: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CVSS {
    pub vector_string: String,
    pub score: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CWE {
    pub cwe_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Credit {
    pub login: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreditDetailed {
    pub user: User,
    #[serde(rename = "type")]
    pub type_: String,
    pub state: String,
}

impl DiscordTransform for SecurityAdvisoryEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let advisory = &self.security_advisory;

        let (color, emoji, action_text) = match self.action {
            SecurityAdvisoryAction::Published => {
                (Colors::RED, "🚨🔴", "Security Advisory Published")
            }
            SecurityAdvisoryAction::Updated => {
                (Colors::YELLOW, "🚨🟡", "Security Advisory Updated")
            }
            SecurityAdvisoryAction::Withdrawn => {
                (Colors::GRAY, "↩️", "Security Advisory Withdrawn")
            }
        };

        // Extra emphasis for critical severity
        let severity_emoji = match advisory.severity.as_str() {
            "critical" => "🔴🔴🔴 ",
            "high" => "🔴 ",
            "moderate" => "🟡 ",
            "low" => "🟢 ",
            _ => "",
        };

        let title = format!("{}{}{}", severity_emoji, emoji, action_text);

        let mut fields = vec![];

        // Advisory IDs
        fields.push(field("GHSA", &advisory.ghsa_id, true));
        if let Some(cve) = &advisory.cve_id {
            fields.push(field("CVE", cve, true));
        }

        // Severity with emphasis
        let severity_display = match advisory.severity.as_str() {
            "critical" => "🔴 CRITICAL ⚠️⚠️⚠️",
            "high" => "🔴 HIGH ⚠️",
            "moderate" => "🟡 Moderate",
            "low" => "🟢 Low",
            _ => &advisory.severity,
        };
        fields.push(field("Severity", severity_display, true));

        // CVSS Score if available
        if let Some(cvss) = &advisory.cvss {
            fields.push(field(
                "CVSS Score",
                format!("{:.1} ({})", cvss.score, cvss.vector_string),
                true,
            ));
        }

        // State
        let state_display = match advisory.state.as_str() {
            "published" => "📢 Published",
            "withdrawn" => "↩️ Withdrawn",
            "draft" => "📝 Draft",
            _ => &advisory.state,
        };
        fields.push(field("State", state_display, true));

        // Affected packages
        if !advisory.vulnerabilities.is_empty() {
            for vuln in advisory.vulnerabilities.iter().take(3) {
                let package_info = format!(
                    "{} ({}) - Vulnerable: {}{}",
                    vuln.package.name,
                    vuln.package.ecosystem,
                    vuln.vulnerable_version_range,
                    vuln.first_patched_version
                        .as_ref()
                        .map(|v| format!(" | Fixed: {}", v.identifier))
                        .unwrap_or_default()
                );
                fields.push(field("📦 Affected Package", package_info, false));
            }

            if advisory.vulnerabilities.len() > 3 {
                fields.push(field(
                    "Additional Packages",
                    format!("... and {} more", advisory.vulnerabilities.len() - 3),
                    false,
                ));
            }
        }

        // CWEs
        if !advisory.cwes.is_empty() {
            let cwe_list = advisory
                .cwes
                .iter()
                .take(3)
                .map(|cwe| format!("{}: {}", cwe.cwe_id, cwe.name))
                .collect::<Vec<_>>()
                .join("\n");
            fields.push(field("Weaknesses", cwe_list, false));
        }

        // Credits
        if !advisory.credits_detailed.is_empty() {
            let credits = advisory
                .credits_detailed
                .iter()
                .take(5)
                .map(|c| c.user.login.clone())
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(field("🏆 Credits", credits, false));
        }

        // Repository if available
        if let Some(repo) = &self.repository {
            fields.push(field(
                "Repository",
                format!("[{}]({})", repo.full_name, repo.html_url),
                true,
            ));
        }

        // Organization if available
        if let Some(org) = &self.organization {
            fields.push(field("Organization", &org.login, true));
        }

        let description = Some(format!(
            "⚠️ **{}**\n\n{}",
            advisory.summary,
            if advisory.description.len() > 400 {
                format!("{}...", &advisory.description[..397])
            } else {
                advisory.description.clone()
            }
        ));

        DiscordEmbed {
            title,
            description,
            url: Some(advisory.html_url.clone()),
            color,
            author: self.sender.as_ref().map(|s| DiscordAuthor {
                name: s.login.clone(),
                url: Some(s.html_url.clone()),
                icon_url: Some(s.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Security Advisory • IMMEDIATE ATTENTION".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}
