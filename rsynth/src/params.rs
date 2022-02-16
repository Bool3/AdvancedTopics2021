
use std::sync::Mutex;

use vst::plugin::PluginParameters;

use crate::wave::Wave;

#[derive(Debug)]
pub struct RParams {
    pub inner: Mutex<RParamsInner>
}

impl Default for RParams {
    fn default() -> RParams {
        return RParams { inner: Mutex::new(RParamsInner::default()) }
    }
}

impl PluginParameters for RParams {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => return String::from(""),
            _ => return String::from("UNKNOWN")
        }
    }
    
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => {
                let inner = self.inner.lock().unwrap();
                
                return inner.wave.to_string();
            },
            _ => return String::from("UNKNOWN")
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => return String::from("Wave"),
            _ => return String::from("UNKNOWN")
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => {
                let inner = self.inner.lock().unwrap();
                
                return inner.wave.to_f32();
            },
            _ => return 0f32
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => {
                let mut inner = self.inner.lock().unwrap();
                
                inner.wave = Wave::from_f32(value);
            },
            _ => {}
        }
    }

    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        match index {
            0 => {
                let mut inner = self.inner.lock().unwrap();
                
                inner.wave = Wave::from_string(text);
                
                return true
            },
            _ => {
                return false
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct RParamsInner {
    pub wave: Wave
}
