# simple-aicommits

A simple CLI tool that generates commit messages from git diffs using Claude AI.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/simple-aicommits.git
cd simple-aicommits

# Build and install
cargo install --path .
```

## Usage

First, ensure you have staged your changes with `git add`.

```bash
# Set your Anthropic API key
export ANTHROPIC_API_KEY="your-api-key"

# Generate a commit message without committing
simple-aicommits

# Generate a commit message and automatically commit
simple-aicommits --commit
```

### Options

- `-c, --commit`: Automatically commit changes with the generated message
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## How it Works

simple-aicommits:

1. Gets the git diff of staged changes
2. Sends the diff to Claude AI to generate a concise, informative commit message
3. Optionally commits the changes with the generated message

## Requirements

- Rust 1.65 or higher
- Git installed and in your PATH
- An Anthropic API key

## License

MIT