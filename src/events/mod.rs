// Event modules - one per GitHub webhook event type
use serde::{Deserialize, Serialize};
use crate::types::{Repository, User, Organization, Installation};
use crate::DiscordEmbed;

pub mod push;
pub mod pull_request;
pub mod pull_request_review;
pub mod pull_request_review_comment;
pub mod issues;
pub mod issue_comment;
pub mod workflow_run;
pub mod workflow_job;
pub mod workflow_dispatch;
pub mod create;
pub mod delete;
pub mod fork;
pub mod star;
pub mod watch;
pub mod release;
pub mod commit_comment;
pub mod repository;
pub mod gollum;
pub mod public;
pub mod ping;
pub mod status;
pub mod milestone;
pub mod label;
pub mod deployment;
pub mod deployment_status;
pub mod merge_group;
pub mod page_build;
pub mod meta;
pub mod repository_dispatch;
pub mod check_run;
pub mod check_suite;
pub mod team;
pub mod organization;
pub mod member;
pub mod membership;
pub mod code_scanning_alert;
pub mod secret_scanning_alert;
pub mod dependabot_alert;
pub mod project;
pub mod discussion;
pub mod installation;
pub mod installation_repositories;
pub mod sponsorship;
pub mod package;
pub mod team_add;
pub mod security_advisory;
pub mod github_app_authorization;
pub mod registry_package;
pub mod project_card;
pub mod project_column;
pub mod discussion_comment;
pub mod branch_protection_rule;
pub mod branch_protection_configuration;
pub mod repository_import;
pub mod repository_vulnerability_alert;
pub mod repository_advisory;
pub mod repository_ruleset;
pub mod marketplace_purchase;
pub mod org_block;
pub mod personal_access_token_request;
pub mod custom_property;
pub mod custom_property_values;
pub mod security_and_analysis;
pub mod secret_scanning_alert_location;
pub mod secret_scanning_scan;

pub use push::PushEvent;
pub use pull_request::PullRequestEvent;
pub use pull_request_review::PullRequestReviewEvent;
pub use pull_request_review_comment::PullRequestReviewCommentEvent;
pub use issues::IssuesEvent;
pub use issue_comment::IssueCommentEvent;
pub use workflow_run::WorkflowRunEvent;
pub use workflow_job::WorkflowJobEvent;
pub use workflow_dispatch::WorkflowDispatchEvent;
pub use create::CreateEvent;
pub use delete::DeleteEvent;
pub use fork::ForkEvent;
pub use star::StarEvent;
pub use watch::WatchEvent;
pub use release::ReleaseEvent;
pub use commit_comment::CommitCommentEvent;
pub use repository::RepositoryEvent;
pub use gollum::GollumEvent;
pub use public::PublicEvent;
pub use ping::PingEvent;
pub use status::StatusEvent;
pub use milestone::MilestoneEvent;
pub use label::LabelEvent;
pub use deployment::DeploymentEvent;
pub use deployment_status::DeploymentStatusEvent;
pub use merge_group::MergeGroupEvent;
pub use page_build::PageBuildEvent;
pub use meta::MetaEvent;
pub use repository_dispatch::RepositoryDispatchEvent;
pub use check_run::CheckRunEvent;
pub use check_suite::CheckSuiteEvent;
pub use team::TeamEvent;
pub use organization::OrganizationEvent;
pub use member::MemberEvent;
pub use membership::MembershipEvent;
pub use code_scanning_alert::CodeScanningAlertEvent;
pub use secret_scanning_alert::SecretScanningAlertEvent;
pub use dependabot_alert::DependabotAlertEvent;
pub use project::ProjectEvent;
pub use discussion::DiscussionEvent;
pub use installation::InstallationEvent;
pub use installation_repositories::InstallationRepositoriesEvent;
pub use sponsorship::SponsorshipEvent;
pub use package::PackageEvent;
pub use team_add::TeamAddEvent;
pub use security_advisory::SecurityAdvisoryEvent;
pub use github_app_authorization::GitHubAppAuthorizationEvent;
pub use registry_package::RegistryPackageEvent;
pub use project_card::ProjectCardEvent;
pub use project_column::ProjectColumnEvent;
pub use discussion_comment::DiscussionCommentEvent;
pub use branch_protection_rule::BranchProtectionRuleEvent;
pub use branch_protection_configuration::BranchProtectionConfigurationEvent;
pub use repository_import::RepositoryImportEvent;
pub use repository_vulnerability_alert::RepositoryVulnerabilityAlertEvent;
pub use repository_advisory::RepositoryAdvisoryEvent;
pub use repository_ruleset::RepositoryRulesetEvent;
pub use marketplace_purchase::MarketplacePurchaseEvent;
pub use org_block::OrgBlockEvent;
pub use personal_access_token_request::PersonalAccessTokenRequestEvent;
pub use custom_property::CustomPropertyEvent;
pub use custom_property_values::CustomPropertyValuesEvent;
pub use security_and_analysis::SecurityAndAnalysisEvent;
pub use secret_scanning_alert_location::SecretScanningAlertLocationEvent;
pub use secret_scanning_scan::SecretScanningScanEvent;

