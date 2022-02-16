
use sdl2::{rect::Rect, pixels::Color};

use crate::ui::Pos;

use super::Widget;

pub struct Button<'a, T> where T: Widget {
    rect: Rect,
    child: Box<T>,
    on_click: &'a dyn Fn()
}

impl<'a, T> Button<'a, T> where T: Widget {
    pub fn new(rect: Rect, child: T, on_click: &'a dyn Fn()) -> Button<'a, T> where T: Widget {
        return Button {
            rect: rect,
            child: Box::new(child),
            on_click: on_click
        }
    }
}

impl<'a, T> Widget for Button<'a, T> where T: Widget {
    fn render(&mut self, context: &mut crate::ui::Context) {
        context.canvas.set_draw_color(Color::RGB(255, 0, 0));
        
        let _ = context.canvas.fill_rect(self.rect);
    }
    
    fn input(&mut self) {
        
    }
}
