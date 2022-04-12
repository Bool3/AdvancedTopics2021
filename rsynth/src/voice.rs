use crate::{wave::Wave, adsr::Adsr, osc::Osc};

fn note_to_frequency(note_number: u8) -> f32 {
    440.0 * 2.0_f32.powf((note_number as f32 - 69.0) / 12.0)
}

pub struct RVoice {
    sample_rate: f32,
    pub note: u8,
    pub is_on: bool,

    pub envelope: Adsr,

    osc_1: Osc,
    osc_2: Osc,

    osc_1_detune: i32,      // in cents
    osc_2_detune: i32,      // in cents
}

impl RVoice {
    pub fn new(sample_rate: f32) -> RVoice {
        RVoice {
            sample_rate,
            note: 69,
            is_on: false,

            envelope: Adsr::new(),

            osc_1: Osc::new(note_to_frequency(69), sample_rate),
            osc_2: Osc::new(note_to_frequency(69), sample_rate),

            osc_1_detune: 0,
            osc_2_detune: 0,
        }
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;

        self.osc_1.update_sample_rate(sample_rate);
        self.osc_2.update_sample_rate(sample_rate);
    }

    pub fn reset(&mut self) {
        self.is_on = false;
        self.envelope.reset();

        self.osc_1.reset();
        self.osc_2.reset();
    }

    pub fn play(&mut self, note: u8, velocity: u8) {
        if velocity != 0 {
            self.note = note;
            self.is_on = true;
            self.envelope.start(velocity);

            self.osc_1.update_frequency(note_to_frequency(note));
            self.osc_1.reset();

            self.osc_2.update_frequency(note_to_frequency(note));
            self.osc_2.reset();
        }
    }

    pub fn stop(&mut self) {
        self.is_on = false;
    }

    pub fn release_envelope(&mut self) {
        self.envelope.release();
    }

    pub fn multiply_frequency(&mut self, multiplier: f32) {
        let new_frequency = note_to_frequency(self.note) * multiplier;

        self.osc_1.update_frequency(new_frequency);
        self.osc_2.update_frequency(new_frequency);
    }
    
    pub fn process(&mut self, wave_1: Wave, wave_2: Wave, osc_1_vol: f32, osc_2_vol: f32) -> f32 {
        let mut val = 0.0;

        if self.is_on {
            
            val += self.osc_1.process(wave_1) * osc_1_vol;
            val += self.osc_2.process(wave_2) * osc_2_vol;

            // maybe divide by 2 for two oscillators
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