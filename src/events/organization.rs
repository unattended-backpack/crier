use serde::{Deserialize, Serialize};
use crate::types::{Organization, User, Installation, DiscordTransform};
use crate::transform::{Colors, field};
use crate::{DiscordEmbed, DiscordAuthor, DiscordFooter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrganizationEvent {
    pub action: OrganizationAction,
    pub organization: Organization,
    pub sender: User,
    pub membership: Option<Membership>,
    pub invitation: Option<Invitation>,
    
    // Optional fields
    pub installation: Option<Installation>,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationAction {
    Deleted,
    Renamed,
    MemberAdded,
    MemberRemoved,
    MemberInvited,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Membership {
    pub url: String,
    pub state: String,  // "active" or "pending"
    pub role: String,   // "admin", "member"
    pub organization_url: String,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Invitation {
    pub id: i64,
    pub login: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub created_at: String,
    pub inviter: User,
    pub team_count: Option<i64>,
    pub invitation_teams_url: Option<String>,
}

impl DiscordTransform for OrganizationEvent {
    fn to_discord_embed(&self, _event_type: &str) -> DiscordEmbed {
        let org = &self.organization;
        
        let (color, emoji, action_text) = match self.action {
            OrganizationAction::Deleted => (Colors::RED, "💀", "deleted"),
            OrganizationAction::Renamed => (Colors::DULL_BLUE, "✏️", "renamed"),
            OrganizationAction::MemberAdded => (Colors::DULL_GREEN, "➕", "member added"),
            OrganizationAction::MemberRemoved => (Colors::DULL_RED, "➖", "member removed"),
            OrganizationAction::MemberInvited => (Colors::DULL_YELLOW, "📧", "member invited"),
        };
        
        let title = format!("{} Organization {}", emoji, action_text);
        
        let mut fields = vec![];
        
        // Organization name
        fields.push(field("Organization", &org.login, true));
        
        // Action specific fields
        match self.action {
            OrganizationAction::MemberAdded | OrganizationAction::MemberRemoved => {
                if let Some(membership) = &self.membership {
                    fields.push(field("Member", &membership.user.login, true));
                    fields.push(field("Role", &membership.role, true));
                    fields.push(field("Status", &membership.state, true));
                }
            }
            OrganizationAction::MemberInvited => {
                if let Some(invitation) = &self.invitation {
                    let invitee = invitation.login.as_ref()
                        .or(invitation.email.as_ref())
                        .map(|s| s.as_str())
                        .unwrap_or("Unknown");
                    fields.push(field("Invited", invitee, true));
                    fields.push(field("Role", &invitation.role, true));
                    fields.push(field("Invited By", &invitation.inviter.login, true));
                    if let Some(team_count) = invitation.team_count {
                        fields.push(field("Teams", team_count.to_string(), true));
                    }
                }
            }
            _ => {}
        }
        
        // Organization details
        if let Some(desc) = &org.description {
            if !desc.is_empty() {
                fields.push(field("Description", desc, false));
            }
        }
        
        // Company
        if let Some(company) = &org.company {
            if !company.is_empty() {
                fields.push(field("Company", company, true));
            }
        }
        
        // Location
        if let Some(location) = &org.location {
            if !location.is_empty() {
                fields.push(field("Location", location, true));
            }
        }
        
        // Email
        if let Some(email) = &org.email {
            if !email.is_empty() {
                fields.push(field("Email", email, true));
            }
        }
        
        // Blog
        if let Some(blog) = &org.blog {
            if !blog.is_empty() {
                fields.push(field("Blog", blog, true));
            }
        }
        
        // Public repos count
        if let Some(public_repos) = org.public_repos {
            fields.push(field("Public Repos", public_repos.to_string(), true));
        }
        
        let description = Some(format!(
            "Organization '{}' was {}",
            org.login, action_text
        ));
        
        DiscordEmbed {
            title,
            description,
            url: org.html_url.clone(),
            color,
            author: Some(DiscordAuthor {
                name: self.sender.login.clone(),
                url: Some(self.sender.html_url.clone()),
                icon_url: Some(self.sender.avatar_url.clone()),
            }),
            fields,
            footer: Some(DiscordFooter {
                text: "GitHub Organization".to_string(),
                icon_url: Some(
                    "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                        .to_string(),
                ),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        }
    }
}