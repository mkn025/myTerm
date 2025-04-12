use speedy2d::Window;

mod window_handler;

fn main () {
    let window = Window::new_centered("myTerm", (640, 480)).unwrap();   
    window.run_loop(window_handler::MyWindowHandler::new());
}
