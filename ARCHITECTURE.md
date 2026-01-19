# 🏗️ Architecture

## Core Components

- **AppState**: Main application state holding todos, list selection, focused view, and input value
- **TodoItem**: Data structure for individual todos with title and completion status
- **FocusedView**: Enum tracking which view (List or Add) currently has focus
- **FormActions**: Enum for handling form submission/cancellation

## File Structure

```
ratta/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project dependencies and metadata
└── README.md           # This file
```

## Data Persistence

Todos are automatically saved to `~/.ratta_todos.json` in JSON format:

```json
[
  {
    "title": "Complete the project",
    "completed": false
  },
  {
    "title": "Write documentation",
    "completed": true
  }
]
```

## Technical Details

### Event Loop

The application runs on a simple event loop:

1. **Render**: Draw the current UI state to the terminal
2. **Handle Input**: Process keyboard events and update state
3. **Repeat**: Continue until user quits

### State Management

- **ListState**: Ratatui's stateful widget state for tracking list selection
- **FocusedView**: Controls which view receives keyboard input
- **Input Buffer**: Temporary storage for text being typed in Add view

### Error Handling

Uses `color-eyre` for beautiful error messages with full context and suggestions. All fallible operations return `Result<T>` for proper error propagation.

## Development

### Running in Debug Mode

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
./target/release/ratta
```

### Code Style

The codebase includes extensive inline comments explaining:

- Function purposes and parameters
- Ratatui concepts and patterns
- State management logic
- Keyboard event handling
