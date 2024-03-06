use proconio::input;

fn main() {
    input! {
        n: i32,
        sum: i32,
    };
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if 1 <= sum - i - j && sum - i - j <= n  {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
