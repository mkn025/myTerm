use std::{f32, str};

use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::font::Font;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;

pub struct MyWindowHandler {
    input: String,
}

impl MyWindowHandler {
    pub fn new() -> Self {
        MyWindowHandler {
            input: String::new(),
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let bytes = include_bytes!("assets/fonts/default_light.ttf");
        let font = Font::new(bytes).unwrap();
        let text = format!("myTerm: {}", self.input);
        let block = font.layout_text(&text, 20.0, TextOptions::new());
        graphics.draw_text((5.0, 5.0), Color::WHITE, &block);

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }

    fn on_keyboard_char(&mut self, _helper: &mut WindowHelper, unicode_codepoint: char) {
        self.input.push(unicode_codepoint);
    }
}
