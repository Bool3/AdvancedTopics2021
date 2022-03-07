fn line(index: u32, start: f32, finish: f32, duration: u32) -> f32 {  
    (index as f32) * (finish - start) / (duration as f32) + start
}

pub struct Adsr {
    peak: f32,                  // peak volume of envelope [0, 1]
    pub sustain: f32,           // a fraction [0, 1] (the actual sustain volume = sustain * peak)
    
    pub attack: u32,            // time (in samples)
    pub decay: u32,             // ""
    pub release: u32,           // ""

    current_sample: u32,        // current sample of the envelope's duration
    volume: f32,                // the current (latest) volume of the envelope

    pub is_releasing: bool,
    pub is_done: bool,
}

impl Adsr {

    pub fn new() -> Adsr {
        Adsr {
            peak: 0.0,
            sustain: 0.5,

            attack: 44100,
            decay: 44100,
            release: 44100,

            current_sample: 0,
            volume: 0.0,

            is_releasing: false,
            is_done: true,
        }
    }

    pub fn start(&mut self, velocity: u8) {
        self.peak = (velocity as f32) / 127.0;

        self.current_sample = 0;
        self.volume = 0.0;

        self.is_releasing = false;
        self.is_done = false;
    }

    pub fn release(&mut self) {
        self.is_releasing = true;
        self.current_sample = self.attack + self.decay + 1;
    }

    pub fn reset(&mut self) {
        self.peak = 0.0;

        self.current_sample = 0;
        self.volume = 0.0;
        
        self.is_releasing = false;
        self.is_done = true;
    }

    pub fn process(&mut self, val: f32) -> f32 {
        let mut val = val;

        // you did a stupid and passed 0 in as the peak
        if self.peak == 0.0 {

            self.is_done = true;
            val = 0.0;

        // the peak is non-zero
        } else {
            
            if !self.is_releasing {

                // ATTACK
                if self.current_sample <= self.attack {
                    self.volume = line(self.current_sample,
                                                0.0,
                                                self.peak,
                                                self.attack
                    );
                    self.current_sample += 1;

                // DECAY
                } else if self.current_sample <= self.attack + self.decay {
                    self.volume = line(self.current_sample - self.attack,
                                                self.peak,
                                                self.sustain * self.peak,
                                                self.decay
                    );
                    self.current_sample += 1;
        
                // SUSTAIN
                } else {
                    self.volume = self.sustain * self.peak;
                    // don't increment current_sample
                }

                val *= self.volume;

            // RELEASE
            } else {
                // note: volume stops being updated
                //       because we now want to decay from that value
                if self.current_sample <= self.attack + self.decay + self.release {
                    val *= line(self.current_sample - (self.attack + self.decay),
                                self.volume,
                                0.0,
                                self.release
                    );
                    self.current_sample += 1;

                // DONE
                } else {
                    self.is_done = true;
                    val = 0.0;
                }
            }
        }

        return val;
    }
}

// observe going from release to pressed
// do you want it to start from zero, or from the last release multiplier?