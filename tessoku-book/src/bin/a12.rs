use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l = 1;
    let mut r = 1_000_000_000;
    let mut m;
    while l < r {
        m = (l + r) / 2;
        let mut count = 0;
        for &i in &a {
            count += m / i;
        }
        if count >= k { 
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{}", l);
}