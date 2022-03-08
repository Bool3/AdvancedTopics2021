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
}

impl RProcessor {
    pub fn new(params: Arc<RParams>) -> RProcessor {

        // unpack the parameters
        let attack_lock = params.attack.lock().unwrap();
        let attack = attack_lock.clone();
        drop(attack_lock);

        let decay_lock = params.decay.lock().unwrap();
        let decay = decay_lock.clone();
        drop(decay_lock);

        let sustain_lock = params.sustain.lock().unwrap();
        let sustain = sustain_lock.clone();
        drop(sustain_lock);

        let release_lock = params.release.lock().unwrap();
        let release = release_lock.clone();
        drop(release_lock);

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
            volume: 0.125
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

    pub fn pitch_bend(&mut self, pitch_bend: u16) {

        let bend = (pitch_bend as i16) - 8192;
        
        // if there is no pitch bend
        if bend == 0 {
            return
        
        } else {

            



            for voice in &mut self.voices {
                if voice.is_on {
                    
                }
            }
        }
    }
    
    pub fn process(&mut self) -> f32 {

        // unlock parameters
        let wave_lock = self.params.wave.lock().unwrap();
        let wave = wave_lock.clone();
        drop(wave_lock);

        let attack_lock = self.params.attack.lock().unwrap();
        let attack = attack_lock.clone();
        drop(attack_lock);
        
        let decay_lock = self.params.decay.lock().unwrap();
        let decay = decay_lock.clone();
        drop(decay_lock);

        let sustain_lock = self.params.sustain.lock().unwrap();
        let sustain = sustain_lock.clone();
        drop(sustain_lock);

        let release_lock = self.params.release.lock().unwrap();
        let release = release_lock.clone();
        drop(release_lock);
        
        let mut val = 0.0;
        
        for voice in &mut self.voices {
            // set adsr (only if envelope is finished)
            if voice.envelope.is_done {
                voice.envelope.attack = ms_to_samples(attack, self.sample_rate);
                voice.envelope.decay = ms_to_samples(decay, self.sample_rate);
                voice.envelope.sustain = sustain;
                voice.envelope.release = ms_to_samples(release, self.sample_rate);
            }

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