use crate::{FrequencyHz, Signal, TimeSecs};

pub fn sample(rate: FrequencyHz, s: Signal) -> impl Iterator<Item = f64> {
    (0..).map(move |n: u32| {
        let sample_period = rate.period();
        let t = n as f64 * sample_period.0;
        s.at(TimeSecs(t))
    })
}

/// Estimates the root mean square (RMS) of a signal over a period
///
/// # Arguments
///
/// * `s` - The `Signal` used to compute the RMS
/// * `period` - The period (starting at phase=0) over which the computation is made
///
pub fn rms(s: Signal, period: TimeSecs) -> f64 {
    // The number of samples should depend on the period.
    let samples = 100;
    let rate = FrequencyHz(((samples as f64) / period.0) as u32);

    // The fold function is derived from:
    //
    // let a_1, ... a_n be the items of the iterator.
    //
    // mean_n = (1/n) * \sum_{i=1}^n a_i
    //        = (1/n) * ( (n - 1) * mean_{n - 1} + a_n )
    //        = mean_{n - 1} + (a_n - mean_{n - 1}) / n
    //
    sample(rate, s)
        .map(|x| x.powf(2.0))
        .enumerate()
        .take(samples)
        .fold(0.0 as f64, |m, (n, x)| {
            m + (x - m) / ((n as i32 + 1) as f64)
        })
        .sqrt()
}
