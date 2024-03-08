use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: f64,
    }
    let mut l = 0_f64;
    let mut r = 100_f64;
    let mut m = 0_f64;
    for _ in 0..20 {
        m = (l + r) / 2.;
        if abs(f(m) - n) < 0.001 { break; }
        if f(m) > n {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", m);
}

fn f(x: f64) -> f64 {
    x * x * x + x
}

fn abs(x: f64) -> f64 {
    if x > 0. { x } else { -x }
}
