use std::sync::Arc;

use crate::params;

fn ms_to_samples(time: f32, sample_rate: f32) -> u32 {
    (sample_rate * time / 1000.0) as u32
}

pub fn pitch_bend(pitch_bend: u16) -> f32 {
    // center pitch_bend around 0 (easier to read)
    let bend = (pitch_bend as f32) - 8192.0;

    // get the pitch_bend_limit parameter
    let pb_limit: u8 = 12;

    // how many semitones we're bending (a fraction of the pitch_bend_limit)
    let semitones;

    // bending up (1 <= bend <= 8191)
    if bend > 0.0 {
        semitones = (pb_limit as f32) * (bend / 8191.0);

    // no bend
    } else if bend == 0.0 {
        semitones = 0.0;

    // bending down (-8192 <= bend <= -1)
    } else {
        semitones = (pb_limit as f32) * (bend / 8192.0);
    }

    // what we multiply by our base frequency to bend it however many semitones we want
    let multiplier = 2.0_f32.powf(semitones / 12.0);

    return 440.0 * multiplier;
}

#[test]
fn test_pitch_bend() {
    let f = pitch_bend(12771);

    println!("{}", f);
}