use std::mem;

use rand;
use rand::distributions::{IndependentSample, Normal};
use state::{State, StateHistory};

const P: f64    =  5.0;
const E: f64    =  0.2;
const D2_1: f64 = 36.0;
const D2_0: f64 =  2.0;
const QC: f64   = 65.0;
const QNP: f64  = 62.0;

pub struct MarkovChain2014 {
    state:   State,
    q:       f64,
    h:       f64,
    precip:  f64,
    event:   Vec<f64>,
    history: Vec<(StateHistory, Vec<f64>)>,
    rng:     rand::ThreadRng,
    noise:   Normal,
}

impl MarkovChain2014 {
    pub fn new(state: State, q: f64, h: f64) -> MarkovChain2014 {
        MarkovChain2014 {
            state: state,
            q: q,
            h: h,
            precip: 0.0,
            event: vec![q],
            history: Vec::new(),
            rng:      rand::thread_rng(),
            noise:    Normal::new(0.0, h.sqrt()),
        }
    }

    pub fn sample(&mut self) {
        let w = self.noise.ind_sample(&mut self.rng);

        match self.state {
            State::NotPrecipitating => {
                self.q += E * self.h + D2_0.sqrt() * w;
                self.event.push(self.q);

                if self.q >= QC {
                    self.record_event();
                    self.state = State::Precipitating;
                }
            }

            State::Precipitating => {
                self.q      += -P * self.h + D2_1.sqrt() * w;
                self.precip +=  P * self.h;
                self.event.push(self.q);

                if self.q <= QNP {
                    self.record_event();
                    self.state = State::NotPrecipitating;
                }
            }
        }
    }

    pub fn simulate(&mut self, n: usize) {
        for _ in 0..n {
            self.sample();

            // if self.q > 120.0 || self.q < 25.0 {
            //     println!("Unlikely CWV Error");
            //     println!("state: {:?}", self.state);
            //     println!("CWV: {}", self.q);
            //     println!("Event: {:?}", self.event);
            //     panic!();                
            // }
        }
    }

    pub fn get_history(&mut self) -> &[(StateHistory, Vec<f64>)] {
        if !self.event.is_empty() {
            self.record_event();
        }

        &self.history
    }

    fn record_event(&mut self) {
        let event = mem::replace(&mut self.event, Vec::new());
        let state_hist  = match self.state {
            State::Precipitating => StateHistory::Precipitating(self.precip),
            State::NotPrecipitating => StateHistory::NotPrecipitating(0.0),
        };

        self.history.push( (state_hist, event) );
        self.precip = 0.0;
    }
}