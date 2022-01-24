use::core::f64::consts::PI;

fn main() {
    let hamming_window_sinc = WindowSinc::new(32, 0.25, "hamming");
    let k = hamming_window_sinc.kernel();
    
    println!("{:?}", k);
    //print_for_desmos(&k);
    //print_for_pure_data(&k);
}

struct WindowSinc {
    length: u32,
    cutoff: f64,
    window: &'static str,
}

impl WindowSinc {
    fn new(length: u32, cutoff: f64, window: &'static str) -> WindowSinc {
        WindowSinc {
            length,
            cutoff,
            window
        }
    }

    fn kernel(&self) -> Vec<f64> {
        let mut output = Vec::new();
        let mut sum = 0.0;

        for i in 0..self.length + 1 {
            if self.window == "hamming" {
                let o_i = self.sinc(&i) * self.hamming(&i);
                output.push(o_i);
                sum += o_i;
            }
            else if self.window == "blackman" {
                let o_i = self.sinc(&i) * self.blackman(&i);
                output.push(o_i);
                sum += o_i;
            }
            else {
                panic!("input correct window -- either 'hamming' or 'blackman'");
            }
        }

        for i in 0..(output.len() - 1) {
            output[i] = output[i] / sum;
        }

        output
    }

    fn sinc(&self, i: &u32) -> f64 {
        let i = *i as f64;
        let f = self.cutoff;
        let m = self.length as f64;

        if i == m/2.0 {
            2.0*PI * f
        } else {
            (2.0*PI * f * (i - m/2.0)).sin() / (i - m/2.0)
        }
    }

    fn hamming(&self, i: &u32) -> f64 {
        let i = *i as f64;
        let m = self.length as f64;

        0.54 - 0.46 * (2.0*PI * i/m).cos()
    }

    fn blackman(&self, i: &u32) -> f64 {
        let i = *i as f64;
        let m = self.length as f64;

        0.42 - 0.5 * (2.0*PI * i/m).cos() + 0.08 * (4.0*PI * i/m).cos()
    }
}

fn print_for_desmos(v: &Vec<f64>) {
    let mut o = Vec::new();

    for i in 0..v.len() {
        o.push((i, v[i]))
    }
    
    println!("{:?}", o);
}

fn print_for_pure_data(v: &Vec<f64>) {
    for e in v.into_iter() {
        print!("{} ", *e as f32);
    }
}