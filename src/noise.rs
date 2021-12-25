use crate::{FrequencyHz, Signal, TimeSecs};

pub fn sample(rate: FrequencyHz, s: Signal) -> impl Iterator<Item = f64> {
    (0..).map(move |n: u32| {
        let sample_period = rate.period();
        let t = n as f64 * sample_period.0;
        s.at(TimeSecs(t))
    })
}
