use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut r = vec![1; n];
    let mut ans = 0;
    for i in 1..=n - 1 {
        r[i] = r[i - 1];
        while r[i] < n && a[r[i]] - a[i - 1] <= k {
            r[i] += 1;
        }
        ans += r[i] - i;
    }
    println!("{}", ans);
}
