use rand;
use rand::distributions::{IndependentSample, Range, Normal};
use std::mem;

use gnuplot::{PlotOption, Color};

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
    r_10,  4.0,   0.0,  61.0,  2.0,
    evap,  0.4,   0.4,   0.0,  0.0,
    prec,  3.0,   3.0,  64.5,  1.0,
    d2_0,  2.0,   2.0,   0.0,  0.0,
    d2_1,  16.0, 64.04, 64.5,  1.0
);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Precipitating,
    NotPrecipitating,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateHistory {
    Precipitating(f64),
    NotPrecipitating(f64),
}

impl From<StateHistory> for PlotOption<'static> {
    fn from(s: StateHistory) -> PlotOption<'static> {
        match s {
            StateHistory::Precipitating(_)    => Color("blue"),
            StateHistory::NotPrecipitating(_) => Color("red"),
        }
    }
}

pub struct MarkovChain {
    state:   State,
    cwv:     f64,
    h:       f64,
    prec:    f64,
    event:   Vec<f64>,
    history: Vec<(StateHistory, Vec<f64>)>,
    rng:     rand::ThreadRng,
    uniform: Range<f64>,
    noise:   Normal,
}

impl MarkovChain {
    pub fn new(state: State, cwv: f64, h: f64) -> MarkovChain {
        MarkovChain {
            state:    state,
            cwv:      cwv,
            h:        h,
            prec:     0.,
            event:    vec![cwv],
            history:  Vec::new(),
            rng:      rand::thread_rng(),
            uniform:  Range::new(0.0, 1.0),
            noise:    Normal::new(0.0, h.sqrt()),
        }
    }

    pub fn sample(&mut self) {
        let w = self.noise.ind_sample(&mut self.rng);
        let u = self.uniform.ind_sample(&mut self.rng);

        // Update cwv = q
        match self.state {
            State::NotPrecipitating => {
                let w = d2_0(self.cwv).sqrt() * w;
                self.prec += evap(self.cwv) * self.h;
                self.cwv += self.prec + w;

                self.event.push(self.cwv);

                if u <= 1.0 - (-self.h * r_01(self.cwv)).exp() {
                    self.state = State::Precipitating;
                    let event = mem::replace(&mut self.event, Vec::new());
                    self.history.push(
                        (StateHistory::NotPrecipitating(self.prec), event)
                    );
                    self.prec = 0.0;
                }
            },

            State::Precipitating => {
                let w = d2_1(self.cwv).sqrt() * w;
                self.prec += -prec(self.cwv) * self.h;
                self.cwv += self.prec - w;

                self.event.push(self.cwv);

                if u <= 1.0 - (-self.h * r_10(self.cwv)).exp() {
                    self.state = State::NotPrecipitating;
                    let event = mem::replace(&mut self.event, Vec::new());
                    self.history.push(
                        (StateHistory::Precipitating(self.prec), event)
                    );
                    self.prec = 0.0;
                }
            }
        }
    }

    pub fn simulate(&mut self, n: usize) {
        for _ in 0..n {
            self.sample();

            // if self.cwv > 120.0 || self.cwv < 25.0 {
            //     println!("Unlikely CWV Error");
            //     println!("state: {:?}", self.state);
            //     println!("CWV: {}", self.cwv);
            //     println!("Event: {:?}", self.event);
            //     break;
            // }
        }
    }

    pub fn get_history(&mut self) -> &[(StateHistory, Vec<f64>)] {
        if !self.event.is_empty() {
            let event = mem::replace(&mut self.event, Vec::new());
            let state = match self.state {
                State::NotPrecipitating => StateHistory::NotPrecipitating(self.prec),
                State::Precipitating    => StateHistory::Precipitating(self.prec),
            };
            self.history.push( (state, event) );
        }

        &self.history
    }
}

#[cfg(test)]
mod test {
    use gnuplot::{Figure, Caption, Color, AutoOption, Graph, Axis};
    use gnuplot::AxesCommon;
    use super::*;

    #[test]
    #[ignore]
    fn plot_functions() {
        const N: usize = 1000;
        const L: f64   = 40.0;
        const R: f64   = 80.0;

        let xs = step(L, R, N);
        let r01_ys = evaluate(r_01, &xs);
        let r10_ys = evaluate(r_10, &xs);

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "transitions.png")
            .axes2d()
            .set_y_range(AutoOption::Fix(-1.0), AutoOption::Fix(5.0))
            .set_aspect_ratio(AutoOption::Fix(0.5))
            .lines(&xs, &r01_ys, &[Caption("r_{01}"), Color("black")])
            .lines(&xs, &r10_ys, &[Caption("r_{10}"), Color("blue")]);

        fg.show();

        let evap_ys = evaluate(evap, &xs);
        let prec_ys = evaluate(prec, &xs);

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "source.png")
            .axes2d()
            .set_legend(Graph(0.35), Graph(0.9), &[], &[])
            .set_y_range(AutoOption::Fix(-1.0), AutoOption::Fix(11.0))
            .set_aspect_ratio(AutoOption::Fix(0.5))
            .lines(&xs, &evap_ys, &[Caption("Evaporation"), Color("black")])
            .lines(&xs, &prec_ys, &[Caption("Precipitation"), Color("blue")]);

        fg.show();

        let d20_ys = evaluate(d2_0, &xs);
        let d21_ys = evaluate(d2_1, &xs);

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "noise.png")
            .axes2d()
            .set_legend(Graph(0.35), Graph(0.9), &[], &[])
            .set_y_range(AutoOption::Fix(-3.0), AutoOption::Fix(70.0))
            .set_aspect_ratio(AutoOption::Fix(0.5))
            .lines(&xs, &d20_ys, &[Caption("Evaporation"), Color("black")])
            .lines(&xs, &d21_ys, &[Caption("Precipitation"), Color("blue")]);

        fg.show();
    }

    fn step(l: f64, r: f64, n: usize) -> Vec<f64> {
        let mut xs = Vec::with_capacity(n);
        let step: f64 = (r - l) / (n as f64);

        for idx in 0..n {
            xs.push(l + idx as f64 * step);
        }

        xs
    }

    fn evaluate<F: Fn(f64) -> f64>(f: F, xs: &[f64]) -> Vec<f64> {
        let mut ys = Vec::with_capacity(xs.len());

        for &x in xs {
            ys.push(f(x));
        }

        ys
    }
}