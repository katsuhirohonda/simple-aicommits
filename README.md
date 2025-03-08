# aicommits

A simple CLI tool that generates commit messages from git diffs using Claude AI.

## Installation

```bash
# Clone the repository
git clone https://github.com/katsuhirohonda/aicommits.git
cd aicommits

# Build and install
cargo install --path .
```

## Usage

First, ensure you have staged your changes with `git add`.

```bash
# Set your Anthropic API key
export ANTHROPIC_API_KEY="your-api-key"

# Generate a commit message without committing
aicommits

# Generate a commit message and automatically commit
aicommits --commit
```

### Options

- `-c, --commit`: Automatically commit changes with the generated message
- `-a, --api-key <API_KEY>`: Specify an Anthropic API key directly (overrides environment variable)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## How it Works

aicommits:

1. Gets the git diff of staged changes
2. Sends the diff to Claude AI to generate a concise, informative commit message
3. Optionally commits the changes with the generated message

## Requirements

- Rust 1.65 or higher
- Git installed and in your PATH
- An Anthropic API key

## License

MIT
# simple-aicommits
