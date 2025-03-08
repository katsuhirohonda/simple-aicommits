/// Centralized module for managing system prompts and prompt templates

/// System prompt used for all AI providers
pub const SYSTEM_PROMPT: &str = "You are a helpful assistant specialized in creating concise, meaningful git commit messages.";

/// Template for generating commit messages
pub fn get_commit_message_template(diff: &str) -> String {
    format!(
        "Generate a git commit message based on the following diff. \
        Use the following format:\n\
        - First line: A concise summary using conventional commits format (type: description) where appropriate\n\
        - Leave a blank line after the first line\n\
        - Then add 2-3 bullet points explaining the key changes in more detail\n\n\
        Focus on WHAT changed and WHY, not HOW. Keep the first line under 70 characters.\n\
        Return ONLY the commit message without any additional text.\n\n```diff\n{}\n```",
        diff
    )
}
