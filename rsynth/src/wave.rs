
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
        if val < 0.25 {
            return Wave::Sine;

        } else if val < 0.5 {
            return Wave::Triangle;

        } else if val < 0.75 {
            return Wave::Square;

        } else {
            return Wave::Saw;
        }
    }
}
