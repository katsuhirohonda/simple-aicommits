use crate::ai_provider::CommitMessageGenerator;
use crate::prompts::{get_commit_message_template, SYSTEM_PROMPT};
use anyhow::{Context, Result};
use async_trait::async_trait;
use google_generative_ai_rs::{
    google::generative::Gemini,
    requests::generation::{GenerationRequest, GenerationRequestConfig, TextPart},
};
use tracing::error;

pub struct GeminiProvider {
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "gemini-1.5-flash".to_string());
        Self { api_key, model }
    }
}

#[async_trait]
impl CommitMessageGenerator for GeminiProvider {
    async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        // Initialize the Gemini client
        let client = Gemini::new(self.api_key.clone());

        // Get prompt from the centralized prompt manager
        // Note: Gemini doesn't have a standard system prompt, so we'll prepend it to the user prompt
        let user_prompt = get_commit_message_template(diff);
        let prompt = format!("System: {}\n\nUser: {}", SYSTEM_PROMPT, user_prompt);

        // Create generation request
        let request = GenerationRequest::default()
            .with_model(self.model.clone())
            .with_contents(vec![TextPart::text_only(prompt)])
            .with_generation_config(
                GenerationRequestConfig::default()
                    .with_temperature(0.7)
                    .with_max_output_tokens(500),
            );

        // Send request to Gemini
        match client.generate_content(request).await {
            Ok(response) => {
                let message = response
                    .candidates
                    .first()
                    .and_then(|candidate| {
                        candidate.content.parts.first().and_then(|part| match part {
                            google_generative_ai_rs::requests::generation::Part::Text { text } => {
                                Some(text.clone())
                            }
                            _ => None,
                        })
                    })
                    .unwrap_or_else(|| "Failed to extract commit message from response".to_string());

                // Clean up the message
                let clean_message = message.trim().trim_matches('"').trim().to_string();
                Ok(clean_message)
            }
            Err(e) => {
                error!("Error from Gemini API: {}", e);
                Err(anyhow::anyhow!("Failed to generate commit message: {}", e))
            }
        }
    }
}
