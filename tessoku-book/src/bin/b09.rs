use proconio::input;

fn main() {
    input! {
        n: usize,
        papers: [(usize, usize, usize, usize); n],
    }
    let mut z = vec![vec![0; 1501]; 1501];
    for &(a, b, c, d) in &papers {
        z[a][b] += 1;
        z[a][d] -= 1;
        z[c][b] -= 1;
        z[c][d] += 1;
    }
    for i in 0..=1500 {
        for j in 1..=1500 {
            z[i][j] = z[i][j - 1] + z[i][j];
        }
    }
    for j in 0..=1500 {
        for i in 1..=1500 {
            z[i][j] = z[i - 1][j] + z[i][j];
        }
    }
    let mut ans = 0;
    for i in 0..=1500 {
        for j in 0..=1500 {
            if z[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
