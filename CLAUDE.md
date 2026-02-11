# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fetchr is an API testing desktop application (Postman alternative) built with Tauri 2.0, combining a Rust backend with a Vue 3 + TypeScript frontend. The application uses Bun as the package manager and runtime.

**Key Features:**
- HTTP request builder with full method support (GET, POST, PUT, DELETE, etc.)
- Environment variable system with interpolation (`{{variable}}` syntax)
- Collection organization with folders and tree view
- Request history tracking
- Response viewer with syntax highlighting
- SQLite database for local storage

## Development Commands

### Frontend Development
```bash
bun run dev              # Start Vite dev server (frontend only)
bun run build            # Build frontend for production
bun run preview          # Preview production build
```

### Tauri (Full Stack) Development
```bash
bun run tauri dev        # Run full Tauri app in development mode
bun run tauri build      # Build production executable
```

### Type Checking
```bash
vue-tsc --noEmit         # Type check Vue components and TypeScript files
```

## Architecture

### Frontend Stack
- **Framework**: Vue 3 with Composition API (`<script setup>` syntax)
- **Build Tool**: Vite (configured for Tauri with fixed port 1420)
- **UI Library**: PrimeVue 4.5+ with PrimeIcons
- **State Management**: Pinia 3.0+
- **Utilities**: VueUse for composables
- **Editor**: Monaco Editor for code editing
- **Syntax Highlighting**: highlight.js

### Backend Stack
- **Framework**: Tauri 2.0
- **Language**: Rust (2021 edition)
- **HTTP Client**: reqwest with JSON, multipart, and cookies support
- **Database**: rusqlite with bundled SQLite
- **Async Runtime**: Tokio (full features)
- **Serialization**: serde + serde_json
- **UUID Generation**: uuid crate for unique IDs

### Tauri-Vue Integration
- Frontend communicates with Rust backend via `invoke()` from `@tauri-apps/api/core`
- Rust commands are defined with `#[tauri::command]` macro
- Commands are registered in `src-tauri/src/lib.rs` via `invoke_handler`

### Project Structure
- `src/` - Vue frontend code
  - `components/` - Vue components (RequestBuilder, ResponseViewer, CollectionSidebar, etc.)
  - `stores/` - Pinia state management (app.ts contains all application state)
  - `types/` - TypeScript type definitions
  - `styles/` - Custom dark theme CSS
  - `main.ts` - Vue app entry point with PrimeVue setup
  - `App.vue` - Root component with main layout
- `src-tauri/` - Rust backend code
  - `src/lib.rs` - Tauri command handlers and app setup
  - `src/db.rs` - SQLite database operations
  - `src/http_client.rs` - HTTP request handling with reqwest
  - `src/main.rs` - Application entry point
  - `tauri.conf.json` - Tauri configuration
  - `Cargo.toml` - Rust dependencies
- `public/` - Static files served by Vite
- `dist/` - Frontend build output (referenced by Tauri config)

## Important Configuration Details

### Tauri Configuration
- **Dev URL**: http://localhost:1420 (Vite dev server)
- **App Identifier**: dev.cyberlion.fetchr
- **Frontend Dist**: ../dist (relative to src-tauri)
- **Dev Command**: `bun run dev`
- **Build Command**: `bun run build`

### Vite Configuration
- Port 1420 (strict) for Tauri integration
- HMR on port 1421
- Ignores watching `src-tauri` directory
- Clear screen disabled to show Rust errors

### TypeScript Configuration
- Target: ES2020
- Strict mode enabled with additional linting rules
- No unused locals/parameters allowed
- Module resolution: bundler mode

## Adding New Tauri Commands

1. Define command in `src-tauri/src/lib.rs`:
```rust
#[tauri::command]
fn my_command(param: &str) -> String {
    // implementation
}
```

2. Register in invoke_handler:
```rust
.invoke_handler(tauri::generate_handler![greet, my_command])
```

3. Call from Vue:
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("my_command", { param: "value" });
```

## Adding Rust Dependencies

Edit `src-tauri/Cargo.toml` and run development mode to install them automatically, or run:
```bash
cd src-tauri && cargo build
```

## Database Schema

The application uses SQLite for local storage. Database file location:
- Windows: `%APPDATA%/dev.cyberlion.fetchr/fetchr.db`
- macOS: `~/Library/Application Support/dev.cyberlion.fetchr/fetchr.db`
- Linux: `~/.local/share/dev.cyberlion.fetchr/fetchr.db`

Tables:
- `collections` - Folders and request collections
- `requests` - Saved HTTP requests with all configuration
- `environments` - Environment variable sets
- `history` - Request execution history

## State Management

The application uses a single Pinia store (`src/stores/app.ts`) that manages:
- Collections and folder tree structure
- Current request configuration (method, URL, headers, body, auth)
- Current response data
- Environments and active environment selection
- Request history

All Tauri commands are called from the store actions, keeping components clean.

## Component Architecture

- **CollectionSidebar** - Tree view with context menu for managing collections
- **RequestBuilder** - Tabbed interface for configuring HTTP requests
- **ResponseViewer** - Displays responses with syntax highlighting
- **EnvironmentManager** - Dialog for managing environment variables
- **HistoryPanel** - Shows request history with filtering

Components communicate via:
- Pinia store for shared state
- Props for parent-child data flow
- Emits for child-parent events
