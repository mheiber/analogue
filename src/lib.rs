use custom_derive::custom_derive;
use newtype_derive::*;
use std::fmt::Debug;
use std::sync::Arc;

pub mod noise;
pub mod standard_signals;

custom_derive! {
    #[derive(Debug, PartialEq, Clone, PartialOrd, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct TimeSecs(pub f64);
}

custom_derive! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct FrequencyHz(pub u32);
}

#[derive(Clone)]
pub struct Signal {
    f: Arc<dyn Fn(TimeSecs) -> f64 + Send + Sync>,
    /// used for increasing frequency
    mul_input: f64,
    /// used for .phase()
    add_input: f64,
    /// used for .scale()
    mul_output: f64,
}

impl Default for Signal {
    fn default() -> Self {
        let f = Arc::new(|_| 0.0);
        Self {
            f,
            mul_input: Default::default(),
            add_input: Default::default(),
            mul_output: Default::default(),
        }
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("signal").unwrap();
        f.write_str("signal")
    }
}

impl std::ops::Add for Signal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let f = move |t: TimeSecs| {
            let r1 = self.at(t);
            let r2 = rhs.at(t);
            r1 + r2
        };
        Self::new(f)
    }
}

impl Signal {
    pub fn sum(signals: Vec<Signal>) -> Signal {
        let f = move |t: TimeSecs| -> f64 { signals.iter().map(|s| s.at(t)).sum() };
        Self::new(f)
    }

    pub fn new<F>(f: F) -> Self
    where
        F: Fn(TimeSecs) -> f64 + Send + Sync + 'static,
    {
        Self {
            f: Arc::new(f),
            mul_input: 1.0,
            add_input: 0.0,
            mul_output: 1.0,
        }
    }
    pub fn scale(&mut self, by: f64) {
        self.mul_output *= by;
    }
    pub fn incr_frequency(&mut self, by: f64) {
        self.mul_input *= by;
    }
    pub fn phase(&mut self, by: f64) {
        self.add_input += by;
    }
    pub fn at(&self, time: TimeSecs) -> f64 {
        (self.f)(TimeSecs(self.mul_input * time.0 + self.add_input)) * self.mul_output
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
