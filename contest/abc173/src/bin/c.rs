use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    };

    let mut ret = 0;
    for a in 0..(1 << h) {
        for b in 0..(1 << w) {
            let mut cnt = 0;
            for i in 0..h {
                for j in 0..w {
                    if (a >> i & 1 == 0) && (b >> j & 1 == 0) && (c[i][j] == '#') {
                        cnt += 1
                    }
                }
            }
            if cnt == k {
                ret += 1
            }
        }
    }
    println!("{}", ret);
}
