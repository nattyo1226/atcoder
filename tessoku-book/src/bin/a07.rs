use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        range: [(usize, usize); n],
    }
    let mut b = vec![0; d + 2];
    for &(l, r) in &range {
        b[l] += 1;
        b[r + 1] -= 1;
    }
    let mut s = 0;
    for i in &b[1..d+1] {
        s += i;
        println!("{}", s);
    }
}
