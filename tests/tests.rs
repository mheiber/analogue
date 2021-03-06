#[cfg(test)]
mod tests {
    use analogue::{
        noise::{gaussian_white_noise, rms, sample},
        standard_signals::{sine_wave, square_wave},
        FrequencyHz, Signal, TimeSecs,
    };
    use proptest::prelude::*;

    prop_compose! {
        fn arb_frequency()(n in any::<u32>()) -> FrequencyHz {
            FrequencyHz(n)
        }
    }

    prop_compose! {
        fn arb_timesecs()(t in any::<f64>()) -> TimeSecs {
            TimeSecs(t)
        }
    }

    prop_compose! {
        fn arb_signal()(amplitude in any::<f64>()) -> Signal {
            let f = move |_| amplitude;
            Signal::new(f)
        }
    }

    fn approx_eq(lhs: f64, rhs: f64, tolerance: f64) -> bool {
        (lhs - rhs).abs() <= tolerance
    }

    macro_rules! prop_assert_approx_eq {
        ($actual:expr, $expected: expr) => {
            prop_assert_approx_eq!($actual, $expected, 1e-5)
        };
        ($actual:expr, $expected: expr, $tolerance: expr) => {
            prop_assert!(
                approx_eq($actual, $expected, $tolerance),
                "expected {} ~= {}",
                $actual,
                $expected
            )
        };
    }

    macro_rules! assert_approx_eq {
        ($actual:expr, $expected: expr) => {
            assert_approx_eq!($actual, $expected, 1e-5)
        };
        ($actual:expr, $expected: expr, $tolerance: expr) => {
            assert!(
                approx_eq($actual, $expected, $tolerance),
                "expected {} ~= {}",
                $actual,
                $expected
            )
        };
    }

    // Make this into a property test (depends on making the number of samples dynamic).
    #[test]
    fn rms_sine_wave() {
        let expected_rms = 0.5f64.sqrt();
        for f in (1..100).map(FrequencyHz) {
            assert_approx_eq!(rms(&sine_wave(f), f.period()), expected_rms, 1e-3);
        }
    }

    proptest! {
        // Skip failing test for now
        // #[test]
        // fn test_sine_f_n_fth_is_0(freq in arb_frequency(), n in any::<f64>()) {
        //     prop_assume!(freq > FrequencyHz(0));
        //     let t = freq.period() * TimeSecs(n);
        //     let res = sine_wave(freq).at(t);
        //     prop_assert_approx_eq!(res, 0.0);
        // }

        #[test]
        fn test_sine_f_1_4th_is_1(freq in arb_frequency()) {
            prop_assume!(freq > FrequencyHz(0));
            let t = freq.period() * TimeSecs(0.25);
            prop_assert_approx_eq!(sine_wave(freq).at(t), 1.0);
        }

        #[test]
        fn test_sine_phase_and_scale(freq in arb_frequency()) {
            prop_assume!(freq > FrequencyHz(0));
            let t = freq.period() * TimeSecs(0.25);
            let mut signal = sine_wave(freq);
            signal.scale(2.0);
            signal.phase(2.0 * t.0);
            prop_assert_approx_eq!(signal.at(t), -2.0);
        }
        #[test]
        fn test_sine_f_3_4th_is_neg_1(freq in arb_frequency()) {
            prop_assume!(freq > FrequencyHz(0));
            let t = TimeSecs(0.75) * freq.period();
            prop_assert_approx_eq!(sine_wave(freq).at(t), -1.0);
        }

        #[test]
        fn abs_sample_sine_lt_1(rate in arb_frequency(), freq in arb_frequency()) {
            for v in sample(rate, &sine_wave(freq)).take(100) {
                prop_assert!(v.abs() <= 1.0);
            }
        }

        #[test]
        fn sample_square_1_or_neg_1(rate in arb_frequency(), freq in arb_frequency()) {
            for v in sample(rate, &square_wave(freq)).take(100) {
                prop_assert!(v.abs() == 1.0);
            }
        }

        #[test]
        fn square_f_0_is_neg_1(freq in arb_frequency()) {
            prop_assert_approx_eq!(square_wave(freq).at(TimeSecs(0.0)), -1.0);
        }

        #[test]
        fn sample_sine_phase_is_periodic(f in arb_frequency(), phase in any::<f64>()) {
            prop_assume!(f > FrequencyHz(0));
            prop_assume!(phase >= 0.0);
            let mut wave = sine_wave(f);
            wave.phase(1.0);
            let expected = wave.at(TimeSecs(0.0));
            for v in sample(f, &wave).take(10) {
                prop_assert_approx_eq!(v, expected);
            }
        }

        #[test]
        fn rms_square_wave(f in arb_frequency(), p in arb_timesecs()) {
            prop_assume!(f > FrequencyHz(0));
            prop_assume!(p > TimeSecs(0.0));
            prop_assert_approx_eq!(rms(&square_wave(f), p), 1.0);
        }

        #[test]
        fn gaussian_white_noise_deterministic(f in arb_frequency(), t in arb_timesecs()) {
            prop_assume!(f > FrequencyHz(0));
            let s = gaussian_white_noise(square_wave(f), 10.0, TimeSecs(1.0));
            let test_values = (0..100).map(|_| s.at(t));
            prop_assert_eq!(test_values.clone().fold(f64::NEG_INFINITY, f64::max),
                            test_values.clone().fold(f64::INFINITY, f64::min));
        }
    }
}
