use crossterm::{
    execute,
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor::MoveTo,
    Result,
};
use std::io::{stdout, Write};


mod models;
mod rep;
use rep::v2::V2;
use rep::line::Line;

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Enable raw mode to handle input events
    terminal::enable_raw_mode()?;

    // Clear the screen
    execute!(stdout, Clear(ClearType::All))?;

    let center = V2::new(15, 15);
    let mut end = V2::new(10, 8);
    let mut inc = 1;

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

        execute!(stdout, Clear(ClearType::All))?;
        for p in Line::new(center, center.add(&end)).as_iter() {
            execute!(
                stdout,
                MoveTo(p.x as u16, p.y as u16),
                Print('@')
            )?;
        }
        end.y *= -1;
        for p in Line::new(center, center.add(&end)).as_iter() {
            execute!(
                stdout,
                MoveTo(p.x as u16, p.y as u16),
                Print('@')
            )?;
        }
        end.x *= -1;
        for p in Line::new(center, center.add(&end)).as_iter() {
            execute!(
                stdout,
                MoveTo(p.x as u16, p.y as u16),
                Print('@')
            )?;
        }
        end.y *= -1;
        for p in Line::new(center, center.add(&end)).as_iter() {
            execute!(
                stdout,
                MoveTo(p.x as u16, p.y as u16),
                Print('@')
            )?;
        }
        end.x *= -1;

        // print!("\n");

        // Draw ASCII characters
        // execute!(
        //     stdout,
        //     SetForegroundColor(Color::Red),
        //     Print('@'),
        //     ResetColor
        // )?;

        end.y += inc;
        if end.y >= 10 || end.y <= 0{
            inc *= -1
        }
    }

    // Clear the screen and disable raw mode before exiting
    execute!(stdout, Clear(ClearType::All))?;
    terminal::disable_raw_mode()?;

    Ok(())
}
