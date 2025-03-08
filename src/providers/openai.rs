use crate::ai_provider::CommitMessageGenerator;
use crate::prompts::{SYSTEM_PROMPT, get_commit_message_template};
use anyhow::Result;
use async_trait::async_trait;
use openai_api_rust::chat::*;
use openai_api_rust::*;
use tracing::error;

pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "gpt-4-turbo-preview".to_string());
        Self { api_key, model }
    }
}

#[async_trait]
impl CommitMessageGenerator for OpenAIProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        let auth = Auth::new(&self.api_key);
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        let messages = vec![
            Message {
                role: Role::System,
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: Role::User,
                content: get_commit_message_template(diff),
            },
        ];

        let body = ChatBody {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(500),
            n: Some(1),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            top_p: Some(1.0),
        };

        match openai.chat_completion_create(&body) {
            Ok(response) => {
                let message = response
                    .choices
                    .first()
                    .and_then(|choice| choice.message.as_ref())
                    .map(|msg| msg.content.clone())
                    .unwrap_or_else(|| {
                        "Failed to extract commit message from response".to_string()
                    });

                Ok(message.trim().trim_matches('"').trim().to_string())
            }
            Err(e) => {
                error!("Error from OpenAI API: {}", e);
                Err(anyhow::anyhow!("Failed to generate commit message: {}", e))
            }
        }
    }
}
