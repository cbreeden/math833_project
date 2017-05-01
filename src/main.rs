#![allow(dead_code)]
extern crate gnuplot;
extern crate rand;
extern crate statrs;


mod hist;
use hist::Histogram;
use hist::Range;

use gnuplot::{Figure, PlotOption, AutoOption, AxesCommon, Caption};

mod state;
use state::{State, StateHistory};

mod markov2011;
use markov2011::MarkovChain2011;

mod markov2014;
use markov2014::MarkovChain2014;

const H: f64   = 0.01;         // 0.01 h = 36 s
const T: usize = 100*24*100; // 13 years.
const S: f64   = 61.0;         // IC for CWV

fn main() {
    let mut mc11 = MarkovChain2011::new(State::NotPrecipitating, S, H);
    mc11.simulate(T);

    let mut mc14 = MarkovChain2014::new(State::NotPrecipitating, S, H);
    mc14.simulate(T);

    let history11 = mc11.get_history();
    let history14 = mc14.get_history();

    let mut lhist11 = Histogram::new();
    let mut phist11 = Histogram::new();
    let mut lhist14 = Histogram::new();
    let mut phist14 = Histogram::new();

    plot_simulation(history14,
        "simulation14.png",
        "100 Day Simulation (2014)",
        &mut lhist14,
        &mut phist14);

    plot_simulation(history11,
        "simulation11.png",
        "100 Day Simulation (2011)",
        &mut lhist11,
        &mut phist11);

    // Plot PDF of total precipitation.
    let mut figure = Figure::new();

    let (xs14, ys14) = phist14.axes();
    let xs14 = xs14.into_iter().map(|x| x as f64 / 10.0);

    let (xs11, ys11) = phist11.axes();
    let xs11 = xs11.into_iter().map(|x| x as f64 / 10.0);

    figure
        .set_terminal("pngcairo", "hist.png")
        .axes2d()
        .set_title("Density of Total Precipitation", &[])
        .set_x_label("total precip (mm) in precipitation event", &[])
        .set_y_label("number of occurances", &[])
        .set_x_log(Some(10.0))
        .set_y_log(Some(10.0))
        .points(xs14, ys14, &[Caption("2014 Model")])
        .points(xs11, ys11, &[Caption("2011 Model")]);

    figure.show();
}

fn plot_simulation(
        history: &[(StateHistory, Vec<f64>)],
        name: &str,
        title: &str,
        length_hist: &mut Histogram,
        precip_hist: &mut Histogram
    ) {
    let mut time = 0.0;

    let mut figure = Figure::new();
    {
        let axis = figure
            .set_terminal("pngcairo size 1000, 300", name)
            .axes2d()
            .set_title(title, &[])
            .set_x_label("time in days", &[])
            .set_y_label("cwv", &[])
            .set_aspect_ratio(AutoOption::Fix(0.2))
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(T as f64));

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

                    let c = (c * 10.0) as usize;
                    if c < 10000 {
                        precip_hist.inc(c);
                    } else { println!("Large c! {}", c); }
                }
                _ => { /* NOOP */ }
            }
        }
    }
    figure.show();
}