#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    None,
    Frequency,
    Amplitude,
}

impl Route {
    pub fn to_string(&self) -> String {
        match self {
            Route::None => String::from("None"),
            Route::Frequency => String::from("Frequency"),
            Route::Amplitude => String::from("Amplitude"),
        }
    }
    
    pub fn from_string(val: String) -> Route {
        match val.as_str() {
            "None" => Route::None,
            "Frequency" => Route::Frequency,
            "Amplitude" => Route::Amplitude,
            _ => Route::None
        }
    }
    
    pub fn to_f32(&self) -> f32 {
        match self {
            Route::None => 0.0,
            Route::Frequency => 0.25,
            Route::Amplitude => 0.5,
        }
    }
    
    pub fn from_f32(val: f32) -> Route {
        match val {
            v if v == 0.0 => Route::None,
            v if v == 0.25 => Route::Frequency,
            v if v == 0.5 => Route::Amplitude,
            _ => Route::None,
        }
    }
}