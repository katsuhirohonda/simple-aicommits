use crate::ai_provider::{CommitMessageGenerator, Provider};
use crate::prompts::{SYSTEM_PROMPT, get_commit_message_template};
use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use tracing::error;

pub struct ClaudeProvider {
    api_key: String,
    model: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| Provider::Claude.default_model().to_string());
        Self { api_key, model }
    }
}

#[async_trait]
impl CommitMessageGenerator for ClaudeProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        // Initialize the Anthropic client
        let client =
            AnthropicClient::new::<MessageError>(self.api_key.clone(), "2023-06-01".to_string())
                .context("Failed to create Anthropic client")?;

        // Get the prompt from the centralized prompt manager
        let prompt = get_commit_message_template(diff);

        // Create message parameters
        let body = CreateMessageParams::new(RequiredMessageParams {
            model: self.model.clone(),
            messages: vec![Message::new_text(Role::User, prompt)],
            max_tokens: 500,
        })
        .with_temperature(0.7)
        .with_system(SYSTEM_PROMPT);

        // Send request to Claude
        match client.create_message(Some(&body)).await {
            Ok(response) => {
                // Extract the text content from the response
                let message = response
                    .content
                    .iter()
                    .find_map(|block| {
                        if let anthropic_ai_sdk::types::message::ContentBlock::Text { text } = block
                        {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        "Failed to extract commit message from response".to_string()
                    });

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
}
