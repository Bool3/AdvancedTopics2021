use std::sync::Arc;

use crate::{voice::RVoice, params::RParams, osc::Osc, route::Route, vcf::Vcf};

fn ms_to_samples(time: f32, sample_rate: f32) -> u32 {
    (sample_rate * time / 1000.0) as u32
}

pub struct RProcessor {
    sample_rate: f32,
    voices: Vec<RVoice>,
    lfo: Osc,
    vcf: Vcf,
    params: Arc<RParams>,
    volume: f32,
    pitch_bend_multiplier: f32,
    val_log_counter: u32, // for logging purposes
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
            lfo: Osc::new(0.0, 44100.0),
            vcf: Vcf::new(44100.0),
            params,
            volume: 0.125,
            pitch_bend_multiplier: 1.0,
            val_log_counter: 0,
        };
        
        processor.update_sample_rate(44100.0);
        
        return processor;
    }
    
    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        
        self.lfo.update_sample_rate(sample_rate);
        self.vcf.update_sample_rate(sample_rate);

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
            semitones = (self.params.pitch_bend_limit() as f32) * (bend / 8191.0);

        // no bend
        } else if bend == 0.0 {
            semitones = 0.0;

        // bending down (-8192 <= bend <= -1)
        } else {
            semitones = (self.params.pitch_bend_limit() as f32) * (bend / 8192.0);
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

        let lfo_frequency = self.params.lfo_frequency();
        let lfo_wave = self.params.lfo_wave();
        let lfo_intensity = self.params.lfo_intensity();
        let lfo_route = self.params.lfo_route();

        let cutoff_frequency = self.params.cutoff_frequency();
        let q_factor = self.params.q_factor();
        let filter_mode = self.params.filter_mode();

        // update and process LFO
        self.lfo.update_frequency(lfo_frequency);
        let lfo_val = self.lfo.process(lfo_wave);

        // declare and set the lfo's multipilers
        let lfo_frequency_multiplier: f32;  // used for frequency modulation
        let lfo_amplitude_multiplier: f32;  // used for amplitude modulation

        /*
        lfo_val : [-1, 1]
        intensity : [0, 1]

        FM:  f_new = f + f * lfo_val * intensity
        AM:  val_new = val * ( (lfo_val / 2 + 0.5) * intensity + (1 - intensity) )
        */

        match lfo_route {
            Route::None => {
                lfo_frequency_multiplier = 1.0;
                lfo_amplitude_multiplier = 1.0;
            },
            Route::Frequency => {
                lfo_frequency_multiplier = 1.0 + lfo_val * lfo_intensity;
                lfo_amplitude_multiplier = 1.0;
            },
            Route::Amplitude => {
                lfo_frequency_multiplier = 1.0;

                // transform the wave so that it oscillates between 0 and 1
                let lfo_val_transformed = lfo_val / 2.0 + 0.5;

                // It's not enough to just multiply the wave by the intensity like in FM,
                // as the intensity grows from 0 -> 1, the wave grows in that same fashion.
                // What we want instead is the wave to "grow" from 1 -> 0.
                // The desired result is to dip from a multiplier of 1, instead of rise from a multiplier of 0.
                // Thus, we need to add (1.0 - intensity) to the wave.
                lfo_amplitude_multiplier = (lfo_val_transformed * lfo_intensity) + (1.0 - lfo_intensity);
            }
        };

        let mut val = 0.0;

        // process voices
        for voice in &mut self.voices {

            // set adsr (only if envelope is finished)
            if voice.envelope.is_done {
                voice.envelope.attack = ms_to_samples(attack, self.sample_rate);
                voice.envelope.decay = ms_to_samples(decay, self.sample_rate);
                voice.envelope.sustain = sustain;
                voice.envelope.release = ms_to_samples(release, self.sample_rate);
            }

            // apply frequency modulation
            voice.multiply_frequency(self.pitch_bend_multiplier * lfo_frequency_multiplier);

            // process
            val += self.volume * voice.process(wave);
        }

        // apply amplitude modulation
        val *= lfo_amplitude_multiplier;

        // filter
        val = self.vcf.svf(val, cutoff_frequency, q_factor, filter_mode);

        return val;
    }
    
    pub fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.reset();
        }
    }
}

impl Default for RProcessor {
    fn default() -> RProcessor {
        RProcessor::new(Arc::new(RParams::default()))
    }
}