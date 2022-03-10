use std::sync::Arc;

use crate::{voice::RVoice, params::RParams};

fn ms_to_samples(time: f32, sample_rate: f32) -> u32 {
    (sample_rate * time / 1000.0) as u32
}

#[derive(Default)]
pub struct RProcessor {
    sample_rate: f32,
    voices: Vec<RVoice>,
    params: Arc<RParams>,
    volume: f32,
    pitch_bend_multiplier: f32,
}

impl RProcessor {
    pub fn new(params: Arc<RParams>) -> RProcessor {

        // get parameters
        let attack = params.attack();
        let decay = params.decay();
        let sustain = params.sustain();
        let release = params.release();

        let mut voices = Vec::new();
        
        for _ in 0..128 {
            // sample rate is set to 44100.0 by default
            let mut new_voice = RVoice::new(44100.0);

            new_voice.envelope.attack = ms_to_samples(attack, 44100.0);
            new_voice.envelope.decay = ms_to_samples(decay, 44100.0);
            new_voice.envelope.sustain = sustain;
            new_voice.envelope.release = ms_to_samples(release, 44100.0);

            voices.push(RVoice::new(44100.0));
        }
        
        let mut processor = RProcessor {
            sample_rate: 44100.0,
            voices,
            params,
            volume: 0.125,
            pitch_bend_multiplier: 1.0,
        };
        
        processor.update_sampling_rate(44100.0);
        
        return processor;
    }
    
    pub fn update_sampling_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
            
        for voice in &mut self.voices {
            voice.update_sample_rate(sample_rate);
        }
    }

    fn find_available_voice(&mut self) -> Option<&mut RVoice> {
        for voice in &mut self.voices {
            if !voice.is_on {
                return Some(voice);
            }
        }
        return None;
    }
    
    pub fn note_on(&mut self, note_number: u8, velocity: u8) {
        match self.find_available_voice() {
            Some(voice) => voice.play(note_number, velocity),
            None => (),
        }
    }
    
    pub fn note_off(&mut self, note_number: u8) {
        for voice in &mut self.voices {
            if !voice.envelope.is_releasing && voice.note == note_number {
                voice.release_envelope();
            }
        }
    }

    pub fn update_pitch_bend_multiplier(&mut self, pitch_bend: u16) {
        // center pitch_bend around 0 (easier to read)
        let bend = (pitch_bend as f32) - 8192.0;

        // how many semitones we're bending (a fraction of the pitch_bend_limit)
        let semitones;

        // bending up (1 <= bend <= 8191)
        if bend > 0.0 {
            semitones = (4.0) * (bend / 8191.0);

        // no bend
        } else if bend == 0.0 {
            semitones = 0.0;

        // bending down (-8192 <= bend <= -1)
        } else {
            semitones = (4.0) * (bend / 8192.0);
        }

        // what we multiply by our base frequency to bend it however many semitones we want
        self.pitch_bend_multiplier = 2.0_f32.powf(semitones / 12.0);
    }
    
    pub fn process(&mut self) -> f32 {

        // get parameters
        let wave = self.params.wave();
        let attack = self.params.attack();
        let decay = self.params.decay();
        let sustain = self.params.sustain();
        let release = self.params.release();
        
        let mut val = 0.0;
        
        for voice in &mut self.voices {
            // set adsr (only if envelope is finished)
            if voice.envelope.is_done {
                voice.envelope.attack = ms_to_samples(attack, self.sample_rate);
                voice.envelope.decay = ms_to_samples(decay, self.sample_rate);
                voice.envelope.sustain = sustain;
                voice.envelope.release = ms_to_samples(release, self.sample_rate);
            }

            // apply frequency modulation
            voice.multiply_frequency(self.pitch_bend_multiplier);

            // process
            val += self.volume * voice.process(wave);
        }

        return val;
    }
    
    pub fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.reset();
        }
    }
}