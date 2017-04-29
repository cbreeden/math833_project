use rand;
use rand::distributions::{IndependentSample, Range};

fn scaled_tanh(q: f64, neg_inf: f64, inf: f64, q_mid: f64, q_width: f64) -> f64 {
    neg_inf + 0.5 * (inf - neg_inf) * (1.0 + ( (q - q_mid) / q_width ).tanh() )
}

macro_rules! scaled_funcs {
    ( $($name:ident, $neg:expr, $inf:expr, $mid:expr, $width:expr),* ) => (
        $(
            fn $name(q: f64) -> f64 {
                scaled_tanh(q, $neg, $inf, $mid, $width)
            }
        )*
    )
}

scaled_funcs!(
    r_01,  0.0,   1.0,  61.0,  2.0,
    r_10,  4.0,   0.0,  63.0,  2.0,
    evap,  0.2,   0.2,   0.0,  0.0,
    prec,  2.0,  10.0,  64.5,  1.0,
    d2_0,  2.0,   2.0,   0.0,  0.0,
    d2_1,  16.0, 64.04, 64.5,  1.0
);

#[derive(Copy, Clone)]
pub enum State {
    Precipitating,
    NotPrecipitating,
}

pub struct MarkovChain {
    state:   State,
    cwv:     f64,
    h:       f64,
    run:     Vec<f64>,
    history: Vec<(State, Vec<f64>)>,
    rng:     rand::ThreadRng,
    range:   Range<f64>,
}

impl MarkovChain {
    pub fn new(state: State, cwv: f64, h: f64) -> MarkovChain {
        MarkovChain {
            state:   state,
            cwv:     cwv,
            h:       h,
            run:     vec![cwv],
            history: Vec::new(),
            rng:     rand::thread_rng(),
            range:   Range::new(0.0, 1.0),
        }
    }

    pub fn sample(&mut self) {
        let w = self.h * self.range.ind_sample(&mut self.rng);
        let u = self.range.ind_sample(&mut self.rng);

        // Update cwv = q
        match self.state {
            State::NotPrecipitating => {
                let w = d2_0(self.cwv).sqrt() * w;
                self.cwv += evap(self.cwv) * self.h + w;

                if u > self.h * r_01(self.cwv) {
                    self.state = State::Precipitating;
                }
            },

            State::Precipitating => {
                let w = d2_1(self.cwv).sqrt() * w;
                self.cwv += prec(self.cwv) * self.h + w;

                if u > self.h * r_10(self.cwv) {
                    self.state = State::NotPrecipitating;
                }
            }
        }
    }
}