use num::pow;
use proconio::input;

fn main() {
    input! { n: usize, };
    let mut ans = String::from("");
    let mut s: String;
    for i in 0..10 {
        s = (n / (pow(2, i)) % 2).to_string();
        ans = s + &ans;
    }
    println!("{}", ans);
}
