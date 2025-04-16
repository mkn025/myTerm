use speedy2d::Window;
use window_handler::MyWindowHandler;
mod window_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> { // Or appropriate error type
    let window = Window::new_centered("myTerm", (1400, 1100)).unwrap();

    match MyWindowHandler::new() {
        Ok(handler) => {
            // Only run the loop if the handler was created successfully
            window.run_loop(handler);
        }
        Err(e) => {
            // Handle the error from MyWindowHandler::new()
            println!("Failed to initialize window handler: {}", e);
            Err(Box::from(e)) // Propagate error (ensure conversion works)
        }
    }
}

