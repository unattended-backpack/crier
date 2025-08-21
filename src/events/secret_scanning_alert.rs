use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecretScanningAlertEvent {
    pub action: SecretScanningAlertAction,
    pub alert: SecretScanningAlert,
    pub repository: Repository,
    pub sender: Option<User>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertAction {
    Created,
    Resolved,
    Reopened,
    RevokeFailed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SecretScanningAlert {
    pub number: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub url: String,
    pub html_url: String,
    pub locations_url: Option<String>,
    pub state: String,  // "open", "resolved"
    pub resolution: Option<String>,  // "false_positive", "wont_fix", "revoked", "used_in_tests", "pattern_edited", "pattern_deleted"
    pub resolved_at: Option<String>,
    pub resolved_by: Option<User>,
    pub resolution_comment: Option<String>,
    pub secret_type: String,
    pub secret_type_display_name: String,
    pub secret: Option<String>,  // Redacted value
    pub push_protection_bypassed: Option<bool>,
    pub push_protection_bypassed_by: Option<User>,
    pub push_protection_bypassed_at: Option<String>,
}

impl DiscordTransform for SecretScanningAlertEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let alert = &self.alert;
        
        let (color, emoji, action_text) = match self.action {
            SecretScanningAlertAction::Created => (Colors::RED, "🔐🚨", "Secret exposed!"),
            SecretScanningAlertAction::Resolved => (Colors::GREEN, "🔐✅", "Secret resolved"),
            SecretScanningAlertAction::Reopened => (Colors::YELLOW, "🔐🔄", "Secret alert reopened"),
            SecretScanningAlertAction::RevokeFailed => (Colors::RED, "🔐❌", "Secret revocation failed!"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Alert number
        fields.push(field("Alert", format!("#{}", alert.number), true));
        
        // Secret type - PROMINENT DISPLAY
        fields.push(field(
            "Secret Type",
            format!("⚠️ {} ⚠️", alert.secret_type_display_name),
            true
        ));
        
        // State
        let state_display = match alert.state.as_str() {
            "open" => "🔓 OPEN - Action Required!",
            "resolved" => "🔒 Resolved",
            _ => &alert.state,
        };
        fields.push(field("State", state_display, true));
        
        // Resolution if resolved
        if let Some(resolution) = &alert.resolution {
            let resolution_display = match resolution.as_str() {
                "false_positive" => "✓ False Positive",
                "wont_fix" => "⚠️ Won't Fix",
                "revoked" => "🔐 Revoked",
                "used_in_tests" => "🧪 Used in Tests",
                "pattern_edited" => "✏️ Pattern Edited",
                "pattern_deleted" => "🗑️ Pattern Deleted",
                _ => resolution,
            };
            fields.push(field("Resolution", resolution_display, true));
            
            if let Some(comment) = &alert.resolution_comment {
                fields.push(field("Resolution Comment", comment, false));
            }
            
            if let Some(resolved_by) = &alert.resolved_by {
                fields.push(field("Resolved By", &resolved_by.login, true));
            }
        }
        
        // Push protection bypass info
        if let Some(bypassed) = alert.push_protection_bypassed {
            if bypassed {
                fields.push(field(
                    "⚠️ Push Protection",
                    "BYPASSED - Secret was pushed despite protection!",
                    false
                ));
                if let Some(bypassed_by) = &alert.push_protection_bypassed_by {
                    fields.push(field("Bypassed By", &bypassed_by.login, true));
                }
            }
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        // Visibility - important for secrets
        let visibility_icon = if self.repository.private { "🔒" } else { "🌍⚠️" };
        fields.push(field(
            "Repository Visibility",
            format!("{} {}", 
                visibility_icon,
                if self.repository.private { "Private" } else { "PUBLIC - Secret may be exposed!" }
            ),
            true
        ));
        
        let description = match self.action {
            SecretScanningAlertAction::Created => {
                Some(format!(
                    "🚨 A {} has been detected in the repository! Immediate action required to prevent unauthorized access.",
                    alert.secret_type_display_name
                ))
            }
            SecretScanningAlertAction::RevokeFailed => {
                Some(format!(
                    "❌ Failed to revoke the exposed {}! Manual intervention required immediately!",
                    alert.secret_type_display_name
                ))
            }
            _ => Some(format!(
                "Secret scanning alert for {} in repository",
                alert.secret_type_display_name
            ))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(alert.html_url.clone()),
            color,
            author: self.sender.as_ref().map(|s| DiscordAuthor {
                name: s.login.clone(),
                url: Some(s.html_url.clone()),
                icon_url: Some(s.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Secret Scanning • SECURITY ALERT".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}