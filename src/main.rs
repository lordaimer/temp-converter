use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Paragraph, Block},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    // 1. enable raw mode
    // 2. enter alternate screen
    // 3. create a crossterm backend for ratatui to work with
    // 4. creates a new terminal instance from the backend
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Input buffer and state
    let mut input = String::new();
    let mut output: Option<String> = None;

    loop { // Draw UI
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Length(3), // Input prompt
                    Constraint::Length(3), // Output result or instructions
                    Constraint::Min(0),    // Flexible space for future (if any)
                    Constraint::Length(1), // Footer line
                ].as_ref())
                .split(size);

            // Title
            let title = Paragraph::new("Temperature Converter")
                .style(Style::default().fg(Color::Green));
            f.render_widget(title, chunks[0]);

            // Input prompt
            let input_text = Paragraph::new(format!("Enter Fahrenheit: {}", input))
                .block(Block::default().borders(ratatui::widgets::Borders::ALL));
            f.render_widget(input_text, chunks[1]);

            // Output text
            let output_text = Paragraph::new(
                output.as_deref().unwrap_or("Type a number and press Enter to convert")
            );
            f.render_widget(output_text, chunks[2]);

            // Footer: split horizontally into two halves
            let footer_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(chunks[4]);

            // Left footer: "Press Esc to Quit"
            let left_footer = Paragraph::new("Press Esc to Quit")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(left_footer, footer_chunks[0]);

            // Right footer: "Author: lordaimer" (aligned right)
            let right_footer = Paragraph::new("Author: lordaimer")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(ratatui::layout::Alignment::Right);
            f.render_widget(right_footer, footer_chunks[1]);
        })?;

        // Handle input events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                #[cfg(windows)]
                {
                    use crossterm::event::KeyEventKind;
                    if key.kind != KeyEventKind::Press {
                        continue; // ignore repeats and releases
                    }
                }
                match key.code {
                    KeyCode::Char(c) => {
                        if c.is_digit(10) || c == '-' {
                            input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        // Try parse and convert
                        match input.trim().parse::<i32>() {
                            Ok(f) => {
                                let c = faren_to_celsius(f);
                                output = Some(format!("{}°F = {}°C", f, c));
                            }
                            Err(_) => {
                                output = Some("Invalid number! Try again.".to_string());
                            }
                        }
                        input.clear();
                    }
                    KeyCode::Esc => {
                        break; // Exit on ESC
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn faren_to_celsius(faren: i32) -> i32 {
    ((faren - 32) * 5) / 9
}

