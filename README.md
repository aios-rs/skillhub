<div align="center">
  <h1>SkillHub CLI</h1>
  <p>Command-line interface for SkillHub - AI Agent Skill Registry</p>
</div>

<div align="center">

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/skillhub)
[![Rust](https://img.shields.io/badge/rust-2024-edition.svg)](https://www.rust-lang.org/)

</div>

SkillHub CLI is the official command-line tool for interacting with SkillHub, an enterprise-grade agent skill registry. Publish, discover, and manage reusable skill packages across your organization from the terminal.

## Features

- **Publish Skills** — Upload agent skill packages with semantic versioning
- **Search & Discovery** — Full-text search with filters by namespace, version, and tags
- **Install Skills** — Install skills directly to your local environment
- **Authentication** — Secure API token-based authentication
- **Team Namespaces** — Organize skills under team or global scopes
- **Configuration** — Manage registry settings and credentials locally

## Installation

### From Crates.io

```bash
cargo install skillhub
```

### From Source

```bash
git clone https://github.com/aios-rs/skillhub.git
cd skillhub-cli
cargo install --path .
```

## Quick Start

### Configure Registry

```bash
# Set registry URL
skillhub config set registry https://skillhub.your-company.com

# Login with API token
skillhub login
```

### Publish a Skill

```bash
# Publish to global namespace
skillhub publish ./my-skill --slug my-skill --version 1.0.0

# Publish to team namespace
skillhub publish ./my-skill --slug my-team--my-skill --version 1.0.0
```

### Search Skills

```bash
# Search all skills
skillhub search email

# Search in specific namespace
skillhub search email --namespace my-team

# Filter by version
skillhub search my-skill --version 1.0.0
```

### Install Skills

```bash
# Install by name
skillhub install my-skill

# Install specific version
skillhub install my-skill --version 1.0.0

# Install from team namespace
skillhub install my-team--my-skill
```

## Commands

| Command | Description |
|---------|-------------|
| `skillhub login` | Authenticate with API token |
| `skillhub logout` | Clear credentials |
| `skillhub publish <path>` | Publish a skill package |
| `skillhub search <query>` | Search for skills |
| `skillhub install <name>` | Install a skill |
| `skillhub config` | Manage configuration |
| `skillhub info <name>` | Show skill details |

## Configuration

The CLI stores configuration in `~/.config/skillhub/config.toml`:

```toml
registry = "https://skillhub.your-company.com"
token = "your-api-token"
default_namespace = "my-team"
```

## Authentication

SkillHub CLI uses API tokens for authentication. Generate a token from the SkillHub web UI and use it to login:

```bash
skillhub login --token YOUR_TOKEN
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- search my-skill
```

## Related Projects

- [SkillHub Server](https://github.com/aios-rs/skillhub) — The backend registry server
- [OpenClaw](https://github.com/openclaw/openclaw) — Open-source agent skill CLI

## Contributing

Contributions are welcome. Please open an issue first to discuss what you'd like to change.

## License

MIT License - see [LICENSE](./LICENSE) for details.
