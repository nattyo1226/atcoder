use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let p = enumerate(&a[..n / 2]);
    let mut q = enumerate(&a[n / 2..]);
    q.sort();

    for &p0 in &p {
        let r = q.partition_point(|&x| x < k - p0);
        if r < q.len() && q[r] == k - p0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn enumerate(a: &[usize]) -> Vec<usize> {
    let mut sum_list: Vec<usize> = Vec::new();
    for i in 0..(1 << a.len()) {
        let mut sum = 0;
        for j in 0..a.len() {
            if (i / (1 << j)) % 2 == 1 {
                sum += a[j];
            }
        }
        sum_list.push(sum);
    }
    sum_list
}
