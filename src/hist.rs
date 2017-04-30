pub struct Histogram {
    pub data: Vec<usize>,
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(n: usize) -> Histogram {
        Histogram {
            data: Vec::with_capacity(n),
        }
    }

    pub fn inc(&mut self, n: usize) {
        if n >= self.data.len() {
            self.data.resize(n+1, 0);
        }

        self.data[n] += 1;
    }

    pub fn axes(&self) -> (Vec<usize>, Vec<usize>) {
        let mut xs = Vec::with_capacity(self.data.len());
        let mut ys = Vec::with_capacity(self.data.len());

        for (value, &count) in self.data.iter().enumerate() {
            if count == 0 { continue; }
            xs.push(value);
            ys.push(count);
        }

        (xs, ys)
    }
}

pub struct Range {
    cur:  f64,
    step: f64,
}

impl Range {
    pub fn new(init: f64, step: f64) -> Range {
        Range {
            cur:  init,
            step: step,
        }
    }
}

impl Iterator for Range {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.cur;
        self.cur += self.step;
        Some(res)
    }
}