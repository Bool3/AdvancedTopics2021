
use std::sync::Mutex;

use vst::plugin::{HostCallback, PluginParameters};
use vst::host::Host;

use crate::{wave::Wave, route::Route, filter_type::FilterType};

const ATTACK_MIN: f32 = 10.0;
const ATTACK_MAX: f32 = 4000.0;

const DECAY_MIN: f32 = 10.0;
const DECAY_MAX: f32 = 4000.0;

const RELEASE_MIN: f32 = 10.0;
const RELEASE_MAX: f32 = 4000.0;

const PITCH_BEND_LIMIT_MIN: u8 = 1;
const PITCH_BEND_LIMIT_MAX: u8 = 84;

const LFO_FREQUENCY_MAX: f32 = 500.0;

const CUTOFF_FREQUENCY_MAX: f32 = 22050.0;

// play with this number to get a good feel for the knob
// higher number will make it more curved
// should be a whole number
const CUTOFF_EXPONENT_BASE: f32 = 10.0;

const OSC_DETUNE_SEMITONES_MAX: i32 = 36;   // 3 octaves
const OSC_DETUNE_CENTS_MAX: i32 = 100;      // 1 semitone
// remember to fix going over the frequency limit in the voice file

pub struct RParams {
    pub host: HostCallback,         // included for logging purposes
    
    wave_1: Mutex<Wave>,              // 0
    
    attack: Mutex<f32>,             // 1
    decay: Mutex<f32>,              // 2
    sustain: Mutex<f32>,            // 3
    release: Mutex<f32>,            // 4

    pitch_bend_limit: Mutex<u8>,    // 5

    log: Mutex<f32>,                // 6

    lfo_frequency: Mutex<f32>,      // 7
    lfo_wave: Mutex<Wave>,          // 8
    lfo_intensity: Mutex<f32>,      // 9
    lfo_route: Mutex<Route>,        // 10

    cutoff_frequency: Mutex<f32>,   // 11 -- internally linear [0, 1]
                                    //    -- exported exponentially [0 Hz, 25505 Hz]
    q_factor: Mutex<f32>,           // 12
    filter_mode: Mutex<FilterType>, // 13

    wave_2: Mutex<Wave>,            // 14

    osc_1_volume: Mutex<f32>,       // 15
    osc_2_volume: Mutex<f32>,       // 16

    osc_1_detune_semitones: Mutex<i32>,     // 17
    osc_1_detune_cents: Mutex<i32>,         // 18
    
    osc_2_detune_semitones: Mutex<i32>,     // 19
    osc_2_detune_cents: Mutex<i32>,         // 20
}

impl RParams {
    pub fn wave_1(&self) -> Wave {
        let wave_1_lock = self.wave_1.lock().unwrap();
        let wave_1 = wave_1_lock.clone();
        drop(wave_1_lock);

        return wave_1;
    }

    pub fn wave_2(&self) -> Wave {
        let wave_2_lock = self.wave_2.lock().unwrap();
        let wave_2 = wave_2_lock.clone();
        drop(wave_2_lock);

        return wave_2;
    }

    pub fn attack(&self) -> f32 {
        let attack_lock = self.attack.lock().unwrap();
        let attack = attack_lock.clone();
        drop(attack_lock);

        return attack;
    }

    pub fn decay(&self) -> f32 {
        let decay_lock = self.decay.lock().unwrap();
        let decay = decay_lock.clone();
        drop(decay_lock);

        return decay;
    }

    pub fn sustain(&self) -> f32 {
        let sustain_lock = self.sustain.lock().unwrap();
        let sustain = sustain_lock.clone();
        drop(sustain_lock);

        return sustain;
    }

    pub fn release(&self) -> f32 {
        let release_lock = self.release.lock().unwrap();
        let release = release_lock.clone();
        drop(release_lock);

        return release;
    }

    pub fn pitch_bend_limit(&self) -> u8 {
        let pb_limit_lock = self.pitch_bend_limit.lock().unwrap();
        let pb_limit = pb_limit_lock.clone();
        drop(pb_limit_lock);

        return pb_limit;
    }

    pub fn log(&self, number: f32) {
        let mut log_lock = self.log.lock().unwrap();

        *log_lock = number;

        drop(log_lock);
        
        // ABSOLUTELY NECESSARY
        self.host.update_display();
    }

