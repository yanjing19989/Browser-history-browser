# Copilot Instructions for Browser History Browser

## Project Overview

This is a **Browser History Visualization** desktop application built with Tauri, providing a local, private, and high-performance solution for analyzing browser history data. The application uses a Rust backend with a native JavaScript frontend and SQLite database.

## Architecture

```
┌──────────────────────────────────────┐
│            Frontend (JavaScript)      │
│  - Native HTML/CSS/JS (no frameworks) │
│  - History browsing and filtering UI   │
│  - Statistics dashboard               │
└─────────────┬────────────────────────┘
              │ Tauri IPC Commands
┌─────────────▼────────────────────────┐
│            Rust Backend              │
│  - Command handlers (commands.rs)    │
│  - Database layer (db.rs)           │
│  - Configuration (config.rs)        │
│  - Domain models (domain.rs)        │
└─────────────┬────────────────────────┘
              │
┌─────────────▼────────────────────────┐
│           SQLite Database            │
│  - Browser history storage          │
│  - Navigation history table         │
│  - Full-text search ready (FTS5)    │
└──────────────────────────────────────┘
```

## Tech Stack

- **Desktop Framework**: Tauri v1.x
- **Backend**: Rust with key dependencies:
  - `rusqlite` for SQLite database operations
  - `serde` for JSON serialization
  - `anyhow` for error handling
  - `chrono` for date/time handling
- **Frontend**: Pure HTML/CSS/JavaScript (no frameworks)
- **Database**: SQLite with potential FTS5 integration
- **Build Tools**: Cargo (Rust), npm/pnpm (JavaScript)

## Code Organization

### Rust Backend (`src-tauri/src/`)

- **`main.rs`**: Application entry point, Tauri setup, command registration
- **`commands.rs`**: Tauri command handlers for IPC communication
  - `list_history`: Paginated history listing with filters
  - `stats_overview`: Statistics and KPI calculations
  - `get_config`/`set_db_path`: Configuration management
  - `validate_db_path`/`browse_db_file`: Database file operations
- **`db.rs`**: Database connection management and schema initialization
- **`domain.rs`**: Data structures, error types, and type definitions
- **`config.rs`**: Application configuration handling with JSON persistence

### Frontend (`src/`)

- **`index.html`**: Main application UI with Chinese/English mixed interface
- **`main.js`**: JavaScript application logic using Tauri API (`window.__TAURI__.tauri.invoke`)
- **`settings.html`/`settings.js`**: Settings page for database configuration
- **`style.css`**: Application styling

## Key Patterns and Conventions

### Rust Code Patterns

1. **Error Handling**: Use `AppResult<T>` type alias for consistent error handling
   ```rust
   pub type AppResult<T> = Result<T, AppError>;
   
   #[tauri::command]
   pub fn some_command() -> AppResult<ResponseType> {
       // Implementation
   }
   ```

2. **Database Operations**: Use `with_conn` closure pattern for database access
   ```rust
   let result = with_conn(|conn| {
       // Database operations here
       Ok(data)
   })?;
   ```

3. **Command Structure**: Tauri commands follow consistent patterns
   ```rust
   #[tauri::command]
   pub fn command_name(param: Type) -> AppResult<ReturnType> {
       // Validation
       // Business logic
       // Return result
   }
   ```

### JavaScript Patterns

1. **Tauri IPC**: Use async/await with Tauri invoke API
   ```javascript
   const { invoke } = window.__TAURI__.tauri;
   const result = await invoke('command_name', { param: value });
   ```

2. **State Management**: Global state object for UI state
   ```javascript
   const state = {
       page: 1,
       pageSize: 20,
       // other state properties
   };
   ```

## Database Schema

The main table is `navigation_history` with these key fields:
- `url` (TEXT PRIMARY KEY)
- `title` (TEXT)
- `last_visited_time` (INTEGER) - Unix timestamp
- `num_visits` (INTEGER)
- `locale` (TEXT)
- Additional metadata fields for future extensions

## Language and Localization

- **Mixed Language**: The codebase uses both Chinese and English
- **UI Labels**: Primarily Chinese for user-facing text
- **Code Comments**: Mix of English and Chinese
- **Error Messages**: Chinese user messages, English technical details

## Development Practices

1. **Code Organization**: Modular structure with clear separation of concerns
2. **Error Handling**: Comprehensive error types with user-friendly messages
3. **Configuration**: JSON-based configuration with validation
4. **Testing**: Cargo test support (minimal tests currently)
5. **Build Scripts**: npm scripts for common development tasks

## Important Files to Know

- **Configuration**: `src-tauri/tauri.conf.json` - Tauri app configuration
- **Dependencies**: `src-tauri/Cargo.toml` and `package.json`
- **Documentation**: `DESIGN.md` - Comprehensive design document
- **Entry Points**: `src-tauri/src/main.rs` and `src/index.html`

## Common Operations

### Adding New Tauri Commands

1. Implement the command in `commands.rs`
2. Add to the handler list in `main.rs`
3. Call from frontend using `invoke()`

### Database Operations

1. Use `with_conn()` for database access
2. Follow the existing query patterns
3. Handle errors appropriately with `AppError`

### Frontend State Updates

1. Update the global `state` object
2. Call corresponding fetch functions
3. Re-render affected UI components

## Development Environment

- **Rust**: Stable toolchain required
- **Node.js**: 18+ required for frontend tooling
- **System Dependencies**: May require GTK/WebKit libraries on Linux
- **Commands**: Use `npm run dev` for development, `npm run build` for production

## Code Style

- **Rust**: Follow standard Rust conventions with `cargo fmt`
- **JavaScript**: ES6+ features, async/await patterns
- **HTML/CSS**: Semantic HTML with CSS Grid/Flexbox layouts
- **Comments**: Document complex business logic and non-obvious code patterns