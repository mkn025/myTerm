use speedy2d::Window;

mod window_handler;

fn main () {
    let window = Window::new_centered("myTerm", (800, 800)).unwrap();
    let x = 2_i32.pow(5_u32);

    window.run_loop(window_handler::MyWindowHandler::new());
    
    
}
