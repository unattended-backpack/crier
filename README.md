# Crier

A Rust server that receives GitHub webhook events and forwards them to Discord with enhanced formatting. Named after town criers who historically delivered news to communities.

## Why Crier?

Discord's native GitHub integration doesn't support all GitHub events. Crier bridges this gap by:
- Supporting ALL GitHub webhook events
- Providing customizable formatting
- Verifying webhook signatures for security
- Transforming events into rich Discord embeds

# Security

If you discover any bug; flaw; issue; dæmonic incursion; or other malicious, negligent, or incompetent action that impacts the security of any of these projects please responsibly disclose them to us; instructions are available [here](./SECURITY.md).

## Setup

### 1. Configure Environment

Copy the example environment file and edit it:
```bash
cp .env.example .env
```

Required variables:
- `DISCORD_WEBHOOK_URL`: Your Discord webhook URL
- `GITHUB_WEBHOOK_SECRET`: (Optional) Secret for signature verification
- `BIND_ADDRESS`: (Optional) Server address, defaults to `0.0.0.0:3000`

### 2. Run the Server

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/crier
```

### 3. Configure GitHub Webhook

1. Go to your repository/organization settings → Webhooks
2. Add webhook:
   - **Payload URL**: `http://your-server:3000/webhook`
   - **Content type**: `application/json`
   - **Secret**: Same as `GITHUB_WEBHOOK_SECRET` (if configured)
   - **Events**: Select any/all events you want to receive

## Testing

Crier includes a comprehensive test suite that sends sample events to a Discord webhook for visual verification.

### Running the Event Tests

1. Set the test webhook URL:
```bash
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/..."
```

2. Run the comprehensive test (60+ event types):
```bash
cargo test
```

This will send sample events for ALL GitHub webhook event types including:

**Core Events:**
- Push, Pull Request (+ reviews, comments, threads)
- Issues (+ comments), Releases, Stars, Forks
- Create/Delete (branch/tag)

**Security Events:**
- Code scanning alerts (+ dismissal requests)
- Secret scanning alerts (+ locations, scans, dismissals)
- Dependabot alerts
- Repository vulnerability alerts
- Security and analysis changes

**CI/CD Events:**
- Workflow runs & jobs
- Check runs & suites
- Deployments & deployment statuses
- Commit statuses

**Collaboration Events:**
- Discussions (+ comments)
- Projects v2 (+ items, status updates)
- Classic projects (+ cards, columns)
- Wiki updates (Gollum)
- Milestones, Labels

**Repository Management:**
- Repository events (create, delete, visibility)
- Branch protection rules & configurations
- Repository rulesets, imports, advisories
- Deploy keys, Custom properties

**Organization Events:**
- Organization membership
- Teams (+ adds, membership)
- Organization blocks
- Personal access token requests

**Package Events:**
- GitHub Packages, Registry packages

**Other Events:**
- Page builds, Sponsorships
- Sub-issues, Merge groups
- Commit comments, Meta (webhook deletion)
- Watches, Bypass requests

Each event will appear in your Discord channel with appropriate formatting and category-specific colors.

## Enhanced Event Details

Crier extracts and displays event-specific information for better context:

### Core Events
- **Push**: Shows branch, commit count, and recent commit messages
- **Pull Request**: Displays PR title, branches, file changes (+/-)
- **Issues**: Shows labels, state, and milestone information
- **Releases**: Indicates pre-release status, tag, and draft state
- **Stars**: Shows total repository star count
- **Forks**: Displays who forked the repository
- **Create/Delete**: Shows branch/tag name and type
- **Issue Comments**: Includes comment preview (first 100 chars)

### Security Events
- **Code Scanning Alerts**: Shows severity, tool (e.g., CodeQL), file location, and vulnerability description
- **Secret Scanning Alerts**: Displays secret type, location in code, and state
- **Dependabot Alerts**: Shows CVE ID, severity, vulnerable versions, and patched version

### CI/CD Events
- **Workflow Runs**: Shows status emoji (✅/❌/⚠️), run number, and branch
- **Check Runs**: Displays app name, conclusion, and summary
- **Deployments**: Shows environment, status with emoji indicators

### Collaboration Events
- **Discussions**: Shows category, state, and content preview
- **Milestones**: Displays progress percentage and due date
- **Packages**: Shows package type, version, and description

# Our Licenses

The [license](./LICENSE) for all of our original work is `LicenseRef-VPL WITH AGPL-3.0-only`. This includes every modification of our own design in this repository: code, documentation, images, branding, and more. You are licensed to use all of it so long as you maintain _maximum possible virality_ and our copyleft licenses.

Permissive open source licenses are tools for the corporate subversion of libre software; visible source licenses are an even more malignant scourge. All original works in this project are to be licensed under the most aggressive, virulently-contagious copyleft terms possible. To that end everything is licensed under the [Viral Public License](./licenses/LicenseRef-VPL) coupled with the [GNU Affero General Public License v3.0](./licenses/AGPL-3.0-only) for use in the event that some unaligned party attempts to weasel their way out of copyleft protections. In short: if you use or modify anything in this project for any reason, your project must be licensed under these same terms.

For art assets specifically, in case you want to further split hairs or attempt to weasel out of this virality, we explicitly license those under the viral and copyleft [Free Art License 1.3](./licenses/FreeArtLicense-1.3).
