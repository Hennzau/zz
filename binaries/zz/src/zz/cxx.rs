use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{self, RED},
        Color, Stylize,
    },
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};

pub struct CXXTab {}

impl Widget for &CXXTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Not available yet.")
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(RED.c700),
            )
            .render(area, buf);
    }
}

impl CXXTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_events(&mut self, event: Event) -> (super::AppState, Option<super::SelectedTab>) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return (super::AppState::Quitting, None),
                KeyCode::Right => {
                    return (
                        super::AppState::Running,
                        Some(super::SelectedTab::new_zed()),
                    )
                }
                KeyCode::Left => {
                    return (
                        super::AppState::Running,
                        Some(super::SelectedTab::new_rust()),
                    )
                }
                _ => {}
            }
        }

        (super::AppState::Running, None)
    }
}
