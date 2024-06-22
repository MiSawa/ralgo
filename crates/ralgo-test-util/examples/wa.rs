pub fn main() {
    proconio::input! { a: i32, b: i32 }
    println!("{}", a.wrapping_add(b));
}
