use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role,
};
use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use tracing::{error, info};

/// A simple tool that generates commit messages from git diffs using Claude AI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Git commit after generating message
    #[arg(short, long)]
    commit: bool,

    /// API key for Claude (overrides ANTHROPIC_API_KEY env var)
    #[arg(short, long)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse command line arguments
    let args = Args::parse();

    let api_key =
        std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");

    // Get git diff
    let diff = get_git_diff().context("Failed to get git diff")?;

    if diff.is_empty() {
        println!("No changes to commit.");
        return Ok(());
    }

    // Generate commit message
    let commit_message = generate_commit_message(&api_key, &diff).await?;

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

/// Generate a commit message using Claude AI
async fn generate_commit_message(api_key: &str, diff: &str) -> Result<String> {
    // Initialize the Anthropic client
    let client =
        AnthropicClient::new::<MessageError>(api_key.to_string(), "2023-06-01".to_string())
            .context("Failed to create Anthropic client")?;

    // Create the prompt for Claude
    let prompt = format!(
        "Generate a concise and informative git commit message based on the following diff. \
        Use the conventional commits format (type: description) where appropriate. \
        Focus on WHAT changed and WHY, not HOW. Keep it under 300 characters if possible. \
        Return ONLY the commit message without any additional text.\n\n```diff\n{}\n```",
        diff
    );

    // Create message parameters
    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![Message::new_text(Role::User, prompt)],
        max_tokens: 300,
    })
    .with_temperature(0.7)
    .with_system("You are a helpful assistant specialized in creating concise, meaningful git commit messages.");

    // Send request to Claude
    info!("Sending request to Claude...");
    match client.create_message(Some(&body)).await {
        Ok(response) => {
            // Extract the text content from the response
            let message = response
                .content
                .iter()
                .find_map(|block| {
                    if let anthropic_ai_sdk::types::message::ContentBlock::Text { text } = block {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "Failed to extract commit message from response".to_string());

            // Clean up the message (remove quotes, etc.)
            let clean_message = message.trim().trim_matches('"').trim().to_string();
            Ok(clean_message)
        }
        Err(e) => {
            error!("Error from Claude API: {}", e);
            Err(anyhow::anyhow!("Failed to generate commit message: {}", e))
        }
    }
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
