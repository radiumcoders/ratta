// Error handling library for better error messages
use color_eyre::{ eyre::{ Ok, Result } };

// Ratatui - Terminal User Interface library
use ratatui::{
    DefaultTerminal, // Pre-configured terminal for quick setup
    Frame, // Represents a single frame to render widgets to
    crossterm::event::{ self, Event, KeyEvent }, // Keyboard/mouse event handling
    layout::{ Constraint, Layout }, // Layout system for positioning widgets
    style::{ Color, Style, Stylize }, // Styling for text and widgets
    text::ToSpan, // Convert strings to styled text spans
    widgets::{ Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget }, // UI widgets
};

// Tracks which view currently has focus (receives keyboard input)
// PartialEq lets us compare with == operator
#[derive(Debug, Default, PartialEq)]
enum FocusedView {
    #[default] // List view is focused by default when app starts
    List,
    Add,
}

// Main application state - holds all data needed for the UI
#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>, // List of all todo items
    list_state: ListState, // Tracks which item is selected in the list (ratatui's stateful widget state)
    focused_view: FocusedView, // Which view (List or Add) currently has focus
    input_value: String, // Text being typed in the add input field
}

// Represents a single todo item
#[derive(Debug, Default)]
struct TodoItem {
    title: String, // The todo text
    completed: bool, // Whether it's been marked as done
}

// Return values from the add form's key handler
// Tells the main loop what action to take
enum FormActions {
    None, // No special action, continue editing
    Submit, // User pressed Enter - save the item
    Cancel, // User pressed Esc - discard changes
}

fn main() -> Result<()> {
    // Install better error formatting
    color_eyre::install()?;

    // Create initial application state
    let mut state = AppState::default();
    state.focused_view = FocusedView::List; // Start with list focused
    state.list_state.select(Some(0)); // Select first item (if list not empty, this will highlight it)

    // Initialize the terminal (enters raw mode, alternate screen)
    let terminal = ratatui::init();

    // Run the main application loop
    let result = run(terminal, &mut state);

    // Restore terminal to normal mode (very important!)
    ratatui::restore();
    result
}

// Main application loop: render -> handle input -> repeat
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // RENDERING: Draw the UI to the terminal
        // terminal.draw() takes a closure that receives a Frame to draw on
        terminal.draw(|f| render(f, app_state))?;

        // INPUT HANDLING: Wait for and process keyboard events
        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            if app_state.focused_view == FocusedView::Add {
                // Add view has focus - handle text input
                match handle_add_new_key(key, app_state) {
                    FormActions::Cancel => {
                        // User pressed Esc - return to list and discard input
                        app_state.focused_view = FocusedView::List;
                        app_state.input_value.clear();
                    }
                    FormActions::None => {}
                    FormActions::Submit => {
                        // User pressed Enter - save new todo item
                        app_state.focused_view = FocusedView::List;
                        app_state.items.push(TodoItem {
                            title: app_state.input_value.drain(..).collect(), // drain() moves string out
                            completed: false,
                        });
                        app_state.input_value.clear();
                    }
                }
            } else {
                // List view has focus - handle navigation and commands
                if handle_key(key, app_state) {
                    break; // handle_key returns true when user wants to quit
                }
            }
        }
    }
    Ok(())
}

// Handle keyboard input when Add view has focus
// Returns FormActions to tell caller what to do
fn handle_add_new_key(key: KeyEvent, app_state: &mut AppState) -> FormActions {
    match key.code {
        event::KeyCode::Esc => {
            // Cancel and return to list
            return FormActions::Cancel;
        }
        event::KeyCode::Enter => {
            // Submit the new todo item
            return FormActions::Submit;
        }
        event::KeyCode::Char(c) => {
            // Regular character - append to input string
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            // Delete last character
            app_state.input_value.pop();
        }
        _ => {} // Ignore other keys
    }
    FormActions::None // No special action, continue editing
}

