use crate::wave::Wave;

const PI: f32 = std::f32::consts::PI;
const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
const BLEP_MAX: f32 = 1.4142135; // maximum value achieved by the blep = blep(PI/4)

pub struct Osc {
    sample_rate: f32,

    frequency: f32,         // Hz
    phase: f32,             // range: [0, TWO_PI)
    phase_increment: f32,   // range: [0, PI]

    blep_splice_sample_length: f32,     // sample length of blep splice
    blep_splice_length: f32,            // phase_increment * blep_splice_sample_length

    last_output: f32,                   // only for triangle (because of leaky integrator)
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

impl Osc {
    pub fn new(frequency: f32, sample_rate: f32) -> Osc {
        let mut new_osc = Osc {
            sample_rate,

            frequency,
            phase: 0.0,
            phase_increment: 0.0,

            blep_splice_sample_length: 4.0,
            blep_splice_length: 0.0,

            last_output: 0.0,
        };
        
        new_osc.update_phase_increment();
        
        return new_osc;
    }
    
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    pub fn update_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;

        self.update_phase_increment();
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;

        self.update_phase_increment();
    }

    fn update_phase_increment(&mut self) {
        let cycles_per_sample = self.frequency / self.sample_rate;
        self.phase_increment = cycles_per_sample * TWO_PI;

        self.update_blep_splice_length();
    }

    pub fn update_blep_splice_sample_length(&mut self, sample_length: f32) {
        self.blep_splice_sample_length = sample_length;

        self.update_blep_splice_length();
    }

    fn update_blep_splice_length(&mut self) {
        // note: sample 0 is included, so the blep splice is really +1 samples long
        self.blep_splice_length = self.phase_increment * self.blep_splice_sample_length;
    }
    
    pub fn process(&mut self, wave: Wave) -> f32 {

        let val = match wave {
            Wave::Sine => {
                f32::sin(self.phase)
            },
            Wave::Triangle => {
                // get bandlimited square
                let square_val = self.process(Wave::Square);

                // move phase back because we call self.process()
                self.phase -= self.phase_increment;

                // leaky integrator: y[n] = A * x[n] + (1 - A) * y[n - 1]
                let tri_val = self.phase_increment * square_val + (1.0 - self.phase_increment) * self.last_output;
                
                self.last_output = tri_val;

                tri_val
            },
            Wave::Square => {
                // near upwards discontinuity (~0)
                if self.phase <= self.blep_splice_length {
                    // we sample the blep function at a fraction of its length (PI)
                    // that fraction being the phase / blep_splice_length : [0, 1]
                    // (remember: the blep_splice_length is some multiple of the phase_increment)
                    // divide by BLEP_MAX to keep values between -1 and 1
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
                    // shift phase to match discontinuity
                    let shifted_phase = self.phase - self.blep_splice_length / 2.0;

                    // downwards saw
                    1.0 - (shifted_phase / PI)
                }
            },
        };

        self.increment_phase();
        
        return self.normalize(val, wave);
    }

    fn increment_phase(&mut self) {
        self.phase += self.phase_increment;

        if self.phase > TWO_PI {
            self.phase -= TWO_PI;
        }
    }

    fn normalize(&self, val: f32, wave: Wave) -> f32 {
        match wave {
            Wave::Sine => {
                val
            }
            Wave::Triangle => {
                val
            }
            Wave::Square => {
                val / BLEP_MAX
            }
            Wave::Saw => {
                val / BLEP_MAX
            }
        }
    }
}