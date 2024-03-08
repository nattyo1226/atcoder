use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut x = a.to_vec();
    x.sort(); 
    x.dedup();

    for i in 0..n {
        let b = x.partition_point(|&y| y < a[i]) + 1;
        if i > 0 {
            print!(" ");
        }
        print!("{}", b);
    }
    println!("");
}