// Master enum of all GitHub webhook events
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum GitHubEvent {
    // Events with more specific fields first
    Push(PushEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    PullRequestReviewComment(PullRequestReviewCommentEvent),
    Issues(IssuesEvent),
    IssueComment(IssueCommentEvent),
    WorkflowRun(WorkflowRunEvent),
    WorkflowJob(WorkflowJobEvent),
    WorkflowDispatch(WorkflowDispatchEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    Fork(ForkEvent),
    Watch(WatchEvent),
    Release(ReleaseEvent),
    Gollum(GollumEvent),
    Ping(PingEvent),
    Status(StatusEvent),
    Milestone(MilestoneEvent),
    Label(LabelEvent),
    Deployment(DeploymentEvent),
    DeploymentStatus(DeploymentStatusEvent),
    MergeGroup(MergeGroupEvent),
    PageBuild(PageBuildEvent),
    Meta(MetaEvent),
    RepositoryDispatch(RepositoryDispatchEvent),
    CheckRun(CheckRunEvent),
    CheckSuite(CheckSuiteEvent),
    Team(TeamEvent),
    Organization(OrganizationEvent),
    Member(MemberEvent),
    Membership(MembershipEvent),
    CodeScanningAlert(CodeScanningAlertEvent),
    SecretScanningAlert(SecretScanningAlertEvent),
    DependabotAlert(DependabotAlertEvent),
    Project(ProjectEvent),
    Discussion(DiscussionEvent),
    Installation(InstallationEvent),
    InstallationRepositories(InstallationRepositoriesEvent),
    Sponsorship(SponsorshipEvent),
    Package(PackageEvent),
    TeamAdd(TeamAddEvent),
    SecurityAdvisory(SecurityAdvisoryEvent),
    GitHubAppAuthorization(GitHubAppAuthorizationEvent),
    RegistryPackage(RegistryPackageEvent),
    ProjectCard(ProjectCardEvent),
    ProjectColumn(ProjectColumnEvent),
    DiscussionComment(DiscussionCommentEvent),
    BranchProtectionRule(BranchProtectionRuleEvent),
    RepositoryImport(RepositoryImportEvent),
    RepositoryVulnerabilityAlert(RepositoryVulnerabilityAlertEvent),
    RepositoryAdvisory(RepositoryAdvisoryEvent),
    MarketplacePurchase(MarketplacePurchaseEvent),
    OrgBlock(OrgBlockEvent),
    PersonalAccessTokenRequest(PersonalAccessTokenRequestEvent),
    CustomPropertyValues(CustomPropertyValuesEvent),
    SecurityAndAnalysis(SecurityAndAnalysisEvent),
    SecretScanningAlertLocation(SecretScanningAlertLocationEvent),
    SecretScanningScan(SecretScanningScanEvent),
    
    // Events with minimal required fields last to prevent false matches
    CommitComment(CommitCommentEvent),
    Repository(RepositoryEvent),
    CustomProperty(CustomPropertyEvent),
    Star(StarEvent),
    RepositoryRuleset(RepositoryRulesetEvent),
    BranchProtectionConfiguration(BranchProtectionConfigurationEvent),
    Public(PublicEvent),
    
    // Catch-all for events we haven't implemented yet
    Unknown(serde_json::Value),
}

impl GitHubEvent {
    /// Deserialize a GitHub event based on the event type from the header
    pub fn from_json(event_type: &str, json: serde_json::Value) -> Result<Self, serde_json::Error> {
        match event_type {
            "push" => serde_json::from_value::<PushEvent>(json).map(GitHubEvent::Push),
            "pull_request" => serde_json::from_value::<PullRequestEvent>(json).map(GitHubEvent::PullRequest),
            "pull_request_review" => serde_json::from_value::<PullRequestReviewEvent>(json).map(GitHubEvent::PullRequestReview),
            "pull_request_review_comment" => serde_json::from_value::<PullRequestReviewCommentEvent>(json).map(GitHubEvent::PullRequestReviewComment),
            // "pull_request_review_thread" => serde_json::from_value::<PullRequestReviewThreadEvent>(json).map(GitHubEvent::PullRequestReviewThread),
            "issues" => serde_json::from_value::<IssuesEvent>(json).map(GitHubEvent::Issues),
            "issue_comment" => serde_json::from_value::<IssueCommentEvent>(json).map(GitHubEvent::IssueComment),
            "workflow_run" => serde_json::from_value::<WorkflowRunEvent>(json).map(GitHubEvent::WorkflowRun),
            "workflow_job" => serde_json::from_value::<WorkflowJobEvent>(json).map(GitHubEvent::WorkflowJob),
            "workflow_dispatch" => serde_json::from_value::<WorkflowDispatchEvent>(json).map(GitHubEvent::WorkflowDispatch),
            "create" => serde_json::from_value::<CreateEvent>(json).map(GitHubEvent::Create),
            "delete" => serde_json::from_value::<DeleteEvent>(json).map(GitHubEvent::Delete),
            "fork" => serde_json::from_value::<ForkEvent>(json).map(GitHubEvent::Fork),
            "watch" => serde_json::from_value::<WatchEvent>(json).map(GitHubEvent::Watch),
            "star" => serde_json::from_value::<StarEvent>(json).map(GitHubEvent::Star),
            "release" => serde_json::from_value::<ReleaseEvent>(json).map(GitHubEvent::Release),
            "gollum" => serde_json::from_value::<GollumEvent>(json).map(GitHubEvent::Gollum),
            "ping" => serde_json::from_value::<PingEvent>(json).map(GitHubEvent::Ping),
            "status" => serde_json::from_value::<StatusEvent>(json).map(GitHubEvent::Status),
            "deployment" => serde_json::from_value::<DeploymentEvent>(json).map(GitHubEvent::Deployment),
            "deployment_status" => serde_json::from_value::<DeploymentStatusEvent>(json).map(GitHubEvent::DeploymentStatus),
            // "deployment_review" => serde_json::from_value::<DeploymentReviewEvent>(json).map(GitHubEvent::DeploymentReview),
            "page_build" => serde_json::from_value::<PageBuildEvent>(json).map(GitHubEvent::PageBuild),
            "public" => serde_json::from_value::<PublicEvent>(json).map(GitHubEvent::Public),
            "repository" => serde_json::from_value::<RepositoryEvent>(json).map(GitHubEvent::Repository),
            "repository_dispatch" => serde_json::from_value::<RepositoryDispatchEvent>(json).map(GitHubEvent::RepositoryDispatch),
            "team" => serde_json::from_value::<TeamEvent>(json).map(GitHubEvent::Team),
            "team_add" => serde_json::from_value::<TeamAddEvent>(json).map(GitHubEvent::TeamAdd),
            "milestone" => serde_json::from_value::<MilestoneEvent>(json).map(GitHubEvent::Milestone),
            "label" => serde_json::from_value::<LabelEvent>(json).map(GitHubEvent::Label),
            "member" => serde_json::from_value::<MemberEvent>(json).map(GitHubEvent::Member),
            "membership" => serde_json::from_value::<MembershipEvent>(json).map(GitHubEvent::Membership),
            "organization" => serde_json::from_value::<OrganizationEvent>(json).map(GitHubEvent::Organization),
            "org_block" => serde_json::from_value::<OrgBlockEvent>(json).map(GitHubEvent::OrgBlock),
            "project" => serde_json::from_value::<ProjectEvent>(json).map(GitHubEvent::Project),
            "project_card" => serde_json::from_value::<ProjectCardEvent>(json).map(GitHubEvent::ProjectCard),
            "project_column" => serde_json::from_value::<ProjectColumnEvent>(json).map(GitHubEvent::ProjectColumn),
            // "projects_v2" => serde_json::from_value::<ProjectsV2Event>(json).map(GitHubEvent::ProjectsV2),
            // "projects_v2_item" => serde_json::from_value::<ProjectsV2ItemEvent>(json).map(GitHubEvent::ProjectsV2Item),
            "check_run" => serde_json::from_value::<CheckRunEvent>(json).map(GitHubEvent::CheckRun),
            "check_suite" => serde_json::from_value::<CheckSuiteEvent>(json).map(GitHubEvent::CheckSuite),
            "code_scanning_alert" => serde_json::from_value::<CodeScanningAlertEvent>(json).map(GitHubEvent::CodeScanningAlert),
            "commit_comment" => serde_json::from_value::<CommitCommentEvent>(json).map(GitHubEvent::CommitComment),
            "discussion" => serde_json::from_value::<DiscussionEvent>(json).map(GitHubEvent::Discussion),
            "discussion_comment" => serde_json::from_value::<DiscussionCommentEvent>(json).map(GitHubEvent::DiscussionComment),
            "branch_protection_rule" => serde_json::from_value::<BranchProtectionRuleEvent>(json).map(GitHubEvent::BranchProtectionRule),
            "branch_protection_configuration" => serde_json::from_value::<BranchProtectionConfigurationEvent>(json).map(GitHubEvent::BranchProtectionConfiguration),
            "dependabot_alert" => serde_json::from_value::<DependabotAlertEvent>(json).map(GitHubEvent::DependabotAlert),
            "installation" => serde_json::from_value::<InstallationEvent>(json).map(GitHubEvent::Installation),
            "installation_repositories" => serde_json::from_value::<InstallationRepositoriesEvent>(json).map(GitHubEvent::InstallationRepositories),
            // "installation_target" => serde_json::from_value::<InstallationTargetEvent>(json).map(GitHubEvent::InstallationTarget),
            "marketplace_purchase" => serde_json::from_value::<MarketplacePurchaseEvent>(json).map(GitHubEvent::MarketplacePurchase),
            "meta" => serde_json::from_value::<MetaEvent>(json).map(GitHubEvent::Meta),
            "package" => serde_json::from_value::<PackageEvent>(json).map(GitHubEvent::Package),
            "registry_package" => serde_json::from_value::<RegistryPackageEvent>(json).map(GitHubEvent::RegistryPackage),
            "repository_import" => serde_json::from_value::<RepositoryImportEvent>(json).map(GitHubEvent::RepositoryImport),
            "repository_vulnerability_alert" => serde_json::from_value::<RepositoryVulnerabilityAlertEvent>(json).map(GitHubEvent::RepositoryVulnerabilityAlert),
            "repository_advisory" => serde_json::from_value::<RepositoryAdvisoryEvent>(json).map(GitHubEvent::RepositoryAdvisory),
            "secret_scanning_alert" => serde_json::from_value::<SecretScanningAlertEvent>(json).map(GitHubEvent::SecretScanningAlert),
            "secret_scanning_alert_location" => serde_json::from_value::<SecretScanningAlertLocationEvent>(json).map(GitHubEvent::SecretScanningAlertLocation),
            "secret_scanning_scan" => serde_json::from_value::<SecretScanningScanEvent>(json).map(GitHubEvent::SecretScanningScan),
            "security_advisory" => serde_json::from_value::<SecurityAdvisoryEvent>(json).map(GitHubEvent::SecurityAdvisory),
            "security_and_analysis" => serde_json::from_value::<SecurityAndAnalysisEvent>(json).map(GitHubEvent::SecurityAndAnalysis),
            "sponsorship" => serde_json::from_value::<SponsorshipEvent>(json).map(GitHubEvent::Sponsorship),
            "github_app_authorization" => serde_json::from_value::<GitHubAppAuthorizationEvent>(json).map(GitHubEvent::GitHubAppAuthorization),
            "personal_access_token_request" => serde_json::from_value::<PersonalAccessTokenRequestEvent>(json).map(GitHubEvent::PersonalAccessTokenRequest),
            "repository_ruleset" => serde_json::from_value::<RepositoryRulesetEvent>(json).map(GitHubEvent::RepositoryRuleset),
            "custom_property" => serde_json::from_value::<CustomPropertyEvent>(json).map(GitHubEvent::CustomProperty),
            "custom_property_values" => serde_json::from_value::<CustomPropertyValuesEvent>(json).map(GitHubEvent::CustomPropertyValues),
            "merge_group" => serde_json::from_value::<MergeGroupEvent>(json).map(GitHubEvent::MergeGroup),
            _ => Ok(GitHubEvent::Unknown(json)),
        }
    }

    pub fn transform_to_discord(&self, event_type: &str) -> DiscordEmbed {
        use crate::types::DiscordTransform;
        
        match self {
            GitHubEvent::Push(e) => e.to_discord_embed(event_type),
            GitHubEvent::PullRequest(e) => e.to_discord_embed(event_type),
            GitHubEvent::PullRequestReview(e) => e.to_discord_embed(event_type),
            GitHubEvent::PullRequestReviewComment(e) => e.to_discord_embed(event_type),
            GitHubEvent::Issues(e) => e.to_discord_embed(event_type),
            GitHubEvent::IssueComment(e) => e.to_discord_embed(event_type),
            GitHubEvent::WorkflowRun(e) => e.to_discord_embed(event_type),
            GitHubEvent::WorkflowJob(e) => e.to_discord_embed(event_type),
            GitHubEvent::WorkflowDispatch(e) => e.to_discord_embed(event_type),
            GitHubEvent::Create(e) => e.to_discord_embed(event_type),
            GitHubEvent::Delete(e) => e.to_discord_embed(event_type),
            GitHubEvent::Fork(e) => e.to_discord_embed(event_type),
            GitHubEvent::Star(e) => e.to_discord_embed(event_type),
            GitHubEvent::Watch(e) => e.to_discord_embed(event_type),
            GitHubEvent::Release(e) => e.to_discord_embed(event_type),
            GitHubEvent::CommitComment(e) => e.to_discord_embed(event_type),
            GitHubEvent::Repository(e) => e.to_discord_embed(event_type),
            GitHubEvent::Gollum(e) => e.to_discord_embed(event_type),
            GitHubEvent::Ping(e) => e.to_discord_embed(event_type),
            GitHubEvent::Status(e) => e.to_discord_embed(event_type),
            GitHubEvent::Milestone(e) => e.to_discord_embed(event_type),
            GitHubEvent::Label(e) => e.to_discord_embed(event_type),
            GitHubEvent::Deployment(e) => e.to_discord_embed(event_type),
            GitHubEvent::DeploymentStatus(e) => e.to_discord_embed(event_type),
            GitHubEvent::MergeGroup(e) => e.to_discord_embed(event_type),
            GitHubEvent::PageBuild(e) => e.to_discord_embed(event_type),
            GitHubEvent::Meta(e) => e.to_discord_embed(event_type),
            GitHubEvent::RepositoryDispatch(e) => e.to_discord_embed(event_type),
            GitHubEvent::CheckRun(e) => e.to_discord_embed(event_type),
            GitHubEvent::CheckSuite(e) => e.to_discord_embed(event_type),
            GitHubEvent::Team(e) => e.to_discord_embed(event_type),
            GitHubEvent::Organization(e) => e.to_discord_embed(event_type),
            GitHubEvent::Member(e) => e.to_discord_embed(event_type),
            GitHubEvent::Membership(e) => e.to_discord_embed(event_type),
            GitHubEvent::CodeScanningAlert(e) => e.to_discord_embed(event_type),
            GitHubEvent::SecretScanningAlert(e) => e.to_discord_embed(event_type),
            GitHubEvent::DependabotAlert(e) => e.to_discord_embed(event_type),
            GitHubEvent::Project(e) => e.to_discord_embed(event_type),
            GitHubEvent::Discussion(e) => e.to_discord_embed(event_type),
            GitHubEvent::Installation(e) => e.to_discord_embed(event_type),
            GitHubEvent::InstallationRepositories(e) => e.to_discord_embed(event_type),
            GitHubEvent::Sponsorship(e) => e.to_discord_embed(event_type),
            GitHubEvent::Package(e) => e.to_discord_embed(event_type),
            GitHubEvent::TeamAdd(e) => e.to_discord_embed(event_type),
            GitHubEvent::SecurityAdvisory(e) => e.to_discord_embed(event_type),
            GitHubEvent::GitHubAppAuthorization(e) => e.to_discord_embed(event_type),
            GitHubEvent::RegistryPackage(e) => e.to_discord_embed(event_type),
            GitHubEvent::ProjectCard(e) => e.to_discord_embed(event_type),
            GitHubEvent::ProjectColumn(e) => e.to_discord_embed(event_type),
            GitHubEvent::DiscussionComment(e) => e.to_discord_embed(event_type),
            GitHubEvent::BranchProtectionRule(e) => e.to_discord_embed(event_type),
            GitHubEvent::BranchProtectionConfiguration(e) => e.to_discord_embed(event_type),
            GitHubEvent::RepositoryImport(e) => e.to_discord_embed(event_type),
            GitHubEvent::RepositoryVulnerabilityAlert(e) => e.to_discord_embed(event_type),
            GitHubEvent::RepositoryAdvisory(e) => e.to_discord_embed(event_type),
            GitHubEvent::RepositoryRuleset(e) => e.to_discord_embed(event_type),
            GitHubEvent::MarketplacePurchase(e) => e.to_discord_embed(event_type),
            GitHubEvent::OrgBlock(e) => e.to_discord_embed(event_type),
            GitHubEvent::PersonalAccessTokenRequest(e) => e.to_discord_embed(event_type),
            GitHubEvent::CustomProperty(e) => e.to_discord_embed(event_type),
            GitHubEvent::CustomPropertyValues(e) => e.to_discord_embed(event_type),
            GitHubEvent::SecurityAndAnalysis(e) => e.to_discord_embed(event_type),
            GitHubEvent::SecretScanningAlertLocation(e) => e.to_discord_embed(event_type),
            GitHubEvent::SecretScanningScan(e) => e.to_discord_embed(event_type),
            GitHubEvent::Public(e) => e.to_discord_embed(event_type),
            GitHubEvent::Unknown(v) => {
                // Log the full event data for unknown events
                eprintln!("===============================================");
                eprintln!("UNKNOWN EVENT RECEIVED: {}", event_type);
                eprintln!("===============================================");
                eprintln!("Full event JSON schema:");
                eprintln!("{}", serde_json::to_string_pretty(&v).unwrap_or_else(|e| {
                    format!("Failed to serialize JSON: {}", e)
                }));
                eprintln!("===============================================");
                
                // Extract basic info for Discord display
                let repo_name = v.get("repository")
                    .and_then(|r| r.get("full_name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown");
                
                let sender = v.get("sender")
                    .and_then(|s| s.get("login"))
                    .and_then(|l| l.as_str())
                    .unwrap_or("unknown");
                
                let action = v.get("action")
                    .and_then(|a| a.as_str())
                    .map(|a| format!(" (action: {})", a))
                    .unwrap_or_default();
                
                // Fallback for unknown events
                crate::DiscordEmbed {
                    title: format!("⚠️ Unknown: {} event", event_type.replace('_', " ")),
                    description: Some(format!(
                        "Received unknown event type '{}'{} in repository {} by @{}.\n\n\
                        ⚠️ Full event data has been logged to console for analysis.",
                        event_type, action, repo_name, sender
                    )),
                    url: None,
                    color: crate::transform::Colors::YELLOW,
                    author: None,
                    fields: vec![
                        crate::DiscordField {
                            name: "Event Type".to_string(),
                            value: format!("`{}`", event_type),
                            inline: true,
                        },
                        crate::DiscordField {
                            name: "Repository".to_string(),
                            value: repo_name.to_string(),
                            inline: true,
                        },
                        crate::DiscordField {
                            name: "Sender".to_string(),
                            value: format!("@{}", sender),
                            inline: true,
                        },
                    ],
                    footer: Some(crate::DiscordFooter {
                        text: "GitHub Unknown Event".to_string(),
                        icon_url: Some(
                            "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                                .to_string(),
                        ),
                    }),
                    timestamp: Some(chrono::Utc::now().to_rfc3339()),
                }
            }
        }
    }
}