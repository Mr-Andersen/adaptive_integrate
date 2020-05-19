use std::f64::consts::PI;

fn a<F>(k: u16, n: u16, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let nf = f64::from(n);

    let kf = k as f64;
    (f(1.) + f(-1.)) / 2.
        + f(0.) * ((-1i32).pow(k.into()) as f64)
        + (1..=(n / 2 - 1))
            .map(|i| {
                let fi = i as f64;
                let c = (fi * PI / nf).cos();
                (2. * kf * fi * PI / nf).cos() * (f(c) + f(-c))
            })
            .sum::<f64>()
}

// calculate integral from -1 to 1 of function f
pub fn integrate<F>(n: u16, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let nf = n as f64;
    2. / nf
        * (a(0, n, &f)
            + a(n / 2, n, &f) / (1. - nf * nf)
            + 2. * (1..=(n / 2 - 1))
                .map(|i| a(i, n, &f) / (1. - 4. * (i as f64).powi(2)))
                .sum::<f64>())
}
