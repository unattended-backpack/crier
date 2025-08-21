use serde::{Deserialize, Serialize};

// Module declarations
pub mod types;
pub mod events;
pub mod transform;

// Re-exports for backward compatibility
pub use transform::Colors;

// Discord structures remain the same
#[derive(Serialize)]
pub struct DiscordWebhook {
    pub content: Option<String>,
    pub username: String,
    pub avatar_url: Option<String>,
    pub embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize)]
pub struct DiscordEmbed {
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: u32,
    pub author: Option<DiscordAuthor>,
    pub fields: Vec<DiscordField>,
    pub footer: Option<DiscordFooter>,
    pub timestamp: Option<String>,
}

#[derive(Serialize)]
pub struct DiscordAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct DiscordField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
pub struct DiscordFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

// Main transformation function - now uses the modular system
pub fn transform_to_discord(event_type: &str, event_json: serde_json::Value) -> DiscordWebhook {
    // Use the new type-aware deserialization
    let embed = match events::GitHubEvent::from_json(event_type, event_json.clone()) {
        Ok(event) => {
            // Debug: Check which variant was matched
            match &event {
                events::GitHubEvent::Unknown(_) => {
                    eprintln!("INFO: {} event handled as Unknown variant (not yet implemented)", event_type);
                }
                _ => {
                    // Successfully matched a specific event type
                    eprintln!("Successfully matched {} event to specific type", event_type);
                }
            }
            // Use the event's built-in transform method
            event.transform_to_discord(event_type)
        }
        Err(e) => {
            // This should rarely happen now since we're using type-aware deserialization
            eprintln!("Failed to deserialize {} event: {}", event_type, e);
            eprintln!("Event JSON: {}", serde_json::to_string_pretty(&event_json).unwrap_or_default());
            
            // Try to extract basic information
            let repo_name = event_json.get("repository")
                .and_then(|r| r.get("full_name"))
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");
            
            let sender = event_json.get("sender")
                .and_then(|s| s.get("login"))
                .and_then(|l| l.as_str())
                .unwrap_or("unknown");
            
            DiscordEmbed {
                title: format!("{} event", event_type.replace('_', " ")),
                description: Some(format!("Event in {} by {}", repo_name, sender)),
                url: None,
                color: Colors::GRAY,
                author: None,
                fields: vec![],
                footer: Some(DiscordFooter {
                    text: "GitHub".to_string(),
                    icon_url: Some(
                        "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                            .to_string(),
                    ),
                }),
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            }
        }
    };
    
    DiscordWebhook {
        content: None,
        username: "Crier".to_string(),
        avatar_url: None,
        embeds: vec![embed],
    }
}

// Send to Discord remains the same
pub async fn send_to_discord(
    client: &reqwest::Client,
    webhook_url: &str,
    message: &DiscordWebhook,
) -> Result<(), reqwest::Error> {
    client
        .post(webhook_url)
        .json(message)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}