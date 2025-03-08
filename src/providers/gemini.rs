use crate::ai_provider::CommitMessageGenerator;
use crate::prompts::{SYSTEM_PROMPT, get_commit_message_template};
use anyhow::Result;
use async_trait::async_trait;
use genai::{
    Client,
    chat::{ChatMessage, ChatRequest},
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
        // Set API key in environment variable
        unsafe {
            std::env::set_var("GOOGLE_API_KEY", &self.api_key);
        }

        // Initialize the Gemini client
        let client = Client::default();

        // Get prompt from the centralized prompt manager
        let user_prompt = get_commit_message_template(diff);
        let prompt = format!("System: {}\n\nUser: {}", SYSTEM_PROMPT, user_prompt);

        // Create chat request
        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);

        // Send request to Gemini
        match client.exec_chat(&self.model, chat_req, None).await {
            Ok(response) => {
                let message = response
                    .content_text_as_str()
                    .unwrap_or_else(|| "Failed to extract commit message from response");
                Ok(message.trim().trim_matches('"').trim().to_string())
            }
            Err(e) => {
                error!("Error from Gemini API: {}", e);
                Err(anyhow::anyhow!("Failed to generate commit message: {}", e))
            }
        }
    }
}
