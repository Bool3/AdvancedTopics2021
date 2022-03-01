fn line(index: u32, start: f32, finish: f32, duration: u32) -> f32 {  
    (index as f32) * (finish - start) / (duration as f32) + start
}

pub struct Adsr {
    peak: f32,                  // [0, 1]
    pub sustain: f32,               // [0, 1]
    
    pub attack: u32,                // time (in samples)
    pub decay: u32,                 // time
    pub release: u32,               // time

    current_sample: u32,        // current sample of duration
    current_multiplier: f32,    // the current volume of the envelope

    pub is_done: bool,
}

impl Adsr {

    pub fn new_at_zero() -> Adsr {
        Adsr {

            peak: 0.0,
            sustain: 0.0,

            attack: 0,
            decay: 0,
            release: 0,

            current_sample: 0,
            current_multiplier: 0.0,

            is_done: false,
        }
    }

    pub fn new_test() -> Adsr {
        Adsr {

            peak: 0.0,
            sustain: 0.5,

            attack: 44100 * 2,
            decay: 44100 * 2,
            release: 44100 * 2,

            current_sample: 0,
            current_multiplier: 0.0,

            is_done: false,
        }
    }

    pub fn start(&mut self, velocity: u8) {
        self.peak = (velocity / 127) as f32;
        self.current_sample = 0;
        self.current_multiplier = 0.0;
        self.is_done = false;
    }

    pub fn release(&mut self) {
        self.peak = 0.0;
        self.current_sample = self.attack + self.decay + 1;
    }

    pub fn reset(&mut self) {
        self.peak = 0.0;
        self.current_sample = 0;
        self.current_multiplier = 0.0;
        self.is_done = false;
    }

    pub fn process(&mut self, val: f32) -> f32 {
        let mut val = val;

        // if the note is being played
        if self.peak != 0.0 {

            // ATTACK
            if self.current_sample <= self.attack {
                self.current_multiplier = line(self.current_sample,
                                               0.0,
                                               self.peak,
                                               self.attack
                );
                self.current_sample += 1;

            // DECAY
            } else if self.current_sample <= self.attack + self.decay {
                self.current_multiplier = line(self.current_sample - self.attack,
                                               self.peak,
                                               self.sustain * self.peak,
                                               self.decay
                );
                self.current_sample += 1;
    
            // SUSTAIN
            } else {
                self.current_multiplier = self.sustain * self.peak;
                // don't increment current_sample
            }

            val *= self.current_multiplier;

        // if the note is not being played
        } else {

            // RELEASE
            // note: current_multiplier stops being updated because we now want to decay from that value
            if self.current_sample <= self.attack + self.decay + self.release {
                val *= line(self.current_sample - (self.attack + self.decay),
                            self.current_multiplier,
                            0.0,
                            self.release
                );
                self.current_sample += 1;

            } else {
                val = 0.0;
                self.is_done = true;
            }
        }

        return val;
    }
}

// observe going from release to pressed
// do you want it to start from zero, or from the last release multiplier?