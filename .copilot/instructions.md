# Copilot Instructions for Browser History Browser

## Project Overview

This is a **Browser History Visualization** desktop application built with Tauri, providing a local, private, and high-performance solution for analyzing browser history data. The application uses a Rust backend with a native JavaScript frontend and SQLite database. It features comprehensive theme management, sortable history lists, detailed item views, and user-friendly interfaces.

Key Features:
- **Theme System**: Auto/light/dark mode with system preference detection
- **Interactive History**: Sortable columns with visual indicators
- **Detail Views**: Comprehensive item information with action buttons
- **User Feedback**: Toast notifications for all user actions
- **External Integration**: Open links in default browser

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
  - `list_history`: Paginated history listing with filters and sorting support
  - `stats_overview`: Statistics and KPI calculations
  - `get_config`/`set_db_path`: Configuration management
  - `validate_db_path`/`browse_db_file`: Database file operations
- **`db.rs`**: Database connection management and schema initialization
- **`domain.rs`**: Data structures, error types, and type definitions
- **`config.rs`**: Application configuration handling with JSON persistence

### Frontend (`src/`)

- **`index.html`**: Main application UI with Chinese/English mixed interface, includes theme switcher
- **`main.js`**: JavaScript application logic using Tauri API (`window.__TAURI__.tauri.invoke`), includes sorting and detail view functionality
- **`settings.html`/`settings.js`**: Settings page for database configuration, includes theme switcher
- **`style.css`**: Application styling with dark/light theme support
- **`theme.js`**: Theme management system with auto/light/dark modes
- **`theme-init.js`**: Prevents theme flicker by applying theme before page render

## Key Patterns and Conventions

### Theme Management

1. **Theme System**: Three modes supported: `auto`, `light`, `dark`
   ```javascript
   // ThemeManager handles theme switching and persistence
   window.themeManager.cycleTheme(); // Cycle through themes
   window.themeManager.isDarkMode(); // Check current effective theme
   ```

2. **Anti-flicker Strategy**: Theme applied before page render
   ```javascript
   // theme-init.js runs synchronously before DOM render
   // Prevents white flash when loading dark theme
   ```

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

2. **State Management**: Global state object for UI state with sorting support
   ```javascript
   const state = {
       page: 1,
       pageSize: 20,
       sortBy: 'last_visited_time',
       sortOrder: 'desc',
       // other state properties
   };
   ```

3. **External Link Handling**: Use Tauri shell API for opening links
   ```javascript
   const { shell } = window.__TAURI__;
   await shell.open(url);
   ```

4. **Toast Notifications**: User feedback system for actions
   ```javascript
   showToast('Message', 'info'); // or 'error'
   ```

## Database Schema

The main table is `navigation_history` with these key fields:
- `url` (TEXT PRIMARY KEY)
- `title` (TEXT)
- `last_visited_time` (INTEGER) - Unix timestamp (sortable)
- `num_visits` (INTEGER) - Visit count (sortable)
- `locale` (TEXT)
- Additional metadata fields for future extensions

Sorting is supported on `title`, `last_visited_time`, and `num_visits` fields.

## Language and Localization

- **Mixed Language**: The codebase uses both Chinese and English
- **UI Labels**: Primarily Chinese for user-facing text
- **Code Comments**: Mix of English and Chinese
- **Error Messages**: Chinese user messages, English technical details
- **Theme Labels**: Chinese UI with emoji indicators (🌓 自动, ☀️ 浅色, 🌙 深色)

## Development Practices

1. **Code Organization**: Modular structure with clear separation of concerns
2. **Error Handling**: Comprehensive error types with user-friendly messages
3. **Configuration**: JSON-based configuration with validation
4. **Testing**: Cargo test support (minimal tests currently)
5. **Build Scripts**: npm scripts for common development tasks
6. **Theme Management**: Systematic theme switching with anti-flicker protection
7. **User Experience**: Interactive sorting, detail views, and toast notifications

## Important Files to Know

- **Configuration**: `src-tauri/tauri.conf.json` - Tauri app configuration (includes theme settings)
- **Dependencies**: `src-tauri/Cargo.toml` and `package.json`
- **Documentation**: `DESIGN.md` - Comprehensive design document
- **Entry Points**: `src-tauri/src/main.rs` and `src/index.html`
- **Theme System**: `src/theme.js` and `src/theme-init.js` - Complete theme management

## Common Operations

### Adding New Tauri Commands

1. Implement the command in `commands.rs`
2. Add to the handler list in `main.rs`
3. Call from frontend using `invoke()`

### Theme Management

1. Toggle themes programmatically: `window.themeManager.cycleTheme()`
2. Check current theme: `window.themeManager.getEffectiveTheme()`
3. Add theme-aware styling using `data-theme` attribute

### Database Operations

1. Use `with_conn()` for database access
2. Follow the existing query patterns with sorting support
3. Handle errors appropriately with `AppError`

### Frontend State Updates

1. Update the global `state` object (including sort parameters)
2. Call corresponding fetch functions
3. Re-render affected UI components
4. Update sort indicators for table headers

## Development Environment

- **Rust**: Stable toolchain required
- **Node.js**: 18+ required for frontend tooling
- **System Dependencies**: May require GTK/WebKit libraries on Linux
- **Commands**: Use `npm run dev` for development, `npm run build` for production

## Code Style

- **Rust**: Follow standard Rust conventions with `cargo fmt`
- **JavaScript**: ES6+ features, async/await patterns
- **HTML/CSS**: Semantic HTML with CSS Grid/Flexbox layouts, theme-aware styling
- **Comments**: Document complex business logic and non-obvious code patterns
- **Theme Classes**: Use CSS custom properties for theme-specific values
- **User Feedback**: Provide toast notifications for user actions