use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SponsorshipEvent {
    pub action: SponsorshipAction,
    pub sponsorship: Sponsorship,
    pub sender: User,
    pub changes: Option<SponsorshipChanges>,
    pub effective_date: Option<String>,
    
    // Optional fields
    pub repository: Option<Repository>,
    pub organization: Option<Organization>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SponsorshipAction {
    Created,
    Cancelled,
    Edited,
    TierChanged,
    PendingCancellation,
    PendingTierChange,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Sponsorship {
    pub node_id: String,
    pub created_at: String,
    pub sponsorable: Sponsorable,
    pub sponsor: Sponsor,
    pub privacy_level: String,  // "public" or "private"
    pub tier: SponsorshipTier,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Sponsorable {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub type_: String,  // "User" or "Organization"
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Sponsor {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(rename = "type")]
    pub type_: String,  // "User" or "Organization"
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SponsorshipTier {
    pub node_id: String,
    pub created_at: String,
    pub description: String,
    pub monthly_price_in_cents: i64,
    pub monthly_price_in_dollars: i64,
    pub name: String,
    pub is_one_time: bool,
    pub is_custom_amount: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SponsorshipChanges {
    pub tier: Option<TierChange>,
    pub privacy_level: Option<PrivacyChange>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TierChange {
    pub from: SponsorshipTier,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PrivacyChange {
    pub from: String,
}

impl DiscordTransform for SponsorshipEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let sponsorship = &self.sponsorship;
        let tier = &sponsorship.tier;
        
        let (color, emoji, action_text) = match self.action {
            SponsorshipAction::Created => (Colors::GOLD, "💖", "New sponsorship started!"),
            SponsorshipAction::Cancelled => (Colors::DULL_RED, "💔", "Sponsorship cancelled"),
            SponsorshipAction::Edited => (Colors::DULL_BLUE, "✏️", "Sponsorship edited"),
            SponsorshipAction::TierChanged => (Colors::DULL_GREEN, "📈", "Sponsorship tier changed"),
            SponsorshipAction::PendingCancellation => (Colors::YELLOW, "⏳", "Sponsorship pending cancellation"),
            SponsorshipAction::PendingTierChange => (Colors::DULL_YELLOW, "⏳", "Tier change pending"),
        };
        
        let title = format!("{} {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Sponsor
        let sponsor_type_icon = if sponsorship.sponsor.type_ == "Organization" { "🏢" } else { "👤" };
        fields.push(field(
            "Sponsor",
            format!("{} [{}]({})",
                sponsor_type_icon,
                sponsorship.sponsor.login,
                sponsorship.sponsor.html_url
            ),
            true
        ));
        
        // Sponsored
        let sponsored_type_icon = if sponsorship.sponsorable.type_ == "Organization" { "🏢" } else { "👤" };
        fields.push(field(
            "Sponsored",
            format!("{} [{}]({})",
                sponsored_type_icon,
                sponsorship.sponsorable.login,
                sponsorship.sponsorable.html_url
            ),
            true
        ));
        
        // Tier information
        fields.push(field("Tier", &tier.name, true));
        
        // Amount
        let amount_str = if tier.is_one_time {
            format!("${} (one-time)", tier.monthly_price_in_dollars)
        } else {
            format!("${}/month", tier.monthly_price_in_dollars)
        };
        fields.push(field("💵 Amount", amount_str, true));
        
        // Tier description
        if !tier.description.is_empty() {
            fields.push(field("Description", &tier.description, false));
        }
        
        // Privacy level
        let privacy_icon = if sponsorship.privacy_level == "public" { "🌍" } else { "🔒" };
        fields.push(field(
            "Visibility",
            format!("{} {}", privacy_icon, sponsorship.privacy_level),
            true
        ));
        
        // Custom amount indicator
        if tier.is_custom_amount {
            fields.push(field("Custom Amount", "✅ Yes", true));
        }
        
        // Changes for tier change or edit
        if let Some(changes) = &self.changes {
            if let Some(tier_change) = &changes.tier {
                fields.push(field(
                    "Previous Tier",
                    format!("{} (${}/month)",
                        tier_change.from.name,
                        tier_change.from.monthly_price_in_dollars
                    ),
                    false
                ));
                
                let diff = tier.monthly_price_in_dollars - tier_change.from.monthly_price_in_dollars;
                let diff_str = if diff > 0 {
                    format!("+${}", diff)
                } else {
                    format!("${}", diff)
                };
                fields.push(field("Change", diff_str, true));
            }
            
            if let Some(privacy_change) = &changes.privacy_level {
                fields.push(field(
                    "Privacy Changed",
                    format!("{} → {}", privacy_change.from, sponsorship.privacy_level),
                    true
                ));
            }
        }
        
        // Effective date for pending changes
        if let Some(date) = &self.effective_date {
            fields.push(field("📅 Effective Date", date, true));
        }
        
        let description = match self.action {
            SponsorshipAction::Created => Some(format!(
                "🎉 {} is now sponsoring {} with the {} tier!",
                sponsorship.sponsor.login,
                sponsorship.sponsorable.login,
                tier.name
            )),
            SponsorshipAction::Cancelled => Some(format!(
                "{} has cancelled their sponsorship of {}",
                sponsorship.sponsor.login,
                sponsorship.sponsorable.login
            )),
            _ => Some(format!(
                "Sponsorship from {} to {} has been updated",
                sponsorship.sponsor.login,
                sponsorship.sponsorable.login
            ))
        };
        
        DiscordEmbed {
            title,
            description,
            url: Some(format!("https://github.com/sponsors/{}", sponsorship.sponsorable.login)),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Sponsors".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}