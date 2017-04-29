#![allow(dead_code)]
extern crate gnuplot;
extern crate rand;
extern crate statrs;

use gnuplot::{Figure, PlotOption};

mod markov;
use markov::MarkovChain;
use markov::State;

fn main() {
    const H: f64 = 0.01;

    let mut mc = MarkovChain::new(State::NotPrecipitating, 61.0, H);
    mc.simulate(100_000);

    let history = mc.get_history();
    let mut time = 0.0;
    let mut figure = Figure::new();

    {
        let mut axis   = figure.axes2d();

        for &(event, ref meas) in history {
            let color = PlotOption::from(event);
            
            let num       = meas.len();
            let mut times = Vec::with_capacity(num);
            for idx in 0..num {
                times.push(time);
                time += (idx as f64) * H;
            }

            axis.lines(times, meas, &[color]);
        }
    }

    figure.show();
}