
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
            Wave::Sine => String::from("Sine"),
            Wave::Triangle => String::from("Triangle"),
            Wave::Square => String::from("Square"),
            Wave::Saw => String::from("Saw"),
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
            Wave::Sine => 0.0,
            Wave::Triangle => 0.25,
            Wave::Square => 0.5,
            Wave::Saw => 0.75,
        }
    }
    
    pub fn from_f32(val: f32) -> Wave {
        match val {
            v if v == 0.0 => Wave::Sine,
            v if v == 0.25 => Wave::Triangle,
            v if v == 0.5 => Wave::Square,
            v if v == 0.75 => Wave::Saw,
            _ => Wave::Sine,
        }
    }
}
