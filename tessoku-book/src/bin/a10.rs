use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        range: [(usize, usize); d],
    }
    let mut p = vec![0; n + 2];
    let mut q = vec![0; n + 2];
    for i in 1..=n {
        p[i] = max(p[i - 1], a[i - 1]);
        q[n - i + 1] = max(q[n - i + 2], a[n - i]);
    }
    for &(l, r) in &range {
        println!("{}", max(p[l - 1], q[r + 1]));
    }
}
