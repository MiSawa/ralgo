use proconio::{fastout, input, marker::Bytes};
use ralgo::flow::dinic::Dinic;
use ralgo::util::modint::Integer;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut b: [Bytes; h]
    }

    let mut dinic = Dinic::new();
    dinic.reserve(h * w + 2);
    let enc = |u, v| u * w + v;
    let s = h * w;
    let t = s + 1;
    for u in 0..h {
        for v in 0..w {
            if b[u][v] == b'#' {
                continue;
            }
            if (u + v).is_even() {
                dinic.add_edge(s, enc(u, v), 1);
            } else {
                dinic.add_edge(enc(u, v), t, 1);
            }
        }
    }

    let mut up = vec![vec![Option::None; w]; h];
    let mut down = up.clone();
    let mut left = up.clone();
    let mut right = up.clone();

    for u in 0..h {
        for v in 0..w {
            if (u + v).is_odd() || b[u][v] == b'#' {
                continue;
            }
            if u > 0 && b[u - 1][v] == b'.' {
                up[u][v] = Some(dinic.add_edge(enc(u, v), enc(u - 1, v), 1));
            }
            if u + 1 < h && b[u + 1][v] == b'.' {
                down[u][v] = Some(dinic.add_edge(enc(u, v), enc(u + 1, v), 1));
            }
            if v > 0 && b[u][v - 1] == b'.' {
                left[u][v] = Some(dinic.add_edge(enc(u, v), enc(u, v - 1), 1));
            }
            if v + 1 < w && b[u][v + 1] == b'.' {
                right[u][v] = Some(dinic.add_edge(enc(u, v), enc(u, v + 1), 1));
            }
        }
    }
    println!("{}", dinic.max_flow(s, t).0);
    for u in 0..h {
        for v in 0..w {
            if Some(true) == up[u][v].map(|e| dinic.get_flow(&e) != 0) {
                b[u][v] = b'^';
                b[u - 1][v] = b'v';
            }
            if Some(true) == down[u][v].map(|e| dinic.get_flow(&e) != 0) {
                b[u][v] = b'v';
                b[u + 1][v] = b'^';
            }
            if Some(true) == left[u][v].map(|e| dinic.get_flow(&e) != 0) {
                b[u][v] = b'<';
                b[u][v - 1] = b'>';
            }
            if Some(true) == right[u][v].map(|e| dinic.get_flow(&e) != 0) {
                b[u][v] = b'>';
                b[u][v + 1] = b'<';
            }
        }
    }
    for line in b {
        println!("{}", String::from_utf8(line).unwrap());
    }
}
