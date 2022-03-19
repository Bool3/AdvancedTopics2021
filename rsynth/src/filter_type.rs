#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilterType {
    HighPass,
    BandPass,
    LowPass,
}

impl FilterType {
    pub fn to_string(&self) -> String {
        match self {
            FilterType::HighPass => String::from("High Pass"),
            FilterType::BandPass => String::from("Band Pass"),
            FilterType::LowPass => String::from("Low Pass"),
        }
    }
    
    pub fn from_string(val: String) -> FilterType {
        match val.as_str() {
            "None" => FilterType::HighPass,
            "Frequency" => FilterType::BandPass,
            "Amplitude" => FilterType::LowPass,
            _ => FilterType::LowPass
        }
    }
    
    pub fn to_f32(&self) -> f32 {
        match self {
            FilterType::HighPass => 0.0,
            FilterType::BandPass => 0.25,
            FilterType::LowPass => 0.5,
        }
    }
    
    pub fn from_f32(val: f32) -> FilterType {
        if val < 0.25 {
            return FilterType::HighPass;

        } else if val < 0.5 {
            return FilterType::BandPass;
            
        } else {
            return FilterType::LowPass;
        }
    }
}