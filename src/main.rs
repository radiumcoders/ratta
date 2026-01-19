use color_eyre::{ eyre::{ Ok, Result } };
use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{ self, Event },
    layout::{ Constraint, Layout },
    style::{ Color, Style },
    widgets::{ Block, BorderType, List, ListItem, ListState, Widget },
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct TodoItem {
    title: String,
    completed: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

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
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => {
                    match char {
                        'j' => app_state.list_state.select_next(),
                        'k' => app_state.list_state.select_previous(),
                        'D' => {
                            if let Some(selected) = app_state.list_state.selected() {
                                app_state.items.remove(selected);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::LightDoubleDashed)
        // .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|x| ListItem::from(x.title.clone())))
        .highlight_symbol("->")
        .highlight_style(Style::default().fg(Color::Magenta));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
