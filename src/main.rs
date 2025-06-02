use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Paragraph, Block, Borders},
    layout::{Layout, Constraint, Direction, Margin},
    style::{Style, Color},
};
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Temperature Converter: Farenheit to Celsius");

    print!("Type the temperature in Farenheit: ");
    io::stdout().flush().unwrap();

    let mut faren = String::new();

    io::stdin().read_line(&mut faren)?;
    let faren : i32 = faren.trim().parse()?;

    let result = faren_to_celsius(faren);
    println!("Result: {}°F is {}°C", faren, result);

    Ok(())
}

fn faren_to_celsius(faren : i32) -> i32 {
    ((faren - 32) * 5 ) / 9
}
