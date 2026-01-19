use color_eyre::{ eyre::{ Ok, Result } };
use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{ self, Event },
    layout::{ Constraint, Layout },
    style::{ Color, Stylize },
    symbols::border,
    widgets::{ Block, BorderType, List, ListItem, Paragraph, Widget },
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
}

#[derive(Debug, Default)]
struct TodoItem {
    title: String,
    completed: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

    state.items.push(TodoItem {
        title: "Learn Rust".to_string(),
        completed: false,
    });
    state.items.push(TodoItem {
        title: "Build a TUI app".to_string(),
        completed: false,
    });

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    //renderig
    terminal.draw(|f| render(f, app_state))?;
    //handling inputs
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
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

    List::new(app_state.items.iter().map(|x| ListItem::from(x.title.clone()))).render(
        inner_area,
        frame.buffer_mut()
    );
}
