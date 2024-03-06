use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        range: [(usize, usize); n],
    }
    let mut a = vec![0; t + 1];
    for &(l, r) in &range {
        a[l] += 1;
        a[r] -= 1;
    }
    let mut ans = 0;
    for i in & a[..t] {
        ans += i;
        println!("{}", ans);
    }
}
