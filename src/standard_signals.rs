use crate::{FrequencyHz, Signal, TimeSecs};
use std::f64::consts::PI;

pub fn sine_wave(freq: FrequencyHz) -> Signal {
    let f = move |t| (2.0 * PI * freq.at(t)).sin();
    Signal::from_fn(f, freq, freq.period())
}

pub fn square_wave(freq: FrequencyHz) -> Signal {
    let f = move |t: TimeSecs| {
        let is_even = (freq.at(t).round() as u32) % 2 == 0;
        if is_even {
            -1.0
        } else {
            1.0
        }
    };
    Signal::from_fn(f, freq, freq.period())
}
