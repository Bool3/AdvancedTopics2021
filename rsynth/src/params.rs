
use std::sync::Mutex;

use vst::plugin::PluginParameters;

use crate::wave::Wave;

const ATTACK_MIN: f32 = 10.0;
const ATTACK_MAX: f32 = 4000.0;

const DECAY_MIN: f32 = 10.0;
const DECAY_MAX: f32 = 4000.0;

const RELEASE_MIN: f32 = 10.0;
const RELEASE_MAX: f32 = 4000.0;

const PITCH_BEND_LIMIT_MIN: u8 = 1;
const PITCH_BEND_LIMIT_MAX: u8 = 84;


#[derive(Debug)]
pub struct RParams {
    pub wave: Mutex<Wave>,
    pub attack: Mutex<f32>,
    pub decay: Mutex<f32>,
    pub sustain: Mutex<f32>,
    pub release: Mutex<f32>,
    pub pitch_bend_limit: Mutex<u8>,
}

impl RParams {
    pub fn wave(&self) -> Wave {
        let wave_lock = self.wave.lock().unwrap();
        let wave = wave_lock.clone();
        drop(wave_lock);

        return wave;
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
}


impl Default for RParams {
    fn default() -> RParams {
        return RParams {
            wave: Mutex::new(Wave::Sine),
            attack: Mutex::new(ATTACK_MIN),
            decay: Mutex::new(DECAY_MIN),
            sustain: Mutex::new(0.5),
            release: Mutex::new(RELEASE_MIN),
            pitch_bend_limit: Mutex::new(PITCH_BEND_LIMIT_MIN),
        }
    }
}

impl PluginParameters for RParams {
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => String::from(""),
            1 | 2 | 4 => String::from("ms"),
            3 => String::from("%velocity"),
            5 => String::from("semitones"),
            _ => String::from("UNKNOWN")
        }
    }
    
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => {
                let wave = self.wave.lock().unwrap();
                return wave.to_string();
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
            _ => String::from("UNKNOWN")
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => {
                let wave = self.wave.lock().unwrap();
                return wave.to_f32();
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
            _ => return 0.0
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => {
                let mut wave = self.wave.lock().unwrap();
                *wave = Wave::from_f32(value);
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
            _ => {}
        }
    }

    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        match index {
            0 => {
                let mut wave = self.wave.lock().unwrap();
                
                *wave = Wave::from_string(text);
                
                return true
            },
            _ => {
                return false
            }
        }
    }
}