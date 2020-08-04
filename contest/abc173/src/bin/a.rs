use proconio::input;

fn main() {
    input! {
        a: i64
    }
    println!("{}", (1000 - a % 1000) % 1000);
}
