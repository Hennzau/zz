use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode},
    layout::Rect,
    style::palette::tailwind::RED,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget},
};

pub struct RustTab {}

impl Widget for &RustTab {
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

impl RustTab {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {})
    }

    pub fn handle_events(
        &mut self,
        event: Event,
    ) -> eyre::Result<(super::AppState, Option<super::SelectedTab>)> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => return Ok((super::AppState::Quitting, None)),
                KeyCode::Right => {
                    return Ok((
                        super::AppState::Running,
                        Some(super::SelectedTab::new_cxx()?),
                    ))
                }
                KeyCode::Left => {
                    return Ok((
                        super::AppState::Running,
                        Some(super::SelectedTab::new_python()?),
                    ))
                }
                _ => {}
            }
        }

        Ok((super::AppState::Running, None))
    }
}
