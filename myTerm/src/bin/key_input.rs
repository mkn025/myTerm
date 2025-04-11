use crossterm::{terminal, event::{self, Event, KeyCode}};
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    println!("Press Esc to exit...");
    // Enable raw mode
    terminal::enable_raw_mode()?;

    loop {
        // Poll for an event for a short duration
        if event::poll(Duration::from_millis(100))? {
            // Read the event
            match event::read()? {
                Event::Key(key_event) => {
                    println!("Key: {:?}", key_event); // Prints detailed key info
                    if key_event.code == KeyCode::Esc {
                        break; // Exit on Escape
                    }
                    // You can check key_event.code for specific KeyCodes (like Char('a'), Enter, Up, etc.)
                    // and key_event.modifiers for Ctrl/Alt/Shift
                }
                Event::Resize(width, height) => {
                     println!("Resized to {}x{}", width, height);
                     // Handle resize if needed
                }
                _ => {
                     // Handle other events like mouse if enabled
                }
            }
        } else {
            // No event occurred within the timeout
            // You could do other work here if needed
        }
    }

    // Always disable raw mode before exiting
    terminal::disable_raw_mode()?;
    Ok(())
}
