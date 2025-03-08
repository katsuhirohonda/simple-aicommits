use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents available AI providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Claude,
    OpenAI,
    Gemini,
}

impl Display for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Claude => write!(f, "claude"),
            Provider::OpenAI => write!(f, "openai"),
            Provider::Gemini => write!(f, "gemini"),
        }
    }
}

impl FromStr for Provider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "claude" => Ok(Provider::Claude),
            "openai" => Ok(Provider::OpenAI),
            "gemini" => Ok(Provider::Gemini),
            _ => Err(anyhow::anyhow!(
                "Unknown provider: {}. Available providers: claude, openai, gemini",
                s
            )),
        }
    }
}

impl Provider {
    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::Claude => "claude-3-5-haiku-20241022",
            Provider::OpenAI => "gpt-4o-mini",
            Provider::Gemini => "gemini-1.5-flash",
        }
    }
}

/// Trait for AI providers that can generate commit messages
#[async_trait]
pub trait CommitMessageGenerator {
    /// Generate a commit message based on the given diff
    async fn generate_commit_message(&self, diff: &str) -> Result<String>;
}
