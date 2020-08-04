use proconio::input;

fn main() {
    let verdicts = vec!["AC", "WA", "TLE", "RE"];
    let mut counts = vec![0; 4];
    input! {
        n: usize,
        a: [String; n],
    };
    for v in a {
        for (i, r) in verdicts.iter().enumerate() {
            if *r == &v {
                counts[i] += 1;
            }
        }
    }
    for i in 0..4 {
        println!("{} x {}", verdicts[i], counts[i])
    }
}
