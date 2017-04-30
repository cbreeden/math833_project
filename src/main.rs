#![allow(dead_code)]
extern crate gnuplot;
extern crate rand;
extern crate statrs;

use std::iter;

mod hist;
use hist::Histogram;
use hist::Range;

use gnuplot::{Figure, PlotOption, AutoOption, AxesCommon};

mod markov;
use markov::MarkovChain;
use markov::State;
use markov::StateHistory;

fn main() {
    const H: f64   = 0.01;          // 0.01 h = 36 s 
    const T: usize = 100*356*24*100; // 13 years.
    const S: f64   = 61.0;          // IC for CWV

    // Simulate using a MarkovChain
    let mut mc = MarkovChain::new(State::NotPrecipitating, S, H);
    mc.simulate(T);

    let history = mc.get_history();
    
    let mut time = 0.0;
    let mut length_hist = Histogram::new();
    let mut precip_hist = Histogram::new();

    let mut figure = Figure::new();
    figure.set_terminal("pngcairo size 1000, 300", "simulation.png");
    {
        let axis = figure
            .axes2d()
            .set_title("100 Day Simulation", &[])
            .set_x_label("time in days", &[])
            .set_y_label("CWV", &[])
            .set_aspect_ratio(AutoOption::Fix(0.2));

        for &(event, ref ys) in history {
            let color = PlotOption::from(event);

            let num = ys.len();
            let ts  = Range::new(time, H/24.0).take(num);
            time += num as f64 * H/24.0;

            axis.lines(ts, ys, &[color]);

            // Collect PDF
            match event {
                StateHistory::Precipitating(c) => {
                    length_hist.inc(num);   // length of event
                    
                    let c = c as usize;     // total precipitation
                    if c < 10000 {
                        precip_hist.inc(c);
                    } else { println!("Large c! {}", c); }
                }
                _ => { /* NOOP */ }
            }

        }
    }

    figure.show();

    // Plot PDF of total precipitation.
    let mut figure = Figure::new();
    let (xs, ys) = precip_hist.axes();
    //let lxs = lxs.into_iter().map(|x| x as f64 / 100.0);

    figure
        .set_terminal("pngcairo", "hist.png")
        .axes2d()
        .set_title("Density of Total Precipitation", &[])
        .set_x_label("total precip (mm) in precipitation event", &[])
        .set_y_label("number of occurances", &[])
        .set_x_log(Some(10.0))
        .set_y_log(Some(10.0))
        .points(xs, ys, &[]);

    figure.show();
}