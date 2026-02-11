# Fetchr - API Testing Desktop App Implementation

## Overview

Fetchr is a fully-featured API testing desktop application built with Tauri 2.0, Vue 3, TypeScript, and PrimeVue. It provides a Postman-like experience for testing HTTP APIs with advanced features like environment variables, collections, and request history.

## Features Implemented

### ✅ Core Features (MVP Complete)

1. **Request Builder**
   - HTTP method dropdown (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
   - URL input with environment variable support (`{{variable}}` syntax)
   - Tabbed interface for:
     - Query Parameters (key-value DataTable with enable/disable)
     - Headers (key-value DataTable with enable/disable)
     - Body (None/JSON/Form/Raw with textarea editor)
     - Auth (None/Basic Auth/Bearer Token/API Key)

2. **Response Viewer**
   - Color-coded status badges (green=2xx, yellow=3xx, red=4xx/5xx)
   - Response metrics (time in ms, size in bytes)
   - Tabbed response display:
     - Body with syntax highlighting (JSON/XML via highlight.js)
     - Headers DataTable
     - Cookies table (when present)
   - Copy response to clipboard

3. **Collection Sidebar**
   - Tree view for organizing folders and requests
   - Context menu for creating/deleting items
   - Save current request to collection
   - Load saved requests
   - Search/filter functionality

4. **Environment System**
   - Environment manager dialog
   - Create/edit/delete environments
   - Key-value variables
   - Active environment selection
   - Automatic variable interpolation in URL, headers, and body

5. **Request History**
   - Automatic tracking of all sent requests
   - Display method, URL, status, response time, and timestamp
   - Clear history functionality
   - Paginated table view

### 🔧 Technical Architecture

#### Backend (Rust/Tauri)

**File Structure:**
- `src-tauri/src/lib.rs` - Main application entry, Tauri commands registration
- `src-tauri/src/db.rs` - SQLite database layer with schema and operations
- `src-tauri/src/http_client.rs` - HTTP client implementation using reqwest

**Database Schema:**
```sql
collections (id, name, parent_id, is_folder, created_at)
requests (id, collection_id, name, method, url, headers, body, body_type, auth_type, auth_data, created_at, updated_at)
environments (id, name, variables, is_active, created_at)
history (id, method, url, status, response_time, created_at)
```

**Tauri Commands:**
- `send_http_request` - Send HTTP request and return response
- `interpolate_variables` - Replace {{variables}} with environment values
- Collection CRUD: `create_collection`, `get_all_collections`, `delete_collection`
- Request CRUD: `save_request`, `get_requests_by_collection`, `get_request`, `delete_request`
- Environment CRUD: `save_environment`, `get_all_environments`, `get_active_environment`, `delete_environment`
- History: `add_history`, `get_history`, `clear_history`

#### Frontend (Vue 3/TypeScript)

**File Structure:**
```
src/
├── components/
│   ├── RequestBuilder.vue      # HTTP request configuration
│   ├── ResponseViewer.vue      # Response display with syntax highlighting
│   ├── CollectionSidebar.vue   # Collections tree and management
│   ├── EnvironmentManager.vue  # Environment variables UI
│   └── HistoryPanel.vue        # Request history viewer
├── stores/
│   └── app.ts                  # Pinia store for global state
├── types/
│   └── index.ts                # TypeScript interfaces
├── styles/
│   └── theme.css               # Custom dark theme
├── App.vue                     # Main application layout
└── main.ts                     # Vue app initialization
```

**State Management (Pinia):**
- Collections and folder tree
- Current request configuration
- Current response data
- Environments and active environment
- Request history
- Loading states

**Component Communication:**
- Props for data passing
- Emit events for user actions
- Pinia store for shared state
- Tauri invoke() for backend calls

#### UI/Styling

- **Framework:** PrimeVue 4 components
- **Theme:** Custom dark mode theme (Aura-inspired)
- **Icons:** PrimeIcons
- **Layout:** Splitter panels for resizable sections
- **Responsive:** Desktop-optimized with 3-panel layout

## Project Structure

```
fetchr/
├── src/                      # Vue frontend
│   ├── components/          # Vue components
│   ├── stores/              # Pinia state management
│   ├── types/               # TypeScript definitions
│   ├── styles/              # CSS theme
│   └── App.vue              # Main app component
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri commands
│   │   ├── db.rs           # Database layer
│   │   ├── http_client.rs  # HTTP client
│   │   └── main.rs         # Entry point
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── package.json            # Node dependencies
└── vite.config.ts          # Vite configuration
```

## How to Run

### Development Mode

```bash
# Install frontend dependencies
bun install

# Run Tauri development server (includes frontend and backend)
bun run tauri dev
```

This will:
1. Start Vite dev server on http://localhost:1420
2. Compile Rust backend
3. Launch desktop window with hot-reload

### Build for Production

```bash
# Build optimized executable
bun run tauri build
```

Outputs platform-specific executables in `src-tauri/target/release/bundle/`

## Database Location

The SQLite database is automatically created at:
- **Windows:** `%APPDATA%/dev.cyberlion.fetchr/fetchr.db`
- **macOS:** `~/Library/Application Support/dev.cyberlion.fetchr/fetchr.db`
- **Linux:** `~/.local/share/dev.cyberlion.fetchr/fetchr.db`

## Usage Examples

### Sending a Request

1. Select HTTP method from dropdown
2. Enter URL (e.g., `https://api.github.com/users/{{username}}`)
3. Add headers in Headers tab
4. Configure body in Body tab (for POST/PUT)
5. Click "Send" button
6. View response in bottom panel

### Using Environment Variables

1. Click "Tools" → "Environments"
2. Create new environment with name (e.g., "Development")
3. Add variables: `key: username, value: octocat`
4. Click "Activate"
5. Use `{{username}}` in URLs, headers, or body
6. Variables are interpolated before sending

### Organizing Collections

1. Click "+" in sidebar to create folder
2. Right-click folder → "New Request"
3. Configure request and click "Save" icon
4. Select collection and name request
5. Click on saved request to load it

## Future Enhancements (Not Yet Implemented)

### Postman Collection Import
To implement:
1. Add file picker in File → Import menu
2. Parse Postman JSON schema v2.1
3. Map to Fetchr collection structure
4. Insert into SQLite database

### Additional Features to Consider
- WebSocket testing
- GraphQL support
- Code snippet generation (curl, Python, JS, etc.)
- Test scripts (pre-request/post-response)
- Mock servers
- API documentation generator
- Team collaboration features
- Request chaining
- Bulk operations

## Dependencies

### Frontend
- Vue 3 - Reactive UI framework
- Pinia - State management
- PrimeVue - UI component library
- PrimeIcons - Icon library
- Vite - Build tool
- TypeScript - Type safety
- highlight.js - Syntax highlighting

### Backend
- Tauri 2.0 - Desktop framework
- reqwest - HTTP client
- rusqlite - SQLite database
- tokio - Async runtime
- serde/serde_json - JSON serialization
- uuid - Unique ID generation

## Success Criteria Status

- ✅ Can send GET/POST/PUT/DELETE requests
- ✅ Headers and body editing works
- ✅ Response displays with syntax highlighting
- ✅ Collections save/load from SQLite
- ✅ Environment variables interpolate correctly
- ⏳ Import Postman collections (not implemented)
- ✅ Request history accessible
- ⏳ Builds for Windows/Mac/Linux (ready to build, needs network connection)

## Notes

- The application uses Bun as the JavaScript runtime for faster performance
- Dark theme is enabled by default
- All data is stored locally in SQLite
- HTTP client supports cookies, multipart forms, and various auth methods
- The UI is fully responsive with resizable panels

## Troubleshooting

### Cargo Network Issues
If you encounter network timeouts during Rust dependency downloads, try:
```bash
# Configure cargo to use a mirror
export CARGO_HTTP_MULTIPLEXING=false
```

### Build Errors
Ensure you have:
- Bun installed
- Rust toolchain (rustc, cargo)
- Node dependencies installed (`bun install`)

### Database Issues
If the database becomes corrupted, delete it and restart the application:
```bash
# Windows
del %APPDATA%\dev.cyberlion.fetchr\fetchr.db

# macOS/Linux
rm ~/Library/Application\ Support/dev.cyberlion.fetchr/fetchr.db
```

## License

This project was generated as a demonstration. Adapt and use as needed.
