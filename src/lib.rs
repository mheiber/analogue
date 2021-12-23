use std::f32::consts::PI;
use std::fmt::Debug;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;

custom_derive! {
    #[derive(Debug, PartialEq, Clone, PartialOrd, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct TimeSecs(pub f32);
}

custom_derive! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeMul, NewtypeDiv)]
    pub struct FrequencyHz(pub u32);
}

pub struct Signal(pub Box<dyn Fn(TimeSecs) -> f32>);

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("signal")
    }
}

impl From<u32> for TimeSecs {
    fn from(n: u32) -> Self {
        Self(n as f32)
    }
}

impl std::ops::Add for Signal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let f = move |t: TimeSecs| {
            let r1 = self.0(t);
            let r2 = rhs.0(t);
            r1 + r2
        };
        Self(Box::new(f))
    }
}

impl Signal {
    pub fn scale(self, by: f32) -> Signal {
        let f = move |t| by * self.0(t);
        Signal(Box::new(f))
    }
    pub fn at(&self, time: TimeSecs) -> f32 {
        self.0(time)
    }
}

impl FrequencyHz {
    pub fn at(self, t: TimeSecs) -> f32 {
        (self.0 as f32) * t.0
    }

    pub fn period(self) -> TimeSecs {
        TimeSecs(1.0) / self.0.into()
    }
}

pub fn sine_wave(freq: FrequencyHz) -> Signal {
    let f = move |t| (2.0 * PI * freq.at(t)).sin();
    Signal(Box::new(f))
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
    Signal(Box::new(f))
}

pub fn sample(rate: FrequencyHz, s: Signal) -> impl Iterator<Item = f32> {
    (0..).map(move |n: u32| {
        let sample_period = rate.period();
        let t = (n as f32).powf(sample_period.0);
        s.0(TimeSecs(t))
    })
}

// stuff below this line is currently unused
pub struct Weight(pub u32);

custom_derive! {
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy, NewtypeAdd, NewtypeSub, NewtypeDiv, NewtypeMul, NewtypeFrom)]
    pub struct Connection(pub f32);
}

pub fn weighted_sum(ws: Vec<Weight>, cs: Vec<Connection>) -> f32 {
    assert_eq!(ws.len(), cs.len());
    ws.iter().zip(cs).map(|(w, c)| (w.0 as f32) * c.0).sum()
}

pub fn run_weights(ws: Vec<Weight>, cs: Vec<Connection>) -> Connection {
    let negated_cs = cs.iter().map(|c| Connection(-c.0));
    Connection(weighted_sum(ws, negated_cs.collect()))
}
