use core::f64::consts::PI;

pub struct Integrator<F: FnMut(f64) -> f64> {
    func: F,
    steps: u32,
    // (i as f64 * PI / n as f64).cos()
    cached: Vec<f64>,
    last: f64,
    err: f64
}

impl<F: FnMut(f64) -> f64> Integrator<F> {
    pub fn new(mut func: F) -> Self {
        let res = Self {
            cached: vec![func(0.), (func(1.) + func(-1.)) * 0.5],
            func,
            steps: 0,
            last: 0.,
            err: std::f64::INFINITY,
        };
        res
    }
    pub fn calc(&self) -> f64 {
        let half = 1 << self.steps;
        let total = half << 1;
        (
            self.a(0) + self.a(half) / (1. - f64::from(total).powi(2))
                + 2. * (1..half).map(|k| {
                    self.a(k) / (1. - f64::from(k << 1).powi(2))
                }).sum::<f64>()
        ) / f64::from(half)
    }
    pub fn step(&mut self) -> f64 {
        let Self {
            ref mut func,
            ref mut cached,
            ref mut steps,
            ..
        } = self;
        let new_num = 1 << *steps;
        *steps += 1;
        let half = new_num << 1;
        let total = half << 1;
        cached.extend((0u16..new_num).map(|i| {
            let numerator = (i << 1) + 1;
            let arg = (PI * f64::from(numerator) / f64::from(total)).cos();
            (func)(arg) + (func)(-arg)
        }));
        let new_res = self.calc();
        let Self {
            ref mut err,
            ref mut last,
            ..
        } = self;
        *err = (new_res - *last).abs() / 2.;
        *last = new_res;
        new_res
    }
    pub fn last(&self) -> f64 {
        self.last
    }
    pub fn with_prec(&mut self, prec: f64) -> f64 {
        while self.err() > prec {
            self.step();
        }
        self.last
    }
    /// Calculates a_2k, but without 2/N
    fn a(&self, k: u16) -> f64 {
        let half = 1 << (self.steps - 1); // aka `N/2`
        let res = self.cached[1]
            + if (k & 1) == 0 { self.cached[0] } else { -self.cached[0] }
            + (1u16..half).map(|n| {
                let idx = numerator_idx(n, half);
                self.cached[2 + idx] * (PI * f64::from(k) * f64::from(n) / f64::from(half)).cos()
            }).sum::<f64>();
        res * 2.
    }
    pub fn err(&self) -> f64 {
        self.err
    }
}

fn numerator_idx(mut nmtr: u16, mut total: u16) -> usize {
    while nmtr & 1 == 0 {
        nmtr >>= 1;
        total >>= 1;
    }
    ((total - 1) as usize >> 1) + (nmtr >> 1) as usize
}
