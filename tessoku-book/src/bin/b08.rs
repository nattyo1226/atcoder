use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(usize, usize); n],
        q: usize,
        rects: [(usize, usize, usize, usize); q],
    }
    let mut z = vec![vec![0; 1501]; 1501];
    for &(x, y) in &points {
        z[x][y] += 1;
    }
    for i in 1..=1500 {
        for j in 1..=1500 {
            z[i][j] = z[i][j - 1] + z[i][j];
        }
    }
    for j in 1..=1500 {
        for i in 1..=1500 {
            z[i][j] = z[i - 1][j] + z[i][j];
        }
    }
    for &(a, b, c, d) in &rects {
        println!("{}", z[c][d] + z[a - 1][b - 1] - z[a - 1][d] - z[c][b - 1]);
    }
}
