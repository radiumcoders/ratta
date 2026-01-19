# 🚀 Installation

## Prerequisites

- Rust (2024 edition) and Cargo
- A terminal emulator that supports modern terminal features

## Running the Application

1. Download the ZIP file from the releases
2. Extract the archive
3. Navigate to the extracted folder
4. Run:

```bash
cargo run -q
```

## Keyboard Shortcuts

### List View (Default)

| Key         | Action                                    |
| ----------- | ----------------------------------------- |
| `↑` / `k`   | Move selection up                         |
| `↓` / `j`   | Move selection down                       |
| `Enter`     | Toggle completion status of selected todo |
| `A`         | Switch to Add view to create a new todo   |
| `D`         | Delete the selected todo                  |
| `q` / `Esc` | Quit the application                      |

### Add View

| Key                | Action                                   |
| ------------------ | ---------------------------------------- |
| Type any character | Add to todo text                         |
| `Backspace`        | Delete last character                    |
| `Enter`            | Save the new todo and return to list     |
| `Esc`              | Cancel and return to list without saving |
