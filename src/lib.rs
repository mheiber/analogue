use std::fmt::Debug;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

pub mod noise;
pub mod standard_signals;

custom_derive! {
    #[derive(Debug, PartialEq, Clone, PartialOrd, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct TimeSecs(pub f64);
}

custom_derive! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct FrequencyHz(pub u64);
}

#[derive(Clone, Debug)]
pub struct Signal {
    sample_rate: FrequencyHz,
    period: TimeSecs,
    samples: Vec<f64>,
}

impl std::ops::Mul<TimeSecs> for FrequencyHz {
    type Output = u64;
    fn mul(self, rhs: TimeSecs) -> Self::Output {
        (self.0 as f64 * rhs.0) as u64
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            sample_rate: FrequencyHz(1),
            period: TimeSecs(1.0),
            samples: vec![0.0],
        }
    }
}

impl std::ops::Add for Signal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Signal::sum(&[self, rhs])
    }
}

impl Signal {
    pub fn new(samples: Vec<f64>, sample_rate: FrequencyHz, period: TimeSecs) -> Self {
        let expected = sample_rate * period - 1;
        if expected > samples.len() as u64 {
            panic!("expected at least {} samples ({:?} * {:?} - 1) but got {}", expected, sample_rate, period, samples.len());
        }
        Self {
            sample_rate,
            period,
            samples,
        }
    }
    pub fn from_fn<F>(f: F, sample_rate: FrequencyHz, period: TimeSecs) -> Self
    where
        F: Fn(TimeSecs) -> f64,
    {
        let samples = (0..sample_rate * period)
            .map(|t| TimeSecs(t as f64))
            .map(f)
            .collect();
        Self {
            samples,
            sample_rate,
            period,
        }
    }

    pub fn sum(signals: &[Self]) -> Self {
        if signals.is_empty() {
            Default::default()
        }
        else {
            let max_period =
                signals
                    .iter()
                    .map(|s| s.period)
                    .fold(TimeSecs(1.0), |max, period| if max >= period { max } else { period });
            let max_sample_rate =
                signals
                    .iter()
                    .map(|s| s.sample_rate).max().unwrap_or(FrequencyHz::default());

            let f = |t| {
                let vals = signals.iter().map(|s| s.at(t));
                let v: Vec<_> = vals.clone().collect();
                vals.sum()
            };
            Signal::from_fn(f, max_sample_rate, max_period)
        }
    }

    pub fn scale(&mut self, by: f64) {
        self.samples = self.samples.iter().map(|amp| amp * by).collect();
    }
    pub fn incr_frequency(&mut self, by: f64) {
        self.period = TimeSecs(self.period.0 / by);
        self.sample_rate = FrequencyHz((self.sample_rate.0 as f64 / by) as u64);
    }
    pub fn phase(&mut self, by: TimeSecs) {
        let pivot = (self.sample_rate * by).rem_euclid(self.samples.len() as u64) as usize ;

        let mut v = Vec::with_capacity(self.samples.len());
        v.extend(self.samples[pivot..].to_vec());
        v.extend(self.samples[0..pivot].to_vec());
        self.samples = v;
    }
    pub fn at(&self, time: TimeSecs) -> f64 {
        self.samples[self.index_for(time)]
    }
    fn index_for(&self, time: TimeSecs) -> usize {
        let index = (self.sample_rate * TimeSecs(time.0.rem_euclid(self.period.0)));
        // TODO: think on this more
        // modulo again to avoid off-by-one errors
        index.rem_euclid(self.samples.len() as u64) as usize
    }
}

impl FrequencyHz {
    pub fn at(self, t: TimeSecs) -> f64 {
        (self.0 as f64) * t.0
    }

    pub fn period(self) -> TimeSecs {
        TimeSecs(1.0) / TimeSecs(self.0 as f64)
    }
}
