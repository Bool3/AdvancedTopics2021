use log::info;

use crate::wave::Wave;


pub struct RVoice {
    frequency: f32,
    pub enabled: bool,
    pub pos: f32,
    delta_pos: f32,
}


fn frequency_by_note_number(note_number: u8) -> f32 {
    return 440f32 * 2f32.powf((note_number as f32 - 69f32) / 12f32);
}


impl RVoice {
    pub fn new(note_number: u8) -> RVoice {
        let mut processor = RVoice {
            frequency: frequency_by_note_number(note_number),
            enabled: false,
            pos: 0f32,
            delta_pos: 0f32,
        };
        
        processor.update_angle_delta(44100f32);
        
        return processor;
    }
    
    pub fn update_angle_delta(&mut self, sample_rate: f32) {
        let cycles_per_sample = self.frequency / sample_rate;
        
        self.delta_pos = cycles_per_sample * 2f32 * std::f32::consts::PI;
    }
    
    pub fn process(&mut self, wave: Wave) -> f32 {
        let val;
        
        if self.enabled {
            val = match wave {
                Wave::Sine => {
                    f32::sin(self.pos)
                },
                Wave::Triangle => {
                    ((-2f32 * f32::abs(self.pos - std::f32::consts::PI)) / std::f32::consts::PI) + 1f32
                },
                Wave::Square => {
                    if self.pos < std::f32::consts::PI {
                        1.0f32
                    } else {
                        -1.0f32
                    }
                },
                Wave::Saw => {
                    (self.pos / std::f32::consts::PI) - 1f32
                },
                _ => {
                    f32::sin(self.pos)
                }
            };
        } else {
            val = 0f32;
        };
        
        self.pos += self.delta_pos;
        
        if self.pos > 2f32 * std::f32::consts::PI {
            self.pos -= 2f32 * std::f32::consts::PI;
        }
        
        return val;
    }
}

