use std::collections::HashMap;

use sdl2::{Sdl, render::Canvas, video::Window};

mod widgets;
mod ui;

pub use ui::UI;
pub use widgets::Widget;

pub struct Pos {
    pub x: i32,
    pub y: i32
}


pub struct FontStore {
    //pub ttf: &'a mut Sdl2TtfContext,
    pub fonts: HashMap<String, String>
}

impl FontStore {
    pub fn new() -> FontStore {
        return FontStore {
            fonts: HashMap::new()
        }
    }
}

pub struct Context<'a> {
    pub sdl: &'a mut Sdl,
    pub font_store: FontStore,
    pub canvas: &'a mut Canvas<Window>
}

impl<'a> Context<'a> {
    pub fn new(sdl: &'a mut Sdl, canvas: &'a mut Canvas<Window>) -> Context<'a> {
        return Context {
            sdl: sdl,
            font_store: FontStore::new(),
            canvas: canvas
        }
    }
}