// Handle keyboard input when List view has focus
// Returns true if user wants to quit the app
fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            // Quit the application
            return true;
        }
        event::KeyCode::Enter => {
            // Toggle completion status of selected item
            if let Some(selected) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(selected) {
                    item.completed = !item.completed;
                }
            }
        }
        // Arrow keys for navigation
        event::KeyCode::Down => {
            app_state.list_state.select_next(); // Built-in method to move selection down
        }
        event::KeyCode::Up => {
            app_state.list_state.select_previous(); // Built-in method to move selection up
        }
        // Character-based commands
        event::KeyCode::Char(char) => {
            match char {
                'q' => {
                    // Quit the application
                    return true;
                }
                'j' => app_state.list_state.select_next(), // Vim-style down
                'k' => app_state.list_state.select_previous(), // Vim-style up
                'D' => {
                    // Delete the selected item
                    if let Some(selected) = app_state.list_state.selected() {
                        app_state.items.remove(selected);
                    }
                }
                'A' => {
                    // Switch focus to Add view (like Vim insert mode)
                    app_state.focused_view = FocusedView::Add;
                }
                _ => {} // Ignore other characters
            }
        }
        _ => {} // Ignore other keys
    }
    false // Don't quit
}

// Main render function - called every frame
// Divides screen into two horizontal sections and renders both views
fn render(frame: &mut Frame, app_state: &mut AppState) {
    // Create a vertical split layout:
    // - Top section: Fill remaining space (the list)
    // - Bottom section: Fixed 5 lines tall (the add input)
    let [list_area, add_area] = Layout::vertical([
        Constraint::Fill(1), // List takes all remaining space
        Constraint::Length(5), // Add view is fixed at 5 lines
    ])
        .margin(1) // 1-character margin around entire layout
        .areas(frame.area()); // Split the full frame area

    // Render both views (the focused one will have highlighted border)
    render_list_view(frame, app_state, list_area);
    render_add_view(frame, app_state, add_area);
}

// Render the add/input view at the bottom of the screen
fn render_add_view(frame: &mut Frame, app_state: &mut AppState, area: ratatui::layout::Rect) {
    // Visual feedback: change border style based on focus
    let is_focused = app_state.focused_view == FocusedView::Add;
    let border_color = if is_focused { Color::Green } else { Color::DarkGray };
    let border_type = if is_focused { BorderType::Double } else { BorderType::Plain };

    // Paragraph widget displays text (in this case, what user is typing)
    Paragraph::new(app_state.input_value.as_str())
        .style(Style::default())
        .block(
            Block::bordered() // Add a border around the paragraph
                .fg(border_color) // Set border color
                .border_type(border_type) // Single or double line border
                .title(
                    // Centered title showing available commands
                    String::from("[  Add New (Press A to focus, Enter to submit, Esc to cancel)  ]")
                        .to_span()
                        .into_centered_line()
                )
                .padding(Padding::horizontal(2)) // 2 spaces padding on left/right
        )
        .render(area, frame.buffer_mut()); // Draw the widget to this area
}

// Render the todo list view at the top of the screen
fn render_list_view(frame: &mut Frame, app_state: &mut AppState, area: ratatui::layout::Rect) {
    // Visual feedback: change border style based on focus
    let is_focused = app_state.focused_view == FocusedView::List;
    let border_color = if is_focused { Color::LightCyan } else { Color::DarkGray };
    let border_type = if is_focused { BorderType::Double } else { BorderType::Plain };

    // Create inner area with 1-line margin inside the border
    // This gives space for the list items inside the bordered box
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    // First, draw the border and title
    Block::bordered()
        .border_type(border_type)
        .fg(border_color)
        .title(
            // Centered title with keyboard shortcuts
            String::from(
                "[  RATTA - Todo List (j/k or ↑/↓ to move, Enter to toggle, D to delete)  ]"
            )
                .to_span()
                .into_centered_line()
        )
        .render(area, frame.buffer_mut());

    // Create the List widget with all todo items
    let list = List::new(
        // Convert each TodoItem into a ListItem widget
        app_state.items.iter().map(|x| {
            // Completed items get strikethrough styling
            let value = if x.completed {
                x.title.to_span().crossed_out()
            } else {
                x.title.to_span()
            };
            ListItem::new(value).fg(Color::White)
        })
    )
        .highlight_symbol("->") // Show -> next to selected item
        .highlight_style(Style::default().fg(Color::Magenta)) // Selected item in magenta
        .block(Block::new().padding(Padding::symmetric(2, 1))); // Padding inside list

    // render_stateful_widget: Renders a widget that maintains selection state
    // The ListState tracks which item is selected
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
