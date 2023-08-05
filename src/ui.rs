use crossterm::style;
use ratatui::widgets::Widget;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    
    let osint_data = app.osint_data.join("\n"); // Join the OSINT data with newlines

    frame.render_widget(
        Paragraph::new(format!(
            "This is your OSINT Tool.\n\
                Press `Esc`, `Ctrl-C`, or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}\n\n\
                OSINT Data:\n\
                {}",
            app.counter, osint_data
        ))
        .block(
            Block::default()
                .title("OSINT Tool")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::DarkGray) // Use dark gray as the background color
        )
        .alignment(Alignment::Center),
        frame.size(),
    )
}