use crier::{send_to_discord, transform_to_discord, DiscordWebhook};
use std::fs;
use std::path::Path;

async fn test_event(event_type: &str, event_json: serde_json::Value) {
    let discord_message = transform_to_discord(event_type, event_json.clone());
    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL");

    assert_eq!(discord_message.username, "Crier");
    assert!(!discord_message.embeds.is_empty());

    // Send to Discord if we have a webhook URL set
    match discord_webhook_url {
        Ok(url) if !url.is_empty() => {
            let http_client = reqwest::Client::new();
            match send_to_discord(&http_client, &url, &discord_message).await {
                Ok(_) => println!("✓ {} event sent to Discord successfully", event_type),
                Err(e) => eprintln!("✗ Failed to send {} event to Discord: {}", event_type, e),
            }
            // Small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
        _ => {
            println!(
                "✓ {} event processed (set DISCORD_WEBHOOK_URL to send to Discord)",
                event_type
            );
        }
    }
}

fn load_json_file(filename: &str) -> serde_json::Value {
    let path = Path::new("tests/json").join(filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e))
}

fn get_all_events() -> Vec<(&'static str, &'static str)> {
    vec![
        ("push", "push.json"),
        ("pull_request", "pull_request.json"),
        ("pull_request_review", "pull_request_review.json"),
        (
            "pull_request_review_comment",
            "pull_request_review_comment.json",
        ),
        ("issues", "issues.json"),
        ("issue_comment", "issue_comment.json"),
        ("workflow_run", "workflow_run.json"),
        ("workflow_job", "workflow_job.json"),
        ("workflow_dispatch", "workflow_dispatch.json"),
        ("create", "create.json"),
        ("delete", "delete.json"),
        ("fork", "fork.json"),
        ("star", "star.json"),
        ("watch", "watch.json"),
        ("release", "release.json"),
        ("commit_comment", "commit_comment.json"),
        ("repository", "repository.json"),
        ("repository_dispatch", "repository_dispatch.json"),
        ("repository_import", "repository_import.json"),
        ("gollum", "gollum.json"),
        ("public", "public.json"),
        ("ping", "ping.json"),
        ("meta", "meta.json"),
        ("status", "status.json"),
        ("check_run", "check_run.json"),
        ("check_suite", "check_suite.json"),
        ("deployment", "deployment.json"),
        ("deployment_status", "deployment_status.json"),
        ("page_build", "page_build.json"),
        ("team", "team.json"),
        ("team_add", "team_add.json"),
        ("organization", "organization.json"),
        ("member", "member.json"),
        ("membership", "membership.json"),
        ("org_block", "org_block.json"),
        ("code_scanning_alert", "code_scanning_alert.json"),
        ("secret_scanning_alert", "secret_scanning_alert.json"),
        (
            "secret_scanning_alert_location",
            "secret_scanning_alert_location.json",
        ),
        ("dependabot_alert", "dependabot_alert.json"),
        ("security_advisory", "security_advisory.json"),
        (
            "repository_vulnerability_alert",
            "repository_vulnerability_alert.json",
        ),
        ("repository_advisory", "repository_advisory.json"),
        ("security_and_analysis", "security_and_analysis.json"),
        ("branch_protection_rule", "branch_protection_rule.json"),
        (
            "branch_protection_configuration",
            "branch_protection_configuration.json",
        ),
        ("label", "label.json"),
        ("milestone", "milestone.json"),
        ("project", "project.json"),
        ("project_card", "project_card.json"),
        ("project_column", "project_column.json"),
        ("discussion", "discussion.json"),
        ("discussion_comment", "discussion_comment.json"),
        ("merge_group", "merge_group.json"),
        ("installation", "installation.json"),
        (
            "installation_repositories",
            "installation_repositories.json",
        ),
        ("github_app_authorization", "github_app_authorization.json"),
        ("repository_ruleset", "repository_ruleset.json"),
        ("custom_property", "custom_property.json"),
        ("custom_property_values", "custom_property_values.json"),
        ("package", "package.json"),
        ("registry_package", "registry_package.json"),
        ("sponsorship", "sponsorship.json"),
        ("marketplace_purchase", "marketplace_purchase.json"),
        (
            "personal_access_token_request",
            "personal_access_token_request.json",
        ),
    ]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_all_github_events() {
    // Spawn with larger stack size to handle complex JSON deserialization
    let handle = tokio::task::spawn_blocking(move || {
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024) // 8MB stack
            .spawn(move || {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    run_all_tests().await;
                })
            })
            .unwrap()
            .join()
            .unwrap()
    });

    handle.await.unwrap();
}

async fn run_all_tests() {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    println!("Testing ALL GitHub to Discord Event Transformations");
    println!("{}", "=".repeat(60));

    let events = get_all_events();
    let total = events.len();
    let mut passed = 0;
    let mut failed = 0;

    for (event_type, json_file) in events {
        print!("Testing {:30} ", format!("{}...", event_type));

        let event_json = load_json_file(json_file);
        test_event(event_type, event_json).await;
        passed += 1;
    }

    // Test unknown event as fallback
    print!("Testing {:30} ", "unknown_event...");
    let unknown_event = serde_json::json!({
        "action": "mysterious_action",
        "some_field": "some_value",
        "nested": {
            "data": "here"
        }
    });
    test_event("unknown_event", unknown_event).await;
    passed += 1;

    println!(
        "\n✅ Test Results: {}/{} events tested successfully!",
        passed,
        total + 1
    );
    if failed > 0 {
        panic!("❌ {} tests failed!", failed);
    }
}
