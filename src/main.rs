use color_eyre::{ eyre::{ Ok, Result } };
use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{ self, Event, KeyEvent },
    layout::{ Constraint, Layout },
    style::{ Color, Style, Stylize },
    symbols::block,
    widgets::{ Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget },
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    can_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    title: String,
    completed: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();
    state.can_add_new = false;
    // Populate with sample data
    state.items.push(TodoItem {
        title: "Learn Rust".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "Build a TUI app".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "fdfd".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "gege".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "dgdgs".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "sdtst".to_string(),
        completed: false,
    });
    // dummy data end

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        //rendering
        terminal.draw(|f| render(f, app_state))?;
        //handling inputs
        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            if app_state.can_add_new {
                if handle_add_new_key(key, app_state) {
                    app_state.can_add_new = false;
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            // break will not work here so
            return true;
        }
        event::KeyCode::Enter => {
            // Here you would normally add the new item to the list
            // For now, we just exit the add new mode
            return true;
        }
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        _ => {}
    }
    false
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            // break will not work here so
            return true;
        }
        event::KeyCode::Char(char) => {
            match char {
                'q' => {
                    return true;
                }
                'j' => app_state.list_state.select_next(),
                'k' => app_state.list_state.select_previous(),
                'D' => {
                    if let Some(selected) = app_state.list_state.selected() {
                        app_state.items.remove(selected);
                    }
                }
                'A' => {
                    app_state.can_add_new = true;
                }
                _ => {}
            }
        }
        _ => {}
    }
    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    if app_state.can_add_new {
        //borrow!!!
        Paragraph::new(app_state.input_value.as_str())
            .style(Style::default())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .border_type(BorderType::Double)
                    .title("Add New Item")
            )
            .render(frame.area(), frame.buffer_mut());
    } else {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Double)
            // .fg(Color::Yellow)
            .title(String::from("RATTA - a minimal todo list management app written in rust :)"))
            .render(border_area, frame.buffer_mut());

        let list = List::new(app_state.items.iter().map(|x| ListItem::from(x.title.as_str())))
            .highlight_symbol("->")
            .highlight_style(Style::default().fg(Color::Magenta))
            .block(Block::new().padding(Padding::symmetric(2, 1)));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
