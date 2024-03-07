use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let mut l = 0;
    let mut r = a.len() - 1;
    let mut m;
    loop {
        m = (l + r) / 2;
        if x < a[m] {
            r = m - 1;
        } else if x == a[m] {
            break;
        } else {
            l = m + 1;
        }
    }
    println!("{:?}", m + 1);
}
