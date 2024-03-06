use proconio::input;
use proconio::marker::Bytes;
use num::pow;

fn main() {
    input! {
        s: Bytes,
    }
    let mut ans = 0;
    for i in 0..s.len() {
        ans += pow(2, s.len() - i - 1) * (s[i] - b'0');
    }
    println!("{}", ans);
}
