use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        range: [(usize, usize); q],
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[i] + a[i]);
    }
    for &(l, r) in &range {
        if 2 * (s[r] - s[l - 1]) > r - l + 1 {
            println!("win");
        } else if 2 * (s[r] - s[l - 1]) == r - l + 1 {
            println!("draw");
        } else {
            println!("lose")
        }
    }
}
