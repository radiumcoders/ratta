use color_eyre::{ eyre::{ Ok, Result } };
use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{ self, Event, KeyEvent },
    layout::{ Constraint, Layout },
    style::{ Color, Style, Stylize },
    symbols::block,
    text::ToSpan,
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

enum FormActions {
    None,
    Submit,
    Cancel,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();
    state.can_add_new = false;
    state.list_state.select(Some(0));

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
                match handle_add_new_key(key, app_state) {
                    FormActions::Cancel => {
                        app_state.can_add_new = false;
                        app_state.input_value.clear();
                    }
                    FormActions::None => {}
                    FormActions::Submit => {
                        app_state.can_add_new = false;
                        app_state.items.push(TodoItem {
                            title: app_state.input_value.drain(..).collect(),
                            completed: false,
                        });
                        app_state.input_value.clear();
                    }
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

fn handle_add_new_key(key: KeyEvent, app_state: &mut AppState) -> FormActions {
    match key.code {
        event::KeyCode::Esc => {
            // break will not work here so
            return FormActions::Cancel;
        }
        event::KeyCode::Enter => {
            // Here you would normally add the new item to the list
            // For now, we just exit the add new mode
            return FormActions::Submit;
        }
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        _ => {}
    }
    FormActions::None
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
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        Paragraph::new(app_state.input_value.as_str())
            .style(Style::default())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .border_type(BorderType::Double)
                    .title(
                        String::from(
                            "[  Enter new todo item (Press Enter to submit, Esc to cancel)  ]"
                        )
                            .to_span()
                            .into_centered_line()
                    )
                    .padding(Padding::symmetric(2, 1))
            )
            .render(border_area, frame.buffer_mut());
    } else {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Double)
            .fg(Color::Yellow)
            .title(
                String::from("[  RATTA - a minimal todo list management app written in rust :)  ]")
                    .to_span()
                    .into_centered_line()
            )
            .render(border_area, frame.buffer_mut());

        let list = List::new(
            app_state.items.iter().map(|x| ListItem::from(x.title.as_str()).fg(Color::White))
        )
            .highlight_symbol("->")
            .highlight_style(Style::default().fg(Color::Magenta))
            .block(Block::new().padding(Padding::symmetric(2, 1)));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
