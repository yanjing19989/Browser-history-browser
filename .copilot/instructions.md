# Copilot Instructions for Browser History Browser

## Project Overview

This is a **Browser History Visualization** desktop application built with Tauri, providing a local, private, and high-performance solution for analyzing browser history data. The application uses a Rust backend with a native JavaScript frontend and SQLite database. It features comprehensive theme management, sortable history lists, detailed item views, and user-friendly interfaces.

Key Features:
- **Theme System**: Auto/light/dark mode with system preference detection
- **Interactive History**: Sortable columns with visual indicators, enhanced pagination with direct page navigation
- **Panel Management**: Toggleable filter and details panels with overlay-based design and responsive visibility
- **Detail Views**: Comprehensive item information with action buttons and enhanced panel management
- **User Feedback**: Toast notifications for all user actions
- **External Integration**: Open links in default browser
- **Browser Database Sync**: Import and manage browser history databases from Chrome, Edge, Firefox, and Safari with enhanced cleanup capabilities
- **Modern UI**: Optimized glass morphism effects with performance improvements, semi-transparent backgrounds, rounded corners, and enhanced visual styling
- **Statistics Dashboard**: Unified KPI cards with top visited sites ranking and improved metrics calculation
- **Performance Optimization**: Reduced GPU usage through optimized backdrop-filter implementation and efficient overlay design

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

## Getting Started

For new contributors working on this project:

1. **Prerequisites**: Ensure you have Rust (stable), Node.js 18+, and system dependencies installed
2. **Initial Setup**: 
   ```bash
   # Clone and install dependencies
   npm install  # or pnpm install
   
   # Verify setup
   npm run check  # Check Rust compilation
   ```
3. **Development Workflow**: 
   ```bash
   npm run dev    # Start development server
   npm run build  # Build for production
   ```
4. **First Steps**: Start by exploring `src/main.js` (frontend) and `src-tauri/src/commands.rs` (backend)

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
  - `stats_overview`: Statistics and KPI calculations with improved site domain extraction
  - `get_config`/`set_db_path`: Configuration management
  - `validate_db_path`/`browse_db_file`: Database file operations
  - `browse_browser_db_file`: Open file dialog for browser database selection
  - `copy_browser_db_to_app`: Copy browser database to application data directory
  - `set_browser_db_path`: Set browser database as application data source
  - `open_db_directory`: Open database directory in file explorer
  - `cleanup_old_dbs`: Clean up old database files automatically
- **`db.rs`**: Database connection management and schema initialization
- **`domain.rs`**: Data structures, error types, and type definitions
- **`config.rs`**: Application configuration handling with JSON persistence

### Frontend (`src/`)

- **`index.html`**: Main application UI with Chinese/English mixed interface, includes theme switcher, enhanced pagination controls, and toggleable panel management
- **`main.js`**: JavaScript application logic using Tauri API (`window.__TAURI__.tauri.invoke`), includes sorting, detail view functionality, pagination enhancements, and panel visibility management with overlay-based interactions
- **`settings.html`/`settings.js`**: Settings page for database configuration, includes theme switcher, browser database synchronization interface, and enhanced file management capabilities
- **`style.css`**: Application styling with dark/light theme support, modern optimized glass morphism effects with performance improvements, overlay-based panel design, and enhanced visual styling
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

4. **Toast Notifications**: User feedback system for actions with success and error variants
   ```javascript
   showToast('Message', 'info'); // or 'error', 'success'
   ```

5. **Pagination Enhancement**: Direct page navigation support
   ```javascript
   // Update pagination state and navigate to specific page
   state.page = targetPage;
   fetchHistory();
   ```

6. **Panel Visibility Management**: Overlay-based design with optimized performance
   ```javascript
   // Toggle panels with CSS class-based visibility
   document.querySelector('.filters').classList.toggle('visible');
   // Click outside to close panels
   document.addEventListener('click', (e) => {
       if (!e.target.closest('.filters')) {
           hideFilters();
       }
   });
   ```

7. **Performance-Optimized Styling**: Reduced backdrop-filter usage for better GPU performance
   ```css
   /* Use backdrop-filter sparingly for performance */
   .panel {
       background: rgba(255, 255, 255, 0.9);
       /* backdrop-filter: blur(10px); - use only when necessary */
   }
   ```

6. **File Dialog Operations**: Browser database file selection and management
   ```javascript
   const browserDbPath = await invoke('browse_browser_db_file');
   await invoke('copy_browser_db_to_app', { sourcePath: browserDbPath });
   ```

