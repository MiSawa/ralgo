use itertools::Itertools;
use proconio::input;
use ralgo::util::modint::{self, One};

struct Modular();
impl modint::Modular for Modular {
    type I = i32;
    type II = i64;

    fn modular() -> Self::I {
        1000000007
    }

    fn convert(ii: Self::II) -> Self::I {
        ii as Self::I
    }
}
type ModInt = modint::ModInt<Modular>;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    };
    if n == k {
        let res: ModInt = a.iter().cloned().map(ModInt::from_large).product();
        println!("{}", res);
        return;
    }
    let mut negatives = a
        .iter()
        .cloned()
        .filter(|x| *x < 0)
        .map(|x| -x)
        .collect_vec();
    let mut positives = a.iter().cloned().filter(|x| *x > 0).collect_vec();
    if negatives.len() + positives.len() < k {
        println!("0");
        return;
    }
    negatives.sort();
    positives.sort();
    let has_zero = positives.len() + negatives.len() != n;
    if positives.is_empty() {
        if k % 2 == 0 {
            let res: ModInt = negatives
                .iter()
                .rev()
                .take(k)
                .cloned()
                .map(ModInt::from_large)
                .product();
            println!("{}", res)
        } else if has_zero {
            println!("0")
        } else {
            let res: ModInt = negatives
                .iter()
                .take(k)
                .cloned()
                .map(ModInt::from_large)
                .product();
            println!("{}", -res)
        }
    } else {
        let mut res = ModInt::one();
        if k % 2 == 1 {
            res = ModInt::from_large(positives.pop().unwrap())
        }
        let mut pairs = positives
            .iter()
            .rev()
            .tuple_windows()
            .map(|(a, b)| a * b)
            .step_by(2)
            .collect_vec();
        pairs.extend(
            negatives
                .iter()
                .rev()
                .tuple_windows()
                .map(|(a, b)| a * b)
                .step_by(2),
        );
        pairs.sort();
        res *= pairs
            .iter()
            .rev()
            .take(k / 2)
            .cloned()
            .map(ModInt::from_large)
            .product();
        println!("{}", res)
    }
}
