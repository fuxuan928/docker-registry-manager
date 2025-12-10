# Docker Registry Manager

A cross-platform desktop application for managing Docker registries. Browse repositories, view image tags, inspect manifests, and manage your container images with an intuitive graphical interface.

![Docker Registry Manager](assets/header.svg)

## Features

- **Multi-Registry Support** - Connect to multiple Docker registries simultaneously
- **Repository Browser** - Browse and search repositories in your registries
- **Tag Management** - View, search, and delete image tags
- **Manifest Inspector** - View detailed manifest information including layers, digests, and configurations
- **Batch Operations** - Select and delete multiple tags at once
- **Secure Credential Storage** - Passwords and tokens are encrypted with AES-256-GCM
- **Authentication Support** - Anonymous, Basic Auth, and Bearer Token authentication
- **Dark/Light Theme** - Switch between themes or follow system preference
- **Import/Export** - Export and import registry configurations

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started)

Install Dioxus CLI:
```bash
cargo install dioxus-cli
```

### Build from Source

Clone the repository and build:

```bash
git clone https://github.com/fuxuan928/docker-registry-manager.git
cd docker-registry-manager
dx build --release
```

### Run in Development Mode

```bash
dx serve
```

## Usage

1. **Add a Registry** - Click the "+" button in the sidebar to add a new registry
2. **Configure Authentication** - Choose Anonymous, Basic Auth, or Bearer Token
3. **Browse Repositories** - Select a registry to view its repositories
4. **Manage Tags** - Click on a repository to view and manage its tags
5. **View Manifests** - Select a tag to inspect its manifest details
6. **Delete Images** - Use the delete button to remove tags or entire repositories

## License

MIT License
