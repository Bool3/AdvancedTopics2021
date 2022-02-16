
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wave {
    Sine,
    Triangle,
    Square,
    Saw
}

impl Default for Wave {
    fn default() -> Wave {
        return Wave::Sine
    }
}

impl Wave {
    pub fn to_string(&self) -> String {
        match self {
            Wave::Sine => return String::from("Sine"),
            Wave::Triangle => return String::from("Triangle"),
            Wave::Square => return String::from("Square"),
            Wave::Saw => return String::from("Saw"),
        }
    }
    
    pub fn from_string(val: String) -> Wave {
        match val.as_str() {
            "Sine" => Wave::Sine,
            "Triangle" => Wave::Triangle,
            "Square" => Wave::Square,
            "Saw" => Wave::Saw,
            _ => Wave::Sine
        }
    }
    
    pub fn to_f32(&self) -> f32 {
        match self {
            Wave::Sine => return 0.0f32,
            Wave::Triangle => return 0.25f32,
            Wave::Square => return 0.5f32,
            Wave::Saw => return 0.75f32,
        }
    }
    
    pub fn from_f32(val: f32) -> Wave {
        match val {
            0.0f32 => return Wave::Sine,
            0.25f32 => return Wave::Triangle,
            0.5f32 => return Wave::Square,
            0.75f32 => return Wave::Saw,
            _ => return Wave::Sine,
        }
    }
}
