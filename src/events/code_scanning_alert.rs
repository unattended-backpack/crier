use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field, code_field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningAlertEvent {
    pub action: CodeScanningAlertAction,
    pub alert: CodeScanningAlert,
    pub repository: Repository,
    pub sender: User,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub commit_oid: String,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningAlertAction {
    AppearedInBranch,
    ClosedByUser,
    Created,
    Fixed,
    Reopened,
    ReopenedByUser,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningAlert {
    pub number: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub url: String,
    pub html_url: String,
    pub state: String,  // "open", "closed", "dismissed", "fixed"
    pub fixed_at: Option<String>,
    pub dismissed_by: Option<User>,
    pub dismissed_at: Option<String>,
    pub dismissed_reason: Option<String>,
    pub dismissed_comment: Option<String>,
    pub rule: CodeScanningRule,
    pub tool: CodeScanningTool,
    pub most_recent_instance: CodeScanningInstance,
    pub instances_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningRule {
    pub id: String,
    pub severity: String,  // "error", "warning", "note"
    pub description: String,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub security_severity_level: Option<String>,  // "critical", "high", "medium", "low"
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningTool {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningInstance {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub analysis_key: String,
    pub environment: String,
    pub category: Option<String>,
    pub state: String,
    pub commit_sha: String,
    pub message: Option<CodeScanningMessage>,
    pub location: Option<CodeScanningLocation>,
    pub classifications: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningMessage {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeScanningLocation {
    pub path: String,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    pub start_column: Option<i64>,
    pub end_column: Option<i64>,
}

impl DiscordTransform for CodeScanningAlertEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let alert = &self.alert;
        
        let (color, emoji, action_text) = match self.action {
            CodeScanningAlertAction::Created => (Colors::RED, "🚨", "Security alert created"),
            CodeScanningAlertAction::AppearedInBranch => (Colors::YELLOW, "⚠️", "Alert appeared in branch"),
            CodeScanningAlertAction::Fixed => (Colors::GREEN, "✅", "Security alert fixed"),
            CodeScanningAlertAction::ClosedByUser => (Colors::DULL_GREEN, "✓", "Alert closed by user"),
            CodeScanningAlertAction::Reopened => (Colors::YELLOW, "🔄", "Alert reopened"),
            CodeScanningAlertAction::ReopenedByUser => (Colors::YELLOW, "👤", "Alert reopened by user"),
        };
        
        // Add extra emphasis for critical/high severity
        let severity_emoji = match alert.rule.security_severity_level.as_deref() {
            Some("critical") => "🔴🔴🔴 ",
            Some("high") => "🔴 ",
            Some("medium") => "🟡 ",
            Some("low") => "🟢 ",
            _ => "",
        };
        
        let title = format!("{}{}{}", severity_emoji, emoji, action_text);
        
        let mut fields = vec![];
        
        // Alert number
        fields.push(field("Alert", format!("#{}", alert.number), true));
        
        // Severity
        if let Some(severity) = &alert.rule.security_severity_level {
            let severity_display = format!("{} {}", 
                match severity.as_str() {
                    "critical" => "🔴 CRITICAL",
                    "high" => "🔴 HIGH",
                    "medium" => "🟡 MEDIUM",
                    "low" => "🟢 LOW",
                    _ => severity,
                },
                if severity == "critical" || severity == "high" { "⚠️" } else { "" }
            );
            fields.push(field("Severity", severity_display, true));
        }
        
        // Rule
        fields.push(field("Rule", &alert.rule.id, true));
        
        // Tool
        let tool_info = if let Some(version) = &alert.tool.version {
            format!("{} v{}", alert.tool.name, version)
        } else {
            alert.tool.name.clone()
        };
        fields.push(field("Scanner", tool_info, true));
        
        // State
        let state_display = match alert.state.as_str() {
            "open" => "🔓 Open",
            "closed" => "🔒 Closed",
            "dismissed" => "👋 Dismissed",
            "fixed" => "✅ Fixed",
            _ => &alert.state,
        };
        fields.push(field("State", state_display, true));
        
        // Branch
        fields.push(code_field("Branch", &self.ref_, true));
        
        // Location if available
        let instance = &alert.most_recent_instance;
        if let Some(location) = &instance.location {
            let location_str = if let (Some(start_line), Some(end_line)) = (location.start_line, location.end_line) {
                format!("{}:{}-{}", location.path, start_line, end_line)
            } else if let Some(start_line) = location.start_line {
                format!("{}:{}", location.path, start_line)
            } else {
                location.path.clone()
            };
            fields.push(code_field("Location", location_str, false));
        }
        
        // Description
        fields.push(field("Description", &alert.rule.description, false));
        
        // Tags if available
        if let Some(tags) = &alert.rule.tags {
            if !tags.is_empty() {
                fields.push(field("Tags", tags.join(", "), false));
            }
        }
        
        // Dismissed info if applicable
        if let Some(dismissed_by) = &alert.dismissed_by {
            fields.push(field("Dismissed By", &dismissed_by.login, true));
            if let Some(reason) = &alert.dismissed_reason {
                fields.push(field("Dismiss Reason", reason, true));
            }
            if let Some(comment) = &alert.dismissed_comment {
                fields.push(field("Dismiss Comment", comment, false));
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = if let Some(msg) = &instance.message {
            Some(msg.text.clone())
        } else {
            Some(format!("Security vulnerability detected: {}", alert.rule.description))
        };
        
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
                text: format!("GitHub Code Scanning • {}", alert.tool.name),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}