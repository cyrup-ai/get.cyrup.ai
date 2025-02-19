use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};
use std::io::{self, Stdout};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;

pub struct Installer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    progress: f64,
    status: String,
}

impl Installer {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            progress: 0.0,
            status: String::from("Starting installation..."),
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.draw()?;
        Ok(())
    }

    pub fn update_progress(&mut self, progress: f64) -> Result<()> {
        self.progress = progress;
        self.draw()?;
        Ok(())
    }

    pub fn update_status(&mut self, status: impl Into<String>) -> Result<()> {
        self.status = status.into();
        self.draw()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();
            
            // Create centered box for content
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Cyrup Installer");
            
            let inner_area = block.inner(area);
            frame.render_widget(block, area);

            // Calculate layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(1), // Progress bar
                    Constraint::Length(1), // Status
                ].as_ref())
                .split(inner_area);

            // Render progress bar
            let gauge = Gauge::default()
                .block(Block::default())
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(self.progress);
            frame.render_widget(gauge, chunks[0]);

            // Render status text
            let status = Paragraph::new(self.status.as_str())
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(status, chunks[1]);

            // Render help text at bottom
            let help = Paragraph::new("Press Ctrl+C to cancel")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(help, inner_area.inner(Margin { 
                vertical: chunks[1].bottom() + 1,
                horizontal: 0 
            }));
        })?;

        Ok(())
    }
}

impl Drop for Installer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
        );
    }
}