7. **Panel State Management**: Control panel visibility and layout
   ```javascript
   // Show/hide panels with overlay design
   const filtersVisible = state.filtersVisible;
   const detailsVisible = state.detailsVisible;
   updateLayout(); // Adjust layout based on panel states
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
4. **Testing**: Currently minimal test coverage - contributions welcome for:
   - Unit tests for database operations (`cargo test`)
   - Integration tests for Tauri commands
   - Frontend JavaScript testing
5. **Build Scripts**: npm scripts for common development tasks
6. **Theme Management**: Systematic theme switching with anti-flicker protection
7. **User Experience**: Interactive sorting, detail views, toast notifications, enhanced pagination, and toggleable panel management
8. **File Management**: Browser database synchronization with cross-platform file operations and enhanced cleanup capabilities
9. **Modern UI Design**: Optimized glass morphism effects with performance considerations, semi-transparent backgrounds, rounded corners, and overlay-based panel design
10. **Statistics Display**: Unified KPI cards with top sites ranking and improved metrics
11. **Performance Optimization**: Reduced GPU usage through minimized backdrop-filter effects and efficient rendering strategies
12. **Layout Management**: Flexible layout system with overlay support and responsive panel management

## Important Files to Know

- **Configuration**: `src-tauri/tauri.conf.json` - Tauri app configuration (includes theme settings)
- **Dependencies**: `src-tauri/Cargo.toml` and `package.json`
- **Documentation**: `DESIGN.md` - Comprehensive design document
- **Entry Points**: `src-tauri/src/main.rs` and `src/index.html`
- **Theme System**: `src/theme.js` and `src/theme-init.js` - Complete theme management
- **Browser Database Sync**: Settings page components for browser database management
- **Enhanced UI**: Modern styling components in `src/style.css` with optimized glass morphism effects, overlay-based design patterns, and performance improvements

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

### Browser Database Synchronization

1. Browse browser database files: `invoke('browse_browser_db_file')`
2. Copy browser database to app: `invoke('copy_browser_db_to_app', { sourcePath })`
3. Set browser database as data source: `invoke('set_browser_db_path', { path })`
4. Open database directory: `invoke('open_db_directory')`
5. Clean up old databases: `invoke('cleanup_old_dbs')` - now handles .db, .db-shm, and .db-wal files

### Panel Visibility Management

1. Toggle filter panel visibility: `toggleFilters()` - uses CSS class-based visibility with overlay design
2. Show details panel: `showDetails(item)` - displays item details in overlay panel
3. Hide details panel: `hideDetails()` - closes details panel and updates layout
4. Update layout based on panel states: `updateLayout()` - responsive layout adjustment
5. Click outside to close: Automatic panel closure when clicking outside panel areas

### Performance Optimization

1. Use backdrop-filter sparingly for better GPU performance
2. Prefer CSS transforms and opacity for animations
3. Minimize use of expensive visual effects during interactions
4. Use overlay-based design patterns for better performance
5. Optimize panel transitions with efficient CSS properties

### Enhanced Pagination

1. Update page input field for direct navigation
2. Synchronize pagination state with UI components
3. Handle pagination bounds and validation
4. Update total page count display

### Frontend State Updates

1. Update the global `state` object (including sort parameters and panel visibility states)
2. Call corresponding fetch functions
3. Re-render affected UI components
4. Update sort indicators for table headers
5. Handle pagination state changes and page input validation
6. Manage panel visibility states (`filtersVisible`, `detailsVisible`)
7. Update layout responsively based on panel states
8. Handle overlay interactions and click-outside-to-close functionality

## Development Environment

- **Rust**: Stable toolchain required
- **Node.js**: 18+ required for frontend tooling
- **System Dependencies**: 
  - Linux: `libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libsoup2.4-dev`
  - Windows/macOS: Usually no additional dependencies needed
- **Commands**: Use `npm run dev` for development, `npm run build` for production

## Code Style

- **Rust**: Follow standard Rust conventions with `cargo fmt`
- **JavaScript**: ES6+ features, async/await patterns, event-driven panel management
- **HTML/CSS**: Semantic HTML with Flexbox layouts for responsive design, overlay-based panel patterns, theme-aware styling
- **Comments**: Document complex business logic and non-obvious code patterns
- **Theme Classes**: Use CSS custom properties for theme-specific values
- **User Feedback**: Provide toast notifications for user actions
- **Performance**: Minimize backdrop-filter usage, prefer transforms and opacity for animations
- **Panel Design**: Use overlay patterns with CSS class-based visibility management
- **Layout Management**: Implement responsive layouts that adapt to panel states

## Testing Guidelines

Currently the project has minimal test coverage. When adding tests:

### Rust Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_operations() {
        // Test database functions
    }
}
```

### Frontend Testing  
- Use browser dev tools for debugging
- Test theme switching across all modes
- Verify toast notifications appear correctly
- Test panel toggle functionality

### Integration Testing
- Verify Tauri commands work end-to-end
- Test database file operations
- Validate configuration loading/saving

## Troubleshooting

### Common Development Issues

1. **Build Failures on Linux**: Install system dependencies:
   ```bash
   sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libsoup2.4-dev
   ```

2. **Database Connection Issues**: 
   - Check database path in configuration
   - Ensure SQLite file permissions are correct
   - Use `npm run check` to verify Rust compilation

3. **Theme Flicker**: 
   - Ensure `theme-init.js` loads before other scripts
   - Check localStorage permissions

4. **Panel Layout Issues**:
   - Verify CSS class names match JavaScript selectors
   - Check for conflicting z-index values

### Performance Issues
- If UI feels sluggish, check for excessive backdrop-filter usage
- Monitor browser dev tools for memory leaks
- Reduce animation complexity if needed

## Contribution Guidelines

1. **Code Style**: Follow existing patterns and run `cargo fmt` for Rust code
2. **Error Handling**: Always use `AppResult<T>` for Tauri commands
3. **Testing**: Add tests for new functionality when possible
4. **Documentation**: Update this file when adding new patterns or commands
5. **Performance**: Consider performance impact of visual effects and database operations
6. **Accessibility**: Ensure new UI elements are accessible
7. **Localization**: Consider Chinese/English mixed interface requirements