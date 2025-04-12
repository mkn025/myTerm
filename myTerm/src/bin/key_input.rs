use crossterm::{terminal, event::{self, Event, KeyCode, KeyEventKind}};
use std::time::Duration;
use std::io::Write; // Import the Write trait for flush()

fn main() -> Result<(), std::io::Error> {
    println!("Press Esc to exit..."); // This initial println is fine before raw mode
    terminal::enable_raw_mode()?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            // Use print! with \r\n and flush
                            print!("Char: {}\r\n", c);
                            std::io::stdout().flush()?; // Ensure it's written immediately
                        }
                        KeyCode::Esc => {
                            // Also ensure exit message is flushed and uses \r\n
                            print!("Exiting...\r\n");
                            std::io::stdout().flush()?;
                            break;
                        }
                        KeyCode::Enter => {
                            print!("Enter pressed\r\n");
                            std::io::stdout().flush()?;
                        }
                        _ => {
                            // Ignored for now
                        }
                    }
                }
                Event::Resize(width, height) => {
                    // Handle resize if needed, ensure flushing
                    print!("Resized to {}x{}\r\n", width, height);
                    std::io::stdout().flush()?;
                }
                _ => {} // Ignore other event types
            }
        } else {
            // Timeout elapsed
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
