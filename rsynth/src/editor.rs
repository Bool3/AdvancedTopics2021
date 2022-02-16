use std::{ffi::c_void, sync::Arc, collections::HashMap};

use log::info;
use sdl2::{Sdl, VideoSubsystem, video::{Window, GLProfile}, pixels::Color, render::Canvas, EventPump, keyboard::Keycode, event::Event};
use vst::editor::Editor;

use crate::{params::RParams, wave::Wave, ui::{UI, Widget, Context, FontStore}};

struct WindowInfo {
    pub canvas: Canvas<Window>
}

pub struct REditor<'a> {
    is_open: bool,
    params: Arc<RParams>,
    sdl: Sdl,
    event_pump: EventPump,
    winfo: Option<WindowInfo>,
    i: i32,
    ui: UI<'a>
}

impl<'a> REditor<'a> {
    pub fn new(params: Arc<RParams>) -> REditor<'a> {
        let sdl = sdl2::init().unwrap();
        
        let event_pump = sdl.event_pump().unwrap();
        
        return REditor {
            is_open: false,
            params: params,
            sdl: sdl,
            event_pump: event_pump,
            winfo: None,
            i: 0,
            ui: UI::new()
        }
    }
}

impl<'a> Editor for REditor<'a> {
    fn size(&self) -> (i32, i32) {
        return (480, 360)
    }

    fn position(&self) -> (i32, i32) {
        return (0, 0)
    }
    
    fn open(&mut self, parent: *mut c_void) -> bool {
        let video_subsystem = self.sdl.video().unwrap();
        
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_double_buffer(true);
        gl_attr.set_multisample_samples(4);
        
        let window = unsafe { Window::from_ll(video_subsystem, sdl2::sys::SDL_CreateWindowFrom(parent)) };
        
        let mut canvas = window.into_canvas().build().unwrap();
        
        canvas.set_draw_color(Color::RGB(self.i as u8, self.i as u8, self.i as u8));
        canvas.clear();
        canvas.present();
        
        self.winfo = Some(WindowInfo {
            canvas: canvas
        });
        
        self.is_open = true;
        
        return true
    }
    
    fn close(&mut self) {
        self.is_open = false;
    }
    
    fn idle(&mut self) {
        match &mut self.winfo {
            Some(winfo) => {
                let canvas = &mut winfo.canvas;
                
                canvas.set_draw_color(Color::RGB(self.i as u8, self.i as u8, self.i as u8));
                canvas.clear();
                
                for ev in self.event_pump.poll_iter() {
                    match ev {
                        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                            let mut inner = self.params.inner.lock().unwrap();
                            
                            if self.i == 0 {
                                self.i = 80;
                                
                                inner.wave = Wave::Triangle;
                            } else if self.i == 80 {
                                self.i = 160;
                                
                                inner.wave = Wave::Square;
                            } else if self.i == 160 {
                                self.i = 240;
                                
                                inner.wave = Wave::Saw;
                            } else {
                                self.i = 0;
                                
                                inner.wave = Wave::Sine;
                            }
                        },
                        _ => {}
                    }
                }
                
                
                self.ui.render(&mut Context::new(&mut self.sdl, canvas));
                
                canvas.present();
            },
            _ => {}
        }
        
    }

    fn is_open(&mut self) -> bool {
        return self.is_open
    }
}
