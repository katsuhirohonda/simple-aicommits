mod ai_provider;
mod prompts;
mod providers;

use ai_provider::{CommitMessageGenerator, Provider};
use anyhow::{Context, Result};
use clap::Parser;
use providers::{ClaudeProvider, GeminiProvider, OpenAIProvider};
use std::process::Command;
use std::str::FromStr;
use tracing::info;

/// A simple tool that generates commit messages from git diffs using various AI models
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Git commit after generating message
    #[arg(short, long)]
    commit: bool,

    /// AI provider to use (claude, openai, gemini)
    #[arg(short, long, default_value = "claude")]
    provider: String,

    /// AI model to use (overrides the default for the provider)
    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse command line arguments
    let args = Args::parse();

    // Parse the provider
    let provider = Provider::from_str(&args.provider)
        .context(format!("Invalid provider: {}", args.provider))?;

    // Get git diff
    let diff = get_git_diff().context("Failed to get git diff")?;

    if diff.is_empty() {
        println!("No changes to commit.");
        return Ok(());
    }

    // Generate commit message based on the selected provider
    let commit_message = generate_commit_message(provider, &diff, args.model).await?;

    println!("\nGenerated commit message:\n{}", commit_message);

    // Commit if requested
    if args.commit {
        git_commit(&commit_message).context("Failed to commit changes")?;
        println!("Changes committed successfully!");
    } else {
        println!("\nTo commit with this message, run:");
        println!("git commit -m \"{}\"", commit_message.replace("\"", "\\\""));
    }

    Ok(())
}

/// Get the git diff for staged changes
fn get_git_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .context("Failed to execute git diff command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git diff command failed: {}", stderr));
    }

    let diff = String::from_utf8(output.stdout).context("Git diff output is not valid UTF-8")?;
    Ok(diff)
}

/// Generate a commit message using the selected AI provider
async fn generate_commit_message(
    provider: Provider,
    diff: &str,
    model_override: Option<String>,
) -> Result<String> {
    info!("Using provider: {}", provider);

    // Create the appropriate provider based on the selection
    let generator: Box<dyn CommitMessageGenerator> = match provider {
        Provider::Claude => {
            let api_key = std::env::var("ANTHROPIC_API_KEY")
                .context("ANTHROPIC_API_KEY environment variable not set")?;
            let model = model_override.or_else(|| std::env::var("ANTHROPIC_MODEL").ok());
            Box::new(ClaudeProvider::new(api_key, model))
        }
        Provider::OpenAI => {
            let api_key = std::env::var("OPENAI_API_KEY")
                .context("OPENAI_API_KEY environment variable not set")?;
            let model = model_override.or_else(|| std::env::var("OPENAI_MODEL").ok());
            Box::new(OpenAIProvider::new(api_key, model))
        }
        Provider::Gemini => {
            let api_key = std::env::var("GEMINI_API_KEY")
                .context("GEMINI_API_KEY environment variable not set")?;
            let model = model_override.or_else(|| std::env::var("GEMINI_MODEL").ok());
            Box::new(GeminiProvider::new(api_key, model))
        }
    };

    // Generate the commit message
    info!("Generating commit message...");
    generator.generate_commit_message(diff).await
}

/// Commit changes with the generated message
fn git_commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to execute git commit command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git commit command failed: {}", stderr));
    }

    Ok(())
}
