use crate::filter_type::FilterType;

const PI: f32 = std::f32::consts::PI;

pub struct Vcf {
    sample_rate: f32,

    previous_highpass_output: f32,
    previous_bandpass_output: f32,
    previous_lowpass_output: f32,

    previous_input: f32,
}

impl Vcf {
    pub fn new(sample_rate: f32) -> Vcf {
        Vcf {
            sample_rate,

            previous_highpass_output: 0.0,
            previous_bandpass_output: 0.0,
            previous_lowpass_output: 0.0,

            previous_input: 0.0,
        }
    }

    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    // state variable filter (two poles, resonant)
    pub fn svf(&mut self, val: f32, cutoff: f32, q_factor: f32, filter_type: FilterType) -> f32 {

        let f = f32::sin(PI * cutoff / self.sample_rate);

        let q = q_factor;

        let highpass_output = val - self.previous_lowpass_output - q * self.previous_bandpass_output;

        let bandpass_output = f * highpass_output + self.previous_bandpass_output;

        let lowpass_output = f * bandpass_output + self.previous_lowpass_output;

        self.previous_highpass_output = highpass_output;
        self.previous_bandpass_output = bandpass_output;
        self.previous_lowpass_output = lowpass_output;

        self.previous_input = val;

        match filter_type {
            FilterType::HighPass => {
                return highpass_output;
            },
            FilterType::BandPass => {
                return bandpass_output;
            },
            FilterType::LowPass => {
                return lowpass_output;
            },
        }
    }
}