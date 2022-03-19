use std::sync::Arc;

use vst::{
    plugin::{Plugin, Info, HostCallback, CanDo, PluginParameters}, 
    plugin_main, 
    buffer::AudioBuffer, 
    api::{Events, Supported}, 
    event::Event, 
    editor::Editor, host::Host
};

mod wave;
mod params;
mod processor;
mod voice;
mod adsr;
mod osc;
mod route;
mod vcf;
mod filter_type;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct RSynth {
    host: HostCallback,
    params: Arc<params::RParams>,
    processor: processor::RProcessor,
}

impl Plugin for RSynth {
    
    fn new(host: HostCallback) -> RSynth {

        let mut p = params::RParams::default();
        p.host = host;

        let params = Arc::new(p);
        
        let processor = processor::RProcessor::new(params.clone());
        
        return RSynth { 
            host,
            params,
            processor
        };
    }
    
    fn get_info(&self) -> vst::plugin::Info {
        return Info {
            name: "RSynth".to_string(),
            unique_id: 42069,
            
            category: vst::plugin::Category::Synth,
            
            midi_inputs: 16,
            midi_outputs: 16,
            
            inputs: 2,
            outputs: 2,
            
            parameters: 14,
            
            
            ..Default::default()
        }
    }
    
    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => {
                return Supported::Yes
            },
            _ => {
                return Supported::Maybe
            }
        }
    }
    
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        return self.params.clone();
    }
    
    fn set_sample_rate(&mut self, rate: f32) {
        self.processor.update_sample_rate(rate);
    }
    
    fn suspend(&mut self) {
        self.processor.reset();
    }
    
    fn process_events(&mut self, events: &Events) {
        
        for ev in events.events() {
            
            match ev {
                Event::Midi(val) => {
                    match val.data[0] {
                        // note on
                        144 => {
                            self.processor.note_on(val.data[1], val.data[2]);
                        },

                        // note off
                        128 => {
                            self.processor.note_off(val.data[1]);
                        },

                        // pitch bend
                        224 => {
                            // we need to combine the data (two u8) into one u16
                            // note: in actuality, MIDI does not use the most siginifcant bit
                            //       so we are really combining two u7 into one u14
                            //       thus, we only bit shift by 7 instead of 8
                            let least_significant = val.data[1] as u16;
                            let most_significant = val.data[2] as u16;

                            let msig_shifted = most_significant << 7;

                            let pitch_bend = least_significant | msig_shifted;

                            self.processor.update_pitch_bend_multiplier(pitch_bend);
                        },
                        _ => {}
                    }
                },
                _ => {
                }
            }
        }
    }
    
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut output) = buffer.split();

        let channel_1 = output.get_mut(0);
        let channel_2 = output.get_mut(1);

        for i in 0..channel_1.len() {
            let val = self.processor.process();

            channel_1[i] = val;
            channel_2[i] = val;
        }
    }
}

plugin_main!(RSynth);