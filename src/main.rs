#![allow(dead_code)]
extern crate gnuplot;
extern crate rand;
extern crate statrs;
extern crate histogram;

use histogram::Histogram;

use gnuplot::{Figure, PlotOption, AutoOption, AxesCommon};

mod markov;
use markov::MarkovChain;
use markov::State;
use markov::StateHistory;

fn main() {
    const H: f64 = 0.01;

    let mut mc = MarkovChain::new(State::NotPrecipitating, 61.0, H);
    mc.simulate(500*24*100);

    let history = mc.get_history();
    let mut time = 0.0;
    let mut figure = Figure::new();

    {
        let mut axis = figure
            .set_terminal("pngcairo", "simulation.png")
            .axes2d()
            .set_pos(0.0, 0.25)
            .set_aspect_ratio(AutoOption::Fix(0.3));

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

    let mut hist = Histogram::new();
    {
        let mut axis_ts = figure
            .axes2d()
            .set_pos(0.0, -0.25)
            .set_y_range(AutoOption::Fix(-1.0), AutoOption::Fix(3.0))
            .set_aspect_ratio(AutoOption::Fix(0.3));

        for &(event, ref meas) in history {
            let color = PlotOption::from(event);

            let num       = meas.len();
            let mut times = Vec::with_capacity(num);
            for idx in 0..num {
                times.push(time);
                time += (idx as f64) * H;
            }

            let ts_ys = match event {
                StateHistory::NotPrecipitating(_) => vec![0; num],
                StateHistory::Precipitating(_)    => {
                    hist.increment(num as u64);
                    vec![1; num]
                }
            };

            axis_ts.lines(times, ts_ys, &[color]);
        }
    }

    figure.show();

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for bucket in hist.into_iter().take(1000) {
        if bucket.count() == 0 { continue; }
        xs.push(bucket.value() as f64 / 100.0);
        ys.push(bucket.count());
    }

    let mut figure = Figure::new();
    figure
        .set_terminal("pngcairo", "hist.png")
        .axes2d()
        .set_x_log(Some(10.0))
        .set_y_log(Some(10.0))
        .points(xs, ys, &[]);

    figure.show();

}