use serde::{Deserialize, Serialize};
use crate::types::{User, Organization, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MarketplacePurchaseEvent {
    pub action: MarketplacePurchaseAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: User,
    
    // Optional fields
    pub organization: Option<Organization>,
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarketplacePurchaseAction {
    Purchased,
    PendingChange,
    PendingChangeCancelled,
    Changed,
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MarketplacePurchase {
    pub account: User,
    pub billing_cycle: String,
    pub unit_count: i64,
    pub on_free_trial: bool,
    pub free_trial_ends_on: Option<String>,
    pub next_billing_date: Option<String>,
    pub plan: MarketplacePlan,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct MarketplacePlan {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub monthly_price_in_cents: i64,
    pub yearly_price_in_cents: i64,
    pub price_model: String,
    pub has_free_trial: bool,
    pub unit_name: Option<String>,
    pub bullets: Vec<String>,
}

impl DiscordTransform for MarketplacePurchaseEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let (color, emoji, action_text) = match self.action {
            MarketplacePurchaseAction::Purchased => (Colors::GREEN, "💳", "purchased"),
            MarketplacePurchaseAction::PendingChange => (Colors::YELLOW, "⏳", "pending change"),
            MarketplacePurchaseAction::PendingChangeCancelled => (Colors::DULL_YELLOW, "↩️", "change cancelled"),
            MarketplacePurchaseAction::Changed => (Colors::BLUE, "🔄", "changed"),
            MarketplacePurchaseAction::Cancelled => (Colors::RED, "❌", "cancelled"),
        };
        
        let title = format!("{} Marketplace subscription {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Plan name
        fields.push(field("Plan", &self.marketplace_purchase.plan.name, true));
        
        // Pricing
        let price_display = format!(
            "${}/month",
            self.marketplace_purchase.plan.monthly_price_in_cents / 100
        );
        fields.push(field("Price", price_display, true));
        
        // Unit count
        if self.marketplace_purchase.unit_count > 1 {
            fields.push(field("Units", self.marketplace_purchase.unit_count.to_string(), true));
        }
        
        // Billing cycle
        fields.push(field("Billing", &self.marketplace_purchase.billing_cycle, true));
        
        // Free trial
        if self.marketplace_purchase.on_free_trial {
            fields.push(field("Status", "🎁 Free Trial", true));
            if let Some(ends_on) = &self.marketplace_purchase.free_trial_ends_on {
                fields.push(field("Trial Ends", ends_on, true));
            }
        }
        
        // Next billing date
        if let Some(next_date) = &self.marketplace_purchase.next_billing_date {
            fields.push(field("Next Billing", next_date, true));
        }
        
        // Account
        fields.push(field("Account", &self.marketplace_purchase.account.login, true));
        
        let description = Some(self.marketplace_purchase.plan.description.clone());
        
        DiscordEmbed {
            title,
            description,
            url: Some("https://github.com/marketplace".to_string()),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Marketplace".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}