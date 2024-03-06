use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    for &i in &a {
        if x == i {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
