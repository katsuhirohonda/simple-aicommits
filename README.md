# simple-aicommits

A simple CLI tool that generates commit messages from git diffs using Claude AI, OpenAI, or Gemini.

## Installation

```bash
# Clone the repository
git clone https://github.com/katsuhirohonda/simple-aicommits.git
cd simple-aicommits

# Build and install
cargo install --path .
```

## Usage

First, ensure you have staged your changes with `git add`.

```bash
# Set your API keys
export ANTHROPIC_API_KEY="your-anthropic-api-key"
export OPENAI_API_KEY="your-openai-api-key"
export GEMINI_API_KEY="your-gemini-api-key"

# Optionally set the models to use
export ANTHROPIC_MODEL="claude-3-5-haiku-20241022"
export OPENAI_MODEL="gpt-4o-mini"
export GEMINI_MODEL="gemini-1.5-flash"

# Generate a commit message using Claude (default)
aicommits

# Generate a commit message using OpenAI
aicommits --provider openai

# Generate a commit message using Gemini
aicommits --provider gemini

# Generate a commit message with a specific model
aicommits --provider openai --model gpt-4o

# Generate a commit message and automatically commit
aicommits --commit
```

### Options

- `-c, --commit`: Automatically commit changes with the generated message
- `-p, --provider`: AI provider to use (claude, openai, gemini)
- `-m, --model`: AI model to use (overrides the default for the provider)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## How it Works

aicommits:

1. Gets the git diff of staged changes
2. Sends the diff to the selected AI provider to generate a structured commit message with:
   - A concise first line in conventional commits format
   - Several bullet points explaining the key changes
3. Optionally commits the changes with the generated message

## Requirements

- Rust 1.65 or higher
- Git installed and in your PATH
- API keys for the AI providers you want to use

## Environment Variables

- `ANTHROPIC_API_KEY`: Required for Claude. Your Anthropic API key.
- `ANTHROPIC_MODEL`: Optional. The Claude model to use (defaults to "claude-3-5-haiku-20241022").
- `OPENAI_API_KEY`: Required for OpenAI. Your OpenAI API key.
- `OPENAI_MODEL`: Optional. The OpenAI model to use (defaults to "gpt-4o-mini").
- `GEMINI_API_KEY`: Required for Gemini. Your Gemini API key.
- `GEMINI_MODEL`: Optional. The Gemini model to use (defaults to "gemini-1.5-flash").

## License

MIT