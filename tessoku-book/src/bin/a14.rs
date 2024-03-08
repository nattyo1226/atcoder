use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    
    let mut p = vec![0; n * n];
    let mut q = vec![0; n * n];
    for i in 0..n {
        for j in 0..n {
            p[n * i + j] = a[i] + b[j];
            q[n * i + j] = c[i] + d[j];
        }
    }
    q.sort();

    for &p0 in &p {
        let r = q.partition_point(|&x| x < k - p0);
        if r < n * n && q[r] == k - p0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
