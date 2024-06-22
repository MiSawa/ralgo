# ralgo-test-util

競プロでランダムテストするのに便利そうな関数たち.

[`assert_cmd`](https://crates.io/crates/assert_cmd) とかを使うほうが正しいと思います.

> [!IMPORTANT]
> `cargo test -- --nocapture` のように `--nocapture` を渡すか, nightly を使う必要があります

## なぜ

- ランダムテストを書く為に解答ファイルを書き換えるのは面倒
- できれば同じプロセス内で実行したい

## proptest と共に使う例

WA なコードとナイーブなコードを比較して WA になる入力ケースを探すには, 次のような感じのテストを書けばよい.

```rust
// 中身は
// pub fn main() {
//     proconio::input! { a: i32, b: i32 }
//     println!("{}", a.wrapping_add(b));
// }
// . `main` の `pub` に注意.
#[path = "../src/wa-solution.rs"]
mod wa;

mod ac {
    pub fn main() {
        proconio::input! { a: i64, b: i64 };
        println!("{}", a + b);
    }
}

proptest! {
    #[test]
    fn ac_vs_wa(a: i32, b: i32) {
        ralgo_test_util::prop_assert_eq_output_for_input!(format!("{} {}", a, b), ac::main, wa::main);
    }
}
```

RE なコードから RE する入力ケースを探すには, 次のような感じ.

```rust
// 中身は
// pub fn main() {
//     proconio::input! { a: i32, b: i32 }
//     println!("{}", a + b);
// }
// . `main` の `pub` に注意.
#[path = "../src/re-solution.rs"]
mod re;

proptest! {
    #[test]
    fn locate_re(a: i32, b: i32) {
        ralgo_test_util::prop_assert_success_with_input!(format!("{} {}", a, b), re::main);
    }
}
```


## proptest を使わない例

代わりに `prop_` prefix の無いマクロを使う.

```rust
#[path = "../src/wa-solution.rs"]
mod wa;
#[path = "../src/naive-solution.rs"]
mod naive;
#[test]
fn test() {
    for a in 0..100 {
        for b in 0..100 {
            ralgo_test_util::assert_eq_output_for_input!(format!("{} {}", a, b), ac::main, wa::main);
        }
    }
}
```

