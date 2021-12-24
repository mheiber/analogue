use crate::{FrequencyHz, Signal, TimeSecs};
use std::f64::consts::PI;
use std::sync::Arc;

pub fn sine_wave(freq: FrequencyHz) -> Signal {
    let f = move |t| (2.0 * PI * freq.at(t)).sin();
    Signal::new(Arc::new(f))
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
    Signal::new(Arc::new(f))
}
