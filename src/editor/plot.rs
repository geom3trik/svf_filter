// just a place to put the bode plot math
use num::complex::Complex;

use std::f32::consts::PI;

pub fn lin_to_db(gain: f32) -> f32 {
    gain.log(10.0) * 20.0
}

pub fn get_svf_bode(cutoff: f32, k: f32, mode: usize) -> Vec<f32> {
    // bilinear transform, sample rate of 1
    // bogus sample rate of 44100, since it just changes the plot's max value and 22050 seems reasonable
    let g = (PI * cutoff / 44100.).tan();
    // resolution of bodeplot
    let len = 1000;

    let mut array = vec![Complex::new(1., 0.); len];
    let mut frequencies = vec![1.; len]; // frequency has to be in range [0, pi/2] for some reason?
    let base: f32 = 10.;
    for i in 0..len {
        frequencies[i] = base.powf((i + 1) as f32 / (len as f32) * 3. - 3.) * PI / 2.;
    }
    let j = Complex::new(0., 1.);
    let mut curr_s: Complex<f32>;
    match mode {
        0 => {
            // lowpass
            for i in 0..len {
                curr_s = frequencies[i] * j;
                array[i] = g.powi(2) / ((curr_s).powi(2) + k * g * curr_s + g.powi(2));
            }
        }
        1 => {
            // highpass
            for i in 0..len {
                curr_s = frequencies[i] * j;
                array[i] = curr_s.powi(2) / ((curr_s).powi(2) + k * g * curr_s + g.powi(2));
            }
        }
        2 => {
            // bandpass
            for i in 0..len {
                curr_s = frequencies[i] * j;
                array[i] = (g * curr_s) / ((curr_s).powi(2) + k * g * curr_s + g.powi(2));
            }
        }
        3 => {
            // notch
            for i in 0..len {
                curr_s = frequencies[i] * j;
                array[i] =
                    (g.powi(2) + curr_s.powi(2)) / ((curr_s).powi(2) + k * g * curr_s + g.powi(2));
            }
        }
        5 => {
            // bandpass (constant peak gain)
            for i in 0..len {
                curr_s = frequencies[i] * j;
                array[i] = (g * curr_s * k) / ((curr_s).powi(2) + k * g * curr_s + g.powi(2));
            }
        }
        _ => (),
    }
    let mut amplitudes = vec![1.; len];
    for i in 0..len {
        amplitudes[i] = lin_to_db(array[i].norm());
    }
    amplitudes
}

#[test]
fn test_cutoff_value() {
    let amplitudes = get_svf_bode(25.1425, 1. / 0.707, 3);
    // println!("{:?}", amplitudes.iter().max().unwrap());
    let len = 1000;

    let mut frequencies = vec![1.; len]; //? probably normalized angular frequency, that is from 0 to 2 pi
                                         // TODO: Frequency should be spaced not-linearly
                                         // offset to skip the stupid low frequencies. 15 means first value is 9.63 Hz
    let base: f32 = 10.;
    for i in 0..len {
        frequencies[i] = base.powf((i + 1) as f32 / (len as f32) * 3. - 3.) * PI / 2.;
        // turns the frequency to hertz
        frequencies[i] *= 44100. / PI;
    }
    println!("amps: {:?}", &amplitudes[0..20]);
    println!("freqs: {:?}", &frequencies[0..20]);

    println!("current lowest: {}", frequencies[0]);
    println!("current highest: {}", frequencies[999]);
}