mod integrator;
use integrator::*;

mod naive;
use naive::*;

fn main() {
    let f = f64::cos;
    let mut int = Integrator::new(f);
    println!("{}", int.with_prec(0.00001));
    println!("{}", integrate(32, f));
    /*let mut ans = [0; 7];
    (1..=7u16).for_each(|i| ans[numerator_idx(i, 8)] = i);
    println!("{:?}", ans);*/
}
