use crate::DiscordField;

// Common formatting utilities for Discord embeds

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

pub fn format_commit_message(msg: &str) -> String {
    let first_line = msg.lines().next().unwrap_or(msg);
    truncate_string(first_line, 50)
}

pub fn code_field(name: impl Into<String>, value: impl Into<String>, inline: bool) -> DiscordField {
    DiscordField {
        name: name.into(),
        value: format!("`{}`", value.into()),
        inline,
    }
}

pub fn field(name: impl Into<String>, value: impl Into<String>, inline: bool) -> DiscordField {
    DiscordField {
        name: name.into(),
        value: value.into(),
        inline,
    }
}

pub fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}