
use k_board::{keyboard::Keyboard, keys::Keys};
use std::process::Command; 
fn get_key_input() {
    println!("Listening for keys. Press Down to clear, Enter to exit...");
    // Keyboard::new() creates an iterator that yields key presses.
    for key in Keyboard::new() {
        match key {
            // Case for the Down arrow key
            Keys::Down => {
                println!("Down arrow pressed - clearing screen.");
                let status = Command::new("clear")
                                  .status(); // Get the exit status

                // Handle potential errors when running the command
                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("Failed to clear screen: command exited with status {}", exit_status);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute clear command: {}", e);
                        // Consider alternatives or just log if 'clear' isn't critical
                    }
                }
            }
            // Case for the Enter key
            Keys::Enter => {
                println!("Enter pressed - exiting loop.");
                // Break out of the for loop
                break;
            }
            // Default case for any other key
            _ => {
                println!("Detected key: {:?}", key);
            }
        }
    }
    println!("Exited key input loop.");
}

// Main function runs 
fn main() {
    get_key_input();
}
