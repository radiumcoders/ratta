# 🐀 Ratta

A beautiful terminal-based todo list application built with Rust and Ratatui.

![Ratta Preview](public/preview.png)

![Rust Version](https://img.shields.io/badge/rust-2024-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## 📚 Documentation

- [Features](FEATURES.md) - Core features overview
- [Installation & Usage](INSTALLATION.md) - Setup and keyboard shortcuts
- [Architecture](ARCHITECTURE.md) - Technical details and design
- [Dependencies](DEPENDENCIES.md) - Project dependencies
- [Limitations](LIMITATIONS.md) - Known limitations
- [Contributing](CONTRIBUTING.md) - How to contribute

## 🎨 Preview

The application features a split-pane layout:

```
┌────────────────────────────────────────────────┐
│  RATTA - Todo List (Navigation shortcuts)     │
│                                                │
│  -> Buy groceries                             │
│     Write documentation (strikethrough)        │
│     Call mom                                   │
│                                                │
│                                                │
└────────────────────────────────────────────────┘
┌────────────────────────────────────────────────┐
│  Add New (Press A to focus, Enter to submit)  │
│                                                │
│  [Your input appears here]                     │
│                                                │
└────────────────────────────────────────────────┘
```

- **Top Pane**: Todo list with selection highlighting (cyan border when focused)
- **Bottom Pane**: Add new todo input field (green border when focused)

## Future Plans

We have exciting plans to enhance Ratta with:

### Dashboard & Analytics

- **Progress Graphs**: Visual charts showing completed vs remaining tasks
- **Statistics Pane**: Display completion rate, total tasks, and daily/weekly progress
- **Charts Integration**: ASCII-based bar and line charts for terminal compatibility

### Enhanced Features

- **Better Preview**: Detailed task preview with metadata (creation date, completion date, estimated time)
- **Task Filtering**: Filter by status, completion date, or search keywords
- **Statistics Dashboard**: Overview of productivity metrics and trends
- **Color Themes**: Customizable color schemes for personalization
- **Performance Metrics**: Track how many tasks you complete per day/week

### Improvements

- Add todo editing capabilities
- Implement categories/tags for organization
- Add priority levels and sorting
- Support for todo reordering (drag & drop)
- Export/import functionality (CSV, JSON formats)
- Recurring tasks support
- Dark and light theme options

## 📝 License

This project is available under the MIT License.

## 🙏 Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) - a Rust terminal UI library
- Inspired by terminal-based productivity tools

---

Made with 🦀 and Rust and ❤️ from RadiumCoders
