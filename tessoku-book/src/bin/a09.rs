use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        snow: [(usize, usize, usize, usize); n],
    }
    let mut z = vec![vec![0; w + 2]; h + 2];
    for &(a, b, c, d) in &snow {
        z[a][b] += 1;
        z[a][d + 1] -= 1;
        z[c + 1][b] -= 1;
        z[c + 1][d + 1] += 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            z[i][j] = z[i][j - 1] + z[i][j];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            z[i][j] = z[i - 1][j] + z[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            print!("{}", z[i][j]);
            if j != w {
                print!(" ");
            }
        }
        println!("");
    }
}