    pub fn lfo_frequency(&self) -> f32 {
        let lfo_frequency_lock = self.lfo_frequency.lock().unwrap();
        let lfo_frequency = lfo_frequency_lock.clone();
        drop(lfo_frequency_lock);

        return lfo_frequency;
    }

    pub fn lfo_wave(&self) -> Wave {
        let lfo_wave_lock = self.lfo_wave.lock().unwrap();
        let lfo_wave = lfo_wave_lock.clone();
        drop(lfo_wave_lock);

        return lfo_wave;
    }

    pub fn lfo_intensity(&self) -> f32 {
        let lfo_intensity_lock = self.lfo_intensity.lock().unwrap();
        let lfo_intensity = lfo_intensity_lock.clone();
        drop(lfo_intensity_lock);

        return lfo_intensity;
    }

    pub fn lfo_route(&self) -> Route {
        let lfo_route_lock = self.lfo_route.lock().unwrap();
        let lfo_route = lfo_route_lock.clone();
        drop(lfo_route_lock);

        return lfo_route;
    }

    pub fn cutoff_frequency(&self) -> f32 {
        let cutoff_frequency_lock = self.cutoff_frequency.lock().unwrap();
        let cutoff_frequency = cutoff_frequency_lock.clone();
        drop(cutoff_frequency_lock);

        let cutoff_exponential = CUTOFF_FREQUENCY_MAX * (f32::powf(CUTOFF_EXPONENT_BASE, cutoff_frequency.clone()) - 1.0) / (CUTOFF_EXPONENT_BASE - 1.0);

        return cutoff_exponential;
    }

    pub fn q_factor(&self) -> f32 {
        let q_factor_lock = self.q_factor.lock().unwrap();
        let q_factor = q_factor_lock.clone();
        drop(q_factor_lock);

        return 1.0 - q_factor; // so that the knob is "flipped"
    }

    pub fn filter_mode(&self) -> FilterType {
        let filter_mode_lock = self.filter_mode.lock().unwrap();
        let filter_mode = filter_mode_lock.clone();
        drop(filter_mode_lock);

        return filter_mode;
    }

    pub fn osc_1_volume(&self) -> f32 {
        let osc_1_volume_lock = self.osc_1_volume.lock().unwrap();
        let osc_1_volume = osc_1_volume_lock.clone();
        drop(osc_1_volume_lock);

        return osc_1_volume;
    }

    pub fn osc_2_volume(&self) -> f32 {
        let osc_2_volume_lock = self.osc_2_volume.lock().unwrap();
        let osc_2_volume = osc_2_volume_lock.clone();
        drop(osc_2_volume_lock);

        return osc_2_volume;
    }

    pub fn osc_1_detune_semitones(&self) -> i32 {
        let osc_1_detune_semitones_lock = self.osc_1_detune_semitones.lock().unwrap();
        let osc_1_detune_semitones = osc_1_detune_semitones_lock.clone();
        drop(osc_1_detune_semitones_lock);

        return osc_1_detune_semitones;
    }

    pub fn osc_1_detune_cents(&self) -> i32 {
        let osc_1_detune_cents_lock = self.osc_1_detune_cents.lock().unwrap();
        let osc_1_detune_cents = osc_1_detune_cents_lock.clone();
        drop(osc_1_detune_cents_lock);

        return osc_1_detune_cents;
    }

    pub fn osc_2_detune_semitones(&self) -> i32 {
        let osc_2_detune_semitones_lock = self.osc_2_detune_semitones.lock().unwrap();
        let osc_2_detune_semitones = osc_2_detune_semitones_lock.clone();
        drop(osc_2_detune_semitones_lock);

        return osc_2_detune_semitones;
    }

    pub fn osc_2_detune_cents(&self) -> i32 {
        let osc_2_detune_cents_lock = self.osc_2_detune_cents.lock().unwrap();
        let osc_2_detune_cents = osc_2_detune_cents_lock.clone();
        drop(osc_2_detune_cents_lock);

        return osc_2_detune_cents;
    }
}


