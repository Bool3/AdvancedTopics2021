use std::{fs::File, sync::{Arc, Mutex}};

use log::{LevelFilter, info};
use params::RParams;
use rand::random;
use simplelog::{CombinedLogger, WriteLogger, Config};
use vst::{plugin::{Plugin, Info, HostCallback, CanDo, PluginParameters}, plugin_main, buffer::AudioBuffer, api::{Events, Supported}, event::Event, editor::Editor};

mod wave;
mod params;
mod processor;
mod voice;
mod adsr;
mod osc;

#[derive(Default)]
struct RSynth {
    host: HostCallback,
    params: Arc<params::RParams>,
    processors: Vec<processor::RProcessor>
}

impl Plugin for RSynth {
    
    fn new(host: HostCallback) -> RSynth {
        let params = Arc::new(params::RParams::default());
        
        let mut processors = Vec::new();
        
        for _i in 0..2 {
            processors.push(processor::RProcessor::new(params.clone()));
        }
        
        return RSynth {
            host,
            params,
            processors,
        }
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
            
            parameters: 2,
            
            
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
        for processor in &mut self.processors {
            processor.update_sampling_rate(rate);
        }
    }
    
    fn suspend(&mut self) {
        for processor in &mut self.processors {
            processor.reset();
        }
    }
    
    fn process_events(&mut self, events: &Events) {
        
        for ev in events.events() {
            
            match ev {
                Event::Midi(val) => {
                    match val.data[0] {
                        // note on
                        144 => {
                            for processor in &mut self.processors {
                                processor.note_on(val.data[1], val.data[2]);
                            }
                        },
                        // note off
                        128 => {
                            for processor in &mut self.processors {
                                processor.note_off(val.data[1]);
                            }
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
        
        let mut channel_number = 0;
        
        for channel in output.into_iter() {
            for sample in channel {
                *sample = self.processors[channel_number].process();
            }
            
            channel_number += 1;
        }
    }
}

plugin_main!(RSynth);