use crate::{FrequencyHz, Signal, TimeSecs};
use rand::distributions::{Distribution, Normal};
use std::sync::Arc;

pub fn sample<'s>(rate: FrequencyHz, s: &'s Signal) -> impl Iterator<Item = f64> + 's {
    (0..).map(move |n: u32| {
        let sample_period = rate.period();
        let t = n as f64 * sample_period.0;
        s.at(TimeSecs(t))
    })
}

/// Estimates the root mean square (RMS) of a signal over a period
///
/// https://en.wikipedia.org/wiki/Root_mean_square
///
/// # Arguments
///
/// * `s` - The `Signal` used to compute the RMS
/// * `period` - The period (starting at phase=0) over which the computation is made
///
pub fn rms(s: &Signal, period: TimeSecs) -> f64 {
    // The number of samples should depend on the period.
    let samples = 100;
    let rate = FrequencyHz(((samples as f64) / period.0) as u32);

    sample(rate, s)
        .map(|x| x.powf(2.0))
        .enumerate()
        .take(samples)
        .fold(0.0 as f64, |running_mean, (n, a)| {
            running_mean + (a - running_mean) / ((n as i32 + 1) as f64)
        })
        .sqrt()
}

/// Applies Gaussian white noise to a Signal at the specified signal to noise ratio.
///
/// The function uses the Additive Gaussian white noise model
/// https://en.wikipedia.org/wiki/Additive_white_Gaussian_noise
///
/// # Arguments
///
/// * `s` - The base Signal to apply noise to.
/// * `signal_to_noise` - The desired signal to noise ratio for the returned signal.
/// * `sample_duration` - A period to analyse the base Signal (should be around one period of the signal)
pub fn gaussian_white_noise(s: Signal, signal_to_noise: f64, sample_duration: TimeSecs) -> Signal {
    let rms_signal = rms(&s, sample_duration);
    let rms_noise = (rms_signal.powf(2.0) / (10.0f64.powf(signal_to_noise / 10.0))).sqrt();
    let dist = Normal::new(0.0, rms_noise.powf(2.0));
    let noise = move |_| dist.sample(&mut rand::thread_rng());
    let noise_signal = Signal::new(Arc::new(noise));
    s.clone() + noise_signal
}
