use crate::{wave::Wave, adsr::Adsr, osc::Osc};

fn note_to_frequency(note_number: u8) -> f32 {
    440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
}

pub struct RVoice {
    sample_rate: f32,

    is_on: bool,
    oscillator: Osc,
    pub envelope: Adsr,
}

impl RVoice {
    pub fn new(note_number: u8, sample_rate: f32) -> RVoice {
        RVoice {
            sample_rate,
            
            is_on: false,
            oscillator: Osc::new(note_to_frequency(note_number), sample_rate),
            envelope: Adsr::new(),
        }
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;

        self.oscillator.update_sample_rate(sample_rate);
    }

    pub fn reset(&mut self) {
        self.is_on = false;
        self.oscillator.reset();
        self.envelope.reset();
    }

    pub fn play(&mut self, velocity: u8) {
        if velocity != 0 {
            self.is_on = true;
            self.envelope.start(velocity);
        }
    }

    pub fn stop(&mut self) {
        self.is_on = false;
    }

    pub fn release_envelope(&mut self) {
        self.envelope.release();
    }
    
    pub fn process(&mut self, wave: Wave) -> f32 {
        let mut val;

        if self.is_on {

            val = self.oscillator.process(wave);
            val = self.envelope.process(val); 

            if self.envelope.is_done {
                self.stop();
            }

        } else {
            val = 0.0;
        }

        return val;
    }
}