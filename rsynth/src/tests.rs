use std::sync::Arc;

use crate::params;

fn ms_to_samples(time: f32, sample_rate: f32) -> u32 {
    (sample_rate * time / 1000.0) as u32
}

#[test]
fn parameters_loading() {
    let params1 = Arc::new(params::RParams::default());

    let params = params1.clone();

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

    println!("{}", ms_to_samples(attack, 44100.0));
    println!("{}", ms_to_samples(decay, 44100.0));
    println!("{}", sustain);
    println!("{}", ms_to_samples(release, 44100.0));
}