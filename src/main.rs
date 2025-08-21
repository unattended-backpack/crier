use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use crier::{send_to_discord, transform_to_discord};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
struct AppState {
    github_secret: Option<String>,
    event_sender: mpsc::Sender<EventData>,
}

struct EventData {
    event_type: String,
    event_json: serde_json::Value,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crier=info".into()),
        )
        .init();

    let discord_webhook_url =
        std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set");

    let github_secret = std::env::var("GITHUB_WEBHOOK_SECRET").ok();

    if github_secret.is_none() {
        warn!("GITHUB_WEBHOOK_SECRET not set - webhook signatures will not be verified");
    }

    // Create a channel for rate-limited event processing
    let (event_sender, mut event_receiver) = mpsc::channel::<EventData>(100);

    let http_client = reqwest::Client::new();
    let discord_url = discord_webhook_url.clone();
    let client = http_client.clone();

    // Spawn the rate-limited event processor
    tokio::spawn(async move {
        // Process one event per second
        let mut ticker = interval(Duration::from_secs(1));
        
        while let Some(event_data) = event_receiver.recv().await {
            ticker.tick().await; // Wait for the next tick (1 second)
            
            info!("Processing {} event (rate limited to 1/sec)", event_data.event_type);
            
            let discord_message = transform_to_discord(&event_data.event_type, event_data.event_json);
            
            match send_to_discord(&client, &discord_url, &discord_message).await {
                Ok(_) => {
                    info!("Successfully forwarded {} event to Discord", event_data.event_type);
                }
                Err(e) => {
                    error!("Failed to send {} event to Discord: {}", event_data.event_type, e);
                }
            }
        }
    });

    let state = Arc::new(AppState {
        github_secret,
        event_sender,
    });

    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/health", axum::routing::get(|| async { "OK" }))
        .with_state(state);

    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    info!("Starting crier server on {}", addr);
    info!("Rate limiting enabled: 1 event per second");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    if let Some(ref secret) = state.github_secret
        && let Err(e) = verify_signature(&headers, &body, secret)
    {
        error!("Signature verification failed: {}", e);
        return (StatusCode::UNAUTHORIZED, "Invalid signature");
    }

    let event_type = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    info!("Received GitHub event: {}", event_type);

    let github_event: serde_json::Value = match serde_json::from_str(&body) {
        Ok(event) => event,
        Err(e) => {
            error!("Failed to parse GitHub event: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid JSON");
        }
    };

    // Queue the event for rate-limited processing
    let event_data = EventData {
        event_type: event_type.to_string(),
        event_json: github_event,
    };

    match state.event_sender.try_send(event_data) {
        Ok(_) => {
            info!("Event queued for processing");
            (StatusCode::OK, "Event queued")
        }
        Err(mpsc::error::TrySendError::Full(_)) => {
            warn!("Event queue is full, rejecting event");
            (StatusCode::SERVICE_UNAVAILABLE, "Queue full, please retry later")
        }
        Err(mpsc::error::TrySendError::Closed(_)) => {
            error!("Event processor has stopped");
            (StatusCode::INTERNAL_SERVER_ERROR, "Service unavailable")
        }
    }
}

fn verify_signature(headers: &HeaderMap, body: &str, secret: &str) -> Result<(), String> {
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing signature header")?;

    if !signature.starts_with("sha256=") {
        return Err("Invalid signature format".to_string());
    }

    let signature = &signature[7..];

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Failed to create HMAC: {}", e))?;

    mac.update(body.as_bytes());

    let expected = hex::encode(mac.finalize().into_bytes());

    if !constant_time_eq(signature.as_bytes(), expected.as_bytes()) {
        return Err("Signature mismatch".to_string());
    }

    Ok(())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (a_byte, b_byte) in a.iter().zip(b.iter()) {
        result |= a_byte ^ b_byte;
    }

    result == 0
}