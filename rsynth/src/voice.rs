//use std::intrinsics::fabsf32;

use log::info;

use crate::wave::Wave;

const PI: f32 = std::f32::consts::PI;
const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub struct RVoice {
    frequency: f32,         // Hz
    pub enabled: bool, 
    pub phase: f32,         // range: [0, TWO_PI)
    phase_increment: f32,   // range: [0, PI]
}


fn note_to_frequency(note_number: u8) -> f32 {
    return 440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0);
}


impl RVoice {
    pub fn new(note_number: u8) -> RVoice {
        let mut new_voice = RVoice {
            frequency: note_to_frequency(note_number),
            enabled: false,
            phase: 0.0,
            phase_increment: 0.0,
        };
        
        new_voice.update_phase_increment(44100.0);
        
        return new_voice;
    }
    
    pub fn update_phase_increment(&mut self, sample_rate: f32) {
        let cycles_per_sample = self.frequency / sample_rate;
        
        self.phase_increment = cycles_per_sample * TWO_PI;
    }
    
    pub fn process(&mut self, wave: Wave) -> f32 {
        let val;
        
        if self.enabled {
            val = match wave {
                Wave::Sine => {
                    f32::sin(self.phase)
                },
                Wave::Triangle => {
                    // naive triangle
                    1.0 + (-2.0 * f32::abs(self.phase - PI)) / PI
                },
                Wave::Square => {
                    // phase is within two samples of discontinuity at PI
                    if f32::abs(PI - self.phase) <= 2.0 * self.phase_increment {
                        self.band_limited_step(self.phase)
                    
                    // phase is within the two samples to the LEFT of dicontinuity at 0 (TWO_PI)
                    } else if self.phase >= TWO_PI - 2.0*self.phase_increment {
                        self.band_limited_step(self.phase - PI + TWO_PI)

                    // phase is within the two samples to the RIGHT of discontinuity at 0 (TWO_PI)
                    } else if self.phase <= 2.0*self.phase_increment {
                        self.band_limited_step(self.phase + PI + TWO_PI)

                    } else if self.phase < PI {
                        1.0

                    } else {
                        -1.0
                    }
                },
                Wave::Saw => {
                    // naive downwards saw
                    let naive_saw = 1.0 - (self.phase / PI);
                    naive_saw
                },
                _ => {
                    f32::sin(self.phase)
                }
            };
        } else {
            val = 0.0;
        };
        
        self.increment_phase();
        
        return val;
    }

    fn increment_phase(&mut self) {
        self.phase += self.phase_increment;

        if self.phase > TWO_PI {
            self.phase -= TWO_PI;
        }
    }

    fn band_limited_step(&self, phase: f32) -> f32 {
        let period = self.phase_increment * 4.0;

        // the coefficient of our input to the cosine function such that the cosine has our period
        let mut period_coefficient = TWO_PI / period;

        // stretches the period by 2 -- without this, the step only lasts from 0 to PI
        period_coefficient /= 2.0;

        // only the first harmonic and third harmonic of the step, multiplied by 3/2 to normalize
        3.0 / 2.0 * (f32::cos(phase * period_coefficient) - f32::cos(phase * period_coefficient * 3.0) / 3.0)
    }
}

