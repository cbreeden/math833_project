use gnuplot::{PlotOption, Color};

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