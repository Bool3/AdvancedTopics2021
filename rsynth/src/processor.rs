use std::sync::Arc;

use log::info;

use crate::{voice::RVoice, wave::Wave, params::RParams};

#[derive(Default)]
pub struct RProcessor {
    voices: Vec<RVoice>,
    params: Arc<RParams>,
    volume: f32,
}

impl RProcessor {
    pub fn new(params: Arc<RParams>) -> RProcessor {
        let mut voices = Vec::new();
        
        for i in 0..128 {
            voices.push(RVoice::new(i));
        }
        
        let mut processor = RProcessor {
            voices: voices,
            params: params,
            volume: 0.125f32
        };
        
        processor.update_angle_delta(44100f32);
        
        return processor;
    }
    
    pub fn update_angle_delta(&mut self, sample_rate: f32) {
        for voice in &mut self.voices {
            voice.update_angle_delta(sample_rate);
        }
    }
    
    pub fn note_on(&mut self, note_number: u8) {
        self.voices[note_number as usize].enabled = true;
    }
    
    pub fn note_off(&mut self, note_number: u8) {
        self.voices[note_number as usize].enabled = false;
    }
    
    pub fn process(&mut self) -> f32 {
        let lock = self.params.inner.lock().unwrap();
        
        let wave = lock.wave;
        
        drop(lock);
        
        let mut val = 0f32;
        
        for voice in &mut self.voices {
            val += self.volume * voice.process(wave);
        }
        
        return val;
    }
    
    pub fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.pos = 0f32;
        }
    }
}