impl Default for RParams {
    fn default() -> RParams {
        return RParams {
            host: HostCallback::default(),

            wave_1: Mutex::new(Wave::Sine),

            attack: Mutex::new(ATTACK_MIN),
            decay: Mutex::new(DECAY_MIN),
            sustain: Mutex::new(0.5),
            release: Mutex::new(RELEASE_MIN),

            pitch_bend_limit: Mutex::new(PITCH_BEND_LIMIT_MIN),

            log: Mutex::new(3.0),

            lfo_frequency: Mutex::new(0.0),
            lfo_wave: Mutex::new(Wave::Sine),
            lfo_intensity: Mutex::new(0.0),
            lfo_route: Mutex::new(Route::None),

            cutoff_frequency: Mutex::new(1.0),
            q_factor: Mutex::new(0.0),
            filter_mode: Mutex::new(FilterType::LowPass),

            wave_2: Mutex::new(Wave::Sine),

            osc_1_volume: Mutex::new(1.0),
            osc_2_volume: Mutex::new(0.0),

            osc_1_detune_semitones: Mutex::new(0),
            osc_1_detune_cents: Mutex::new(0),

            osc_2_detune_semitones: Mutex::new(0),
            osc_2_detune_cents: Mutex::new(0),
        }
    }
}

impl PluginParameters for RParams {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 | 8 | 9 | 10 | 12 | 13 | 14 | 15 | 16 => String::from(""),
            1 | 2 | 4 => String::from("ms"),
            3 => String::from("%velocity"),
            5 | 17 | 19 => String::from("semitones"),
            7 | 11 => String::from("Hz"),
            18 | 20 => String::from("cents"),
            _ => String::from("UNKNOWN"),
        }
    }
    
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => {
                let wave_1 = self.wave_1.lock().unwrap();
                return wave_1.to_string();
            },
            1 => {
                let attack = self.attack.lock().unwrap();
                return attack.to_string();
            },
            2 => {
                let decay = self.decay.lock().unwrap();
                return decay.to_string();
            },
            3 => {
                let sustain = self.sustain.lock().unwrap();
                return ((sustain.clone() * 100.0) as u32).to_string();
            },
            4 => {
                let release = self.release.lock().unwrap();
                return release.to_string();
            },
            5 => {
                let pb_limit = self.pitch_bend_limit.lock().unwrap();
                return pb_limit.to_string();
            },
            6 => {
                let log = self.log.lock().unwrap();
                return log.to_string();
            },
            7 => {
                let lfo_frequency = self.lfo_frequency.lock().unwrap();
                return lfo_frequency.to_string();
            },
            8 => {
                let lfo_wave = self.lfo_wave.lock().unwrap();
                return lfo_wave.to_string();
            },
            9 => {
                let lfo_intensity = self.lfo_intensity.lock().unwrap();
                return lfo_intensity.to_string();
            },
            10 => {
                let lfo_route = self.lfo_route.lock().unwrap();
                return lfo_route.to_string();
            }
            11 => {
                let cutoff_frequency = self.cutoff_frequency.lock().unwrap();

                let cutoff_exponential = CUTOFF_FREQUENCY_MAX * (f32::powf(CUTOFF_EXPONENT_BASE, cutoff_frequency.clone()) - 1.0) / (CUTOFF_EXPONENT_BASE - 1.0);

                return cutoff_exponential.to_string();
            },
            12 => {
                let q_factor = self.q_factor.lock().unwrap();
                return q_factor.to_string();
            },
            13 => {
                let filter_mode = self.filter_mode.lock().unwrap();
                return filter_mode.to_string();
            },
            14 => {
                let wave_2 = self.wave_2.lock().unwrap();
                return wave_2.to_string();
            },
            15 => {
                let osc_1_volume = self.osc_1_volume.lock().unwrap();
                return osc_1_volume.to_string();
            },
            16 => {
                let osc_2_volume = self.osc_2_volume.lock().unwrap();
                return osc_2_volume.to_string();
            },
            17 => {
                let osc_1_detune_semitones = self.osc_1_detune_semitones.lock().unwrap();
                return osc_1_detune_semitones.to_string();
            },
            18 => {
                let osc_1_detune_cents = self.osc_1_detune_cents.lock().unwrap();
                return osc_1_detune_cents.to_string();
            },
            19 => {
                let osc_2_detune_semitones = self.osc_2_detune_semitones.lock().unwrap();
                return osc_2_detune_semitones.to_string();
            },
            20 => {
                let osc_2_detune_cents = self.osc_2_detune_cents.lock().unwrap();
                return osc_2_detune_cents.to_string();
            },
            _ => return String::from("UNKNOWN")
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => String::from("Wave"),
            1 => String::from("Attack"),
            2 => String::from("Decay"),
            3 => String::from("Sustain"),
            4 => String::from("Release"),
            5 => String::from("Pitch Bend Limit"),
            6 => String::from("Log"),
            7 => String::from("LFO Frequency"),
            8 => String::from("LFO Wave"),
            9 => String::from("LFO Intensity"),
            10 => String::from("LFO Route"),
            11 => String::from("Filter Cutoff"),
            12 => String::from("Resonance"),
            13 => String::from("Filter Mode"),
            14 => String::from("Wave"),
            15 => String::from("Osc 1 Vol"),
            16 => String::from("Osc 2 Vol"),
            17 | 18 => String::from("Osc 1 Detune"),
            19 | 20 => String::from("Osc 2 Detune"),
            _ => String::from("UNKNOWN"),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => {
                let wave_1 = self.wave_1.lock().unwrap();
                return wave_1.to_f32();
            },
            1 => {
                let attack = self.attack.lock().unwrap();
                return attack.clone() / ATTACK_MAX;
            },
            2 => {
                let decay = self.decay.lock().unwrap();
                return decay.clone() / DECAY_MAX;
            },
            3 => {
                let sustain = self.sustain.lock().unwrap();
                return sustain.clone();
            },
            4 => {
                let release = self.release.lock().unwrap();
                return release.clone() / RELEASE_MAX;
            },
            5 => {
                let pb_limit = self.pitch_bend_limit.lock().unwrap();
                return (pb_limit.clone() as f32) / (PITCH_BEND_LIMIT_MAX as f32);
            },
            7 => {
                let lfo_frequency = self.lfo_frequency.lock().unwrap();
                return lfo_frequency.clone() / LFO_FREQUENCY_MAX;
            },
            8 => {
                let lfo_wave = self.lfo_wave.lock().unwrap();
                return lfo_wave.to_f32();
            },
            9 => {
                let lfo_intensity = self.lfo_intensity.lock().unwrap();
                return lfo_intensity.clone();
            },
            10 => {
                let lfo_route = self.lfo_route.lock().unwrap();
                return lfo_route.to_f32();
            },
            11 => {
                let cutoff_frequency = self.cutoff_frequency.lock().unwrap();
                return cutoff_frequency.clone();
            },
            12 => {
                let q_factor = self.q_factor.lock().unwrap();
                return q_factor.clone();
            },
            13 => {
                let filter_mode = self.filter_mode.lock().unwrap();
                return filter_mode.to_f32();
            },
            14 => {
                let wave_2 = self.wave_2.lock().unwrap();
                return wave_2.to_f32();
            },
            15 => {
                let osc_1_volume = self.osc_1_volume.lock().unwrap();
                return osc_1_volume.clone();
            },
            16 => {
                let osc_2_volume = self.osc_2_volume.lock().unwrap();
                return osc_2_volume.clone();
            },
            17 => {
                let osc_1_detune_semitones = self.osc_1_detune_semitones.lock().unwrap();
                return (osc_1_detune_semitones.clone() + OSC_DETUNE_SEMITONES_MAX) as f32 / ((2 * OSC_DETUNE_SEMITONES_MAX) as f32);
            },
            18 => {
                let osc_1_detune_cents = self.osc_1_detune_cents.lock().unwrap();
                return (osc_1_detune_cents.clone() + OSC_DETUNE_CENTS_MAX) as f32 / ((2 * OSC_DETUNE_CENTS_MAX) as f32);
            },
            19 => {
                let osc_2_detune_semitones = self.osc_2_detune_semitones.lock().unwrap();
                return (osc_2_detune_semitones.clone() + OSC_DETUNE_SEMITONES_MAX) as f32 / ((2 * OSC_DETUNE_SEMITONES_MAX) as f32);
            },
            20 => {
                let osc_2_detune_cents = self.osc_2_detune_cents.lock().unwrap();
                return (osc_2_detune_cents.clone() + OSC_DETUNE_CENTS_MAX) as f32 / ((2 * OSC_DETUNE_CENTS_MAX) as f32);
            },
            _ => return 0.0
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => {
                let mut wave_1 = self.wave_1.lock().unwrap();
                *wave_1 = Wave::from_f32(value);
            },
            1 => {
                let mut attack = self.attack.lock().unwrap();

                // enforce lower limit
                if value * ATTACK_MAX >= ATTACK_MIN {
                    *attack = value * ATTACK_MAX;
                } else {
                    *attack = ATTACK_MIN;
                }
            },
            2 => {
                let mut decay = self.decay.lock().unwrap();

                // enforce lower limit
                if value * DECAY_MAX >= DECAY_MIN {
                    *decay = value * DECAY_MAX;
                } else {
                    *decay = DECAY_MIN;
                }
            },
            3 => {
                let mut sustain = self.sustain.lock().unwrap();
                *sustain = value;
            },
            4 => {
                let mut release = self.release.lock().unwrap();

                // enforce lower limit
                if value * RELEASE_MAX >= RELEASE_MIN {
                    *release = value * RELEASE_MAX;
                } else {
                    *release = RELEASE_MIN;
                }
            },
            5 => {
                let mut pb_limit = self.pitch_bend_limit.lock().unwrap();

                // enforce lower limit
                if value * PITCH_BEND_LIMIT_MAX as f32 >= PITCH_BEND_LIMIT_MIN as f32 {
                    *pb_limit = (value * PITCH_BEND_LIMIT_MAX as f32) as u8;
                } else {
                    *pb_limit = PITCH_BEND_LIMIT_MIN;
                }
            },
            7 => {
                let mut lfo_frequency = self.lfo_frequency.lock().unwrap();
                *lfo_frequency = value * LFO_FREQUENCY_MAX;
            },
            8 => {
                let mut lfo_wave = self.lfo_wave.lock().unwrap();
                *lfo_wave = Wave::from_f32(value);
            },
            9 => {
                let mut lfo_intensity = self.lfo_intensity.lock().unwrap();
                *lfo_intensity = value;
            },
            10 => {
                let mut lfo_route = self.lfo_route.lock().unwrap();
                *lfo_route = Route::from_f32(value);
            },
            11 => {
                let mut cutoff_frequency = self.cutoff_frequency.lock().unwrap();
                *cutoff_frequency = value;
            },
            12 => {
                let mut q_factor = self.q_factor.lock().unwrap();
                *q_factor = value;
            },
            13 => {
                let mut filter_mode = self.filter_mode.lock().unwrap();
                *filter_mode = FilterType::from_f32(value);
            },
            14 => {
                let mut wave_2 = self.wave_2.lock().unwrap();
                *wave_2 = Wave::from_f32(value);
            },
            15 => {
                let mut osc_1_volume = self.osc_1_volume.lock().unwrap();
                *osc_1_volume = value;
            },
            16 => {
                let mut osc_2_volume = self.osc_2_volume.lock().unwrap();
                *osc_2_volume = value;
            },
            17 => {
                let mut osc_1_detune_semitones = self.osc_1_detune_semitones.lock().unwrap();
                *osc_1_detune_semitones = (value * 2.0 * OSC_DETUNE_SEMITONES_MAX as f32) as i32 - OSC_DETUNE_SEMITONES_MAX;
            },
            18 => {
                let mut osc_1_detune_cents = self.osc_1_detune_cents.lock().unwrap();
                *osc_1_detune_cents = (value * 2.0 * OSC_DETUNE_CENTS_MAX as f32) as i32 - OSC_DETUNE_CENTS_MAX;
            },
            19 => {
                let mut osc_2_detune_semitones = self.osc_2_detune_semitones.lock().unwrap();
                *osc_2_detune_semitones = (value * 2.0 * OSC_DETUNE_SEMITONES_MAX as f32) as i32 - OSC_DETUNE_SEMITONES_MAX;
            },
            20 => {
                let mut osc_2_detune_cents = self.osc_2_detune_cents.lock().unwrap();
                *osc_2_detune_cents = (value * 2.0 * OSC_DETUNE_CENTS_MAX as f32) as i32 - OSC_DETUNE_CENTS_MAX;
            },
            _ => {}
        }
    }

    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        match index {
            0 => {
                let mut wave_1 = self.wave_1.lock().unwrap();
                
                *wave_1 = Wave::from_string(text);
                
                return true
            },
            8 => {
                let mut lfo_wave = self.lfo_wave.lock().unwrap();

                *lfo_wave = Wave::from_string(text);
                
                return true
            },
            10 => {
                let mut lfo_route = self.lfo_route.lock().unwrap();

                *lfo_route = Route::from_string(text);
                
                return true
            },
            13 => {
                let mut filter_mode = self.filter_mode.lock().unwrap();

                *filter_mode = FilterType::from_string(text);
                
                return true
            },
            14 => {
                let mut wave_1 = self.wave_1.lock().unwrap();
                
                *wave_1 = Wave::from_string(text);
                
                return true
            },
            _ => {
                return false
            }
        }
    }
}