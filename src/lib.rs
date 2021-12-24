use std::fmt::Debug;
use std::sync::Arc;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

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
    f: Arc<dyn Fn(TimeSecs) -> f64 + Send + Sync + 'static>,
    /// used for .phase()
    add_input: f64,
    /// used for .scale()
    mul_output: f64,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        Self::new(Arc::new(f))
    }
}

impl Signal {
    pub fn sum(signals: Vec<Signal>) -> Signal {
        let f = move |t: TimeSecs| -> f64 { signals.iter().map(|s| s.at(t)).sum() };
        Self::new(Arc::new(f))
    }

    pub fn new(f: Arc<dyn Fn(TimeSecs) -> f64 + Send + Sync + 'static>) -> Self {
        Self {
            f,
            add_input: 0.0,
            mul_output: 1.0,
        }
    }
    pub fn scale(&self, by: f64) -> Signal {
        Self {
            mul_output: self.mul_output * by,
            ..self.clone()
        }
    }
    pub fn phase(&self, by: f64) -> Signal {
        Self {
            add_input: self.add_input + by,
            ..self.clone()
        }
    }
    pub fn at(&self, time: TimeSecs) -> f64 {
        (self.f)(time + TimeSecs(self.add_input)) * self.mul_output
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

pub fn sample(rate: FrequencyHz, s: Signal) -> impl Iterator<Item = f64> {
    (0..).map(move |n: u32| {
        let sample_period = rate.period();
        let t = n as f64 * sample_period.0;
        s.at(TimeSecs(t))
    })
}
