use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }
    a.sort();
    let mut r;
    for &i in &x {
        r = a.partition_point(|&y| y < i);
        println!("{}", r);
    }
}
