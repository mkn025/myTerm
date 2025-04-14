use nix::Result;
use std::os::unix::io::OwnedFd; 
use std::os::fd::AsRawFd;
use std::io::{self, Write};
use std::fs::File;
use std::io::Read;

/// Create a new pseudoterminal, a master and slave
/// Both master and slave are fileDescriptors
fn save_openpty() -> Result<(OwnedFd, OwnedFd)> {
    let pty_result = nix::pty::openpty(None, None)?;
    // Has two variables that are public, extract them
    let master_fd = pty_result.master;
    let slave_fd = pty_result.slave;

    return Ok((master_fd, slave_fd));
}

/// Write input to the master file, converts it to bytes before being sent 
/// 
/// Flushes to ensure being sent
/// Returns `io::Result<()>` to indicate success or failure.
fn write_to_master(input: &str, master_file: &mut File) -> io::Result<()> {
    // Convert the string slice to bytes
    let command = input.as_bytes();

    // Use write_all to ensure all bytes are written.
    // The ? operator will return early with the Err if write_all fails
    master_file.write_all(command)?;
    println!("Successfully wrote data to master PTY.");

    // Flush the stream to ensure data is sent.
    // The ? operator handles potential errors
    master_file.flush()?;
    println!("Successfully flushed master PTY.");

    // If both write_all and flush succeeded, return Ok
    return Ok(());
}

/// TEST
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to create PTY...");

    match save_openpty() {
        // Check for master and slave
        Ok((master_fd, slave_fd)) => {
            // We got a master and slave
            println!(
            "Successfully created PTY!"
        );
            // Prints master and slaves fd numbers, std = 0 input, std1 = output, std2 = error
            // When we create new FDs they can inherent these, however they do also point to dir,
            // files and so on, and also need theire own number so the computor can distinguish
            // them from each other.

            println!("  Master FD (raw number): {}", master_fd.as_raw_fd());
            println!("  Slave FD (raw number):  {}", slave_fd.as_raw_fd());

            // Now try to write to the master
            println!("Attempting to write to master FD...");

            // Convert OwnedFd to File which has write trait
            // File needs to be mutable 
            let mut master_file = File::from(master_fd);
            let input: &str = "ls -l\n";
            write_to_master(&input, &mut master_file)?;
            println!("Command sent successfully.");

            // Buffer for reading
            let mut buffer = [0u8; 4096];
            match master_file.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("Read {} bytes.", bytes_read);
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    } else {
                        println!("Read 0 bytes. EOF or nothing available.");
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from master PTY: {}", e);
                }
            }
            return Ok(());
        }

        Err(e) => {
            // Failure. The OS likely couldn't create the PTY.
            eprintln!("ERROR: Failed to create PTY: {}", e);
            // Exit with a non-zero status code to indicate failure
            std::process::exit(1);
        }
    }
}
