use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BranchProtectionRuleEvent {
    pub action: BranchProtectionRuleAction,
    pub rule: BranchProtectionRule,
    pub repository: Repository,
    pub sender: User,
    pub changes: Option<serde_json::Value>,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BranchProtectionRule {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub enforce_admins: Option<bool>,
    pub require_code_owner_reviews: Option<bool>,
    pub dismiss_stale_reviews_on_push: Option<bool>,
    pub require_last_push_approval: Option<bool>,
    pub required_approving_review_count: Option<i64>,
    pub required_status_checks_enforcement_level: Option<String>,
    pub strict_required_status_checks_policy: Option<bool>,
    pub required_deployments_enforcement_level: Option<String>,
    pub allow_force_pushes_enforcement_level: Option<String>,
    pub allow_deletions_enforcement_level: Option<String>,
    pub linear_history_requirement_enforcement_level: Option<String>,
    pub lock_branch_enforcement_level: Option<String>,
    pub allow_fork_syncing: Option<bool>,
    pub lock_allows_fetch_and_merge: Option<bool>,
}

impl DiscordTransform for BranchProtectionRuleEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            BranchProtectionRuleAction::Created => (Colors::GREEN, "🛡️", "created"),
            BranchProtectionRuleAction::Edited => (Colors::YELLOW, "✏️", "edited"),
            BranchProtectionRuleAction::Deleted => (Colors::RED, "⚠️", "deleted"),
        };
        
        let title = format!("{} Branch protection rule {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Branch pattern
        fields.push(field("Branch Pattern", &self.rule.name, true));
        
        // Protection settings
        if let Some(enforce_admins) = self.rule.enforce_admins {
            fields.push(field(
                "Enforce for Admins",
                if enforce_admins { "✅ Yes" } else { "❌ No" },
                true
            ));
        }
        
        if let Some(require_code_owner) = self.rule.require_code_owner_reviews {
            fields.push(field(
                "Require Code Owner Review",
                if require_code_owner { "✅ Yes" } else { "❌ No" },
                true
            ));
        }
        
        if let Some(count) = self.rule.required_approving_review_count {
            fields.push(field("Required Approvals", count.to_string(), true));
        }
        
        if let Some(dismiss_stale) = self.rule.dismiss_stale_reviews_on_push {
            fields.push(field(
                "Dismiss Stale Reviews",
                if dismiss_stale { "✅ Yes" } else { "❌ No" },
                true
            ));
        }
        
        if let Some(last_push) = self.rule.require_last_push_approval {
            fields.push(field(
                "Require Last Push Approval",
                if last_push { "✅ Yes" } else { "❌ No" },
                true
            ));
        }
        
        if let Some(allow_force) = &self.rule.allow_force_pushes_enforcement_level {
            fields.push(field("Allow Force Pushes", allow_force, true));
        }
        
        if let Some(allow_deletions) = &self.rule.allow_deletions_enforcement_level {
            fields.push(field("Allow Deletions", allow_deletions, true));
        }
        
        // Repository
        fields.push(field(
            "Repository",
            format!("[{}]({})", self.repository.full_name, self.repository.html_url),
            true
        ));
        
        let description = Some(format!(
            "Branch protection rule for '{}' was {}",
            self.rule.name, action_text
        ));
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!("{}/settings/branches", self.repository.html_url)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Branch Protection".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}