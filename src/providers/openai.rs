use crate::ai_provider::CommitMessageGenerator;
use crate::prompts::{get_commit_message_template, SYSTEM_PROMPT};
use anyhow::{Context, Result};
use async_trait::async_trait;
use openai::{
    api::Client,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, 
        CreateChatCompletionRequestArgs, Role,
    },
};
use tracing::error;

pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "gpt-4o-mini".to_string());
        Self { api_key, model }
    }
}

#[async_trait]
impl CommitMessageGenerator for OpenAIProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        // Initialize the OpenAI client
        let client = Client::new().with_api_key(&self.api_key);

        // Get prompts from the centralized prompt manager
        let user_message = get_commit_message_template(diff);

        // Create chat messages
        let messages = vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(SYSTEM_PROMPT)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_message)
                .build()?,
        ];

        // Create chat completion request
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .temperature(0.7)
            .max_tokens(500_u16)
            .build()?;

        // Send request to OpenAI
        match client.chat().create(request).await {
            Ok(response) => {
                let message = response
                    .choices
                    .first()
                    .and_then(|choice| choice.message.content.clone())
                    .unwrap_or_else(|| "Failed to extract commit message from response".to_string());

                // Clean up the message
                let clean_message = message.trim().trim_matches('"').trim().to_string();
                Ok(clean_message)
            }
            Err(e) => {
                error!("Error from OpenAI API: {}", e);
                Err(anyhow::anyhow!("Failed to generate commit message: {}", e))
            }
        }
    }
}
