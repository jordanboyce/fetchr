# Fetchr

<div align="center">

**Modern API Testing Desktop Application**

A powerful, fast, and intuitive Postman alternative built with Tauri 2.0, Vue 3, and Rust.

[Download Latest Release](#installation) • [Features](#features) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## Overview

Fetchr is a lightweight, cross-platform desktop application for testing and debugging HTTP APIs. Built with modern technologies, it offers a clean interface with powerful features while maintaining a small footprint and blazing-fast performance.

### Why Fetchr?

- **🚀 Lightning Fast** - Built with Rust backend for maximum performance
- **🪶 Lightweight** - Small binary size (~10-15MB) compared to Electron alternatives
- **🎨 Modern UI** - Beautiful dark theme with PrimeVue components
- **💾 Local-First** - All data stored locally in SQLite, no cloud required
- **🔒 Privacy-Focused** - Your API requests never leave your machine
- **🌍 Cross-Platform** - Works on Windows, macOS, and Linux

## Features

### Core Functionality
- ✅ **Full HTTP Method Support** - GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- ✅ **Request Builder** - Intuitive interface for configuring requests
  - URL parameters with variable interpolation
  - Custom headers
  - Multiple body types (JSON, Form Data, Raw)
  - Authentication (Basic Auth, Bearer Token, API Key)
- ✅ **Response Viewer** - Syntax-highlighted responses with automatic formatting
- ✅ **Collections & Folders** - Organize requests hierarchically
- ✅ **Environment Variables** - Manage different environments (dev, staging, prod)
- ✅ **Request History** - Track all executed requests with status and timing
- ✅ **Import/Export** - Postman collection compatibility
- ✅ **Copy as cURL** - Generate cURL commands from requests

### Planned Features
- 🔄 GraphQL support
- 🔄 WebSocket testing
- 🔄 Request chaining and scripting
- 🔄 Mock servers
- 🔄 API documentation generation
- 🔄 Team collaboration features

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases](../../releases) page:

- **Windows**: `Fetchr_0.1.0_x64-setup.exe`
- **macOS**: `Fetchr_0.1.0_universal.dmg`
- **Linux**: `fetchr_0.1.0_amd64.deb` or `Fetchr_0.1.0_amd64.AppImage`

### Build from Source

#### Prerequisites

- [Bun](https://bun.sh/) (v1.0+)
- [Rust](https://www.rust-lang.org/) (v1.70+)
- [Node.js](https://nodejs.org/) (v18+) - optional, for npm compatibility

#### Installation Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/fetchr.git
   cd fetchr
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Run in development mode:
   ```bash
   bun run tauri dev
   ```

4. Build for production:
   ```bash
   bun run tauri build
   ```

   Built applications will be in `src-tauri/target/release/bundle/`

## Usage

### Quick Start

1. **Create a Collection** - Click the "+" icon in the sidebar to create a folder
2. **Save a Request** - Configure your request and click "Save Request"
3. **Use Environment Variables** - Set up environments and use `{{variable}}` syntax
4. **Send Requests** - Hit "Send" and view formatted responses
5. **Organize** - Right-click collections for context menu options (export, delete)

### Environment Variables

Create environments (Dev, Staging, Production) and use variables in your requests:

```
URL: https://{{base_url}}/api/users
Header: Authorization: Bearer {{api_token}}
```

Variables are automatically interpolated when sending requests.

### Keyboard Shortcuts

- `Ctrl/Cmd + Enter` - Send request
- `Ctrl/Cmd + S` - Save current request
- `Ctrl/Cmd + N` - New request tab

## Development

### Project Structure

```
fetchr/
├── src/                    # Vue 3 frontend
│   ├── components/         # Vue components
│   ├── stores/            # Pinia state management
│   ├── types/             # TypeScript definitions
│   └── main.ts            # App entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Tauri commands
│   │   ├── db.rs          # SQLite database
│   │   └── http_client.rs # HTTP request handler
│   └── tauri.conf.json    # Tauri configuration
└── CLAUDE.md              # AI assistant instructions
```

### Tech Stack

**Frontend:**
- Vue 3 (Composition API)
- TypeScript
- Vite
- PrimeVue (UI components)
- Pinia (state management)
- Monaco Editor (code editing)

**Backend:**
- Rust
- Tauri 2.0
- reqwest (HTTP client)
- rusqlite (database)
- tokio (async runtime)

### Development Commands

```bash
# Frontend only (Vite dev server)
bun run dev

# Full app with hot reload
bun run tauri dev

# Type checking
bun run vue-tsc --noEmit

# Build production app
bun run tauri build
```

### Database

Fetchr uses SQLite for local storage. Database location:

- **Windows**: `%APPDATA%/dev.cyberlion.fetchr/fetchr.db`
- **macOS**: `~/Library/Application Support/dev.cyberlion.fetchr/fetchr.db`
- **Linux**: `~/.local/share/dev.cyberlion.fetchr/fetchr.db`

## Documentation

For detailed documentation on architecture, API, and development:

- [CLAUDE.md](./CLAUDE.md) - Project overview and development guide
- [Tauri Docs](https://v2.tauri.app) - Tauri framework documentation
- [Vue 3 Docs](https://vuejs.org) - Vue.js documentation

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** and ensure tests pass
4. **Commit your changes** (`git commit -m 'Add amazing feature'`)
5. **Push to your branch** (`git push origin feature/amazing-feature`)
6. **Open a Pull Request**

### Code Style

- Use TypeScript for all frontend code
- Follow Vue 3 Composition API patterns with `<script setup>`
- Use Rust 2021 edition standards for backend code
- Run type checking before committing: `bun run vue-tsc --noEmit`

### Reporting Issues

Found a bug or have a feature request? Please [open an issue](../../issues/new) with:

- Clear description of the problem/feature
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Screenshots if applicable

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app) - A framework for building desktop apps with web technologies
- UI powered by [PrimeVue](https://primevue.org) - A rich set of Vue 3 components
- Icons by [PrimeIcons](https://primevue.org/icons)

## Support

- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)

---

<div align="center">

**Made with ❤️ by CyberLion**

[⬆ Back to Top](#fetchr)

</div>
