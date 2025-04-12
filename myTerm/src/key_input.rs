 use winit::event::{ElementState, KeyEvent}; // Assuming winit types
 use winit::keyboard::{Key, KeyLocation, PhysicalKey}; // Add necessary imports

pub fn get_text_from_key_event(event: &KeyEvent) -> Option<&str> {
     // You might still want to only consider pressed events for *new* text input
     if event.state == ElementState::Pressed {
         // Borrow the Option<SmolStr> and map it to Option<&str>
         event.text.as_deref() // as_deref() converts Option<SmolStr> to Option<&str>
                               // Equivalent to: event.text.as_ref().map(|s| s.as_str())
     } else {
         // Not a key press, so no new text input from this event
         None
     }
}

