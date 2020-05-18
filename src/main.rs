mod integrator;
use integrator::*;

mod naive;
use naive::*;

fn main() {
    let f = f64::cos;
    let mut int = Integrator::new(f);
    println!("{}", int.with_prec(0.000001));
    println!("{}", integrate(100, f));
}
