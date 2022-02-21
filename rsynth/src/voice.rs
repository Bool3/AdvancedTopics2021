//use std::intrinsics::fabsf32;

use log::info;

use crate::wave::Wave;

const PI: f32 = std::f32::consts::PI;
const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub struct RVoice {
    frequency: f32,             // Hz
    pub enabled: bool, 
    pub phase: f32,             // range: [0, TWO_PI)
    phase_increment: f32,       // range: [0, PI]
    blep_splice_length: f32,    // phase_increment * 4.0
    last_output: f32,           // only for triangle (because of leaky integrator)
}


fn note_to_frequency(note_number: u8) -> f32 {
    return 440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0);
}

fn blep_down(phase: f32) -> f32 {
    // only the first harmonic and third harmonic of the step, multiplied by 3/2 to normalize
    // domain: [0, PI]
    (f32::cos(phase) - f32::cos(phase * 3.0) / 3.0) * (3.0 / 2.0)
}

fn blep_up(phase: f32) -> f32 {
    // step_up is just the step_down flipped top to bottom
    blep_down(phase) * -1.0
}

impl RVoice {
    pub fn new(note_number: u8) -> RVoice {
        let mut new_voice = RVoice {
            frequency: note_to_frequency(note_number),
            enabled: false,
            phase: 0.0,
            phase_increment: 0.0,
            blep_splice_length: 0.0,
            last_output: 0.0,
        };
        
        new_voice.update_phase_increment(44100.0);
        new_voice.update_blep_splice_length(4.0);
        
        return new_voice;
    }
    
    pub fn update_phase_increment(&mut self, sample_rate: f32) {
        let cycles_per_sample = self.frequency / sample_rate;
        
        self.phase_increment = cycles_per_sample * TWO_PI;

    }

    pub fn update_blep_splice_length(&mut self, samples_long: f32) {
        // the band limited step will be "samples_long" samples long, but
        // will contain "samples_long + 1" samples (i.e. includes sample 0)
        self.blep_splice_length = self.phase_increment * samples_long;
    }
    
    pub fn process(&mut self, wave: Wave) -> f32 {
        let val;
        
        if self.enabled {
            val = match wave {
                Wave::Sine => {
                    f32::sin(self.phase)
                },
                Wave::Triangle => {
                    // get bandlimited square
                    let square_val = self.process(Wave::Square);

                    // move phase back because we call self.process()
                    self.phase -= self.phase_increment;

                    // leaky integrator: y[n] = A * x[n] + (1 - A) * y[n - 1]
                    let tri_val = 0.025 * square_val + (1.0 - 0.025) * self.last_output;
                    
                    self.last_output = tri_val;

                    // normalize
                    tri_val * 4.0
                },
                Wave::Square => {
                    // near upwards discontinuity (~0)
                    if self.phase <= self.blep_splice_length {
                        // we sample the blep function at a fraction of its length (PI)
                        // that fraction being the phase / blep_splice_length : [0, 1]
                        // (remember: the blep_splice_length is some multiple of the phase_increment)
                        blep_up(self.phase / self.blep_splice_length * PI)
                    
                    // between upwards discontinuity and downwards discontinuity
                    } else if self.phase < PI {
                        1.0
                    
                    // near downwards discontinuity (~PI)
                    } else if self.phase <= PI + self.blep_splice_length {
                        blep_down((self.phase - PI) / self.blep_splice_length * PI)

                    // between downwards discontinuity and upwards discontinuity
                    } else {
                        -1.0
                    }
                },
                Wave::Saw => {
                    // near upwards discontinuity (~0)
                    if self.phase <= self.blep_splice_length {
                        blep_up(self.phase / self.blep_splice_length * PI)

                    } else {
                        // downwards saw (shifted to match discontinuity)
                        1.0 - ((self.phase - self.blep_splice_length / 2.0) / PI)
                    }
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
}

