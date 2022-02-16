use sdl2::rect::Rect;

use super::widgets::{button::Button, text::Text, Widget};


pub struct UI<'a> {
    button: Button<'a, Text>
}

impl<'a> UI<'a> {
    pub fn new() -> UI<'a> {
        return UI {
            button: Button::new(Rect::new(32, 32, 64, 32), Text::new(), &|| {})
        }
    }
}

impl<'a> Widget for UI<'a> {
    fn render(&mut self, context: &mut super::Context) {
        self.button.render(context);
    }

    fn input(&mut self) {
        
    }
}