use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::font::Font;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::window::VirtualKeyCode::*;
use std::error::Error;

pub struct MyWindowHandler {
    input: String,
    font: Font,
    new_line: String,
}

impl MyWindowHandler {
    pub fn new() -> Result<Self, Box<dyn Error>> { 
        let font_bytes: &'static [u8] = include_bytes!("assets/fonts/default_light.ttf"); 
        let font_def = Font::new(font_bytes)?;

        return Ok(MyWindowHandler {
            input: "~ ".to_string(),
            font: font_def,
            new_line: "\n~ ".to_string(),
        });
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        
        let text = format!("{}", self.input);
        let block = self.font.layout_text(&text, 45.0, TextOptions::new());
        graphics.draw_text((5.0, 5.0), Color::WHITE, &block);

        // Request that we draw another frame once this one has finished
        helper.request_redraw();

    }

    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper, unicode_codepoint: char) {
        self.input.push(unicode_codepoint);
    }

    fn on_key_down(
            &mut self,
            helper: &mut WindowHelper<()>,
            virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
            scancode: speedy2d::window::KeyScancode
        ) {
        match  virtual_key_code {  
            Some(Return) => self.input.push_str(&self.new_line.to_string()),
            _ => println!("{:?} not implemented", virtual_key_code),
        }
    }
}
