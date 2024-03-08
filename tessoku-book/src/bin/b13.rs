use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b = vec![0];
    for i in 0..n {
        b.push(b[i] + a[i]);
    }

    let mut r = vec![0; n + 1];
    let mut ans = 0;
    for i in 1..=n {
        r[i] = r[i - 1];
        while r[i] < n && b[r[i] + 1] - b[i - 1] <= k {
            r[i] += 1;
        }
        ans += r[i] - i + 1;
    }

    println!("{}", ans);
}
