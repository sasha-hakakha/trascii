mod models;
use crossterm::{
    execute,
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Enable raw mode to handle input events
    terminal::enable_raw_mode()?;

    // Clear the screen
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        // Poll for an event
        if event::poll(std::time::Duration::from_millis(100))? {
            let evt = event::read()?;

            // If the user presses 'q', exit the loop
            if let Event::Key(key_event) = evt {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Draw ASCII characters
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print('@'),
            ResetColor
        )?;
    }

    // Clear the screen and disable raw mode before exiting
    execute!(stdout, Clear(ClearType::All))?;
    terminal::disable_raw_mode()?;

    Ok(())
}
