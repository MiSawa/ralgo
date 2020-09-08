use proconio::{fastout, input, marker::Bytes};
use ralgo::flow::dinic::Dinic;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut b: [Bytes; h]
    }

    let mut dinic = Dinic::new();
    let enc = |u, v| u * w + v;
    let s = h * w;
    let t = s + 1;
    for u in 0..h {
        for v in 0..w {
            if b[u][v] == b'#' {
                continue;
            }
            if (u + v) % 2 == 0 {
                dinic.add_edge(s, enc(u, v), 1);
            } else {
                dinic.add_edge(enc(u, v), t, 1);
            }
        }
    }

    let mut edges = Vec::with_capacity(h * w * 4);
    for u in 0..h {
        for v in 0..w {
            if (u + v) % 2 == 1 || b[u][v] == b'#' {
                continue;
            }
            if u > 0 && b[u - 1][v] == b'.' {
                edges.push((
                    u,
                    v,
                    u - 1,
                    v,
                    b'^',
                    b'v',
                    dinic.add_edge(enc(u, v), enc(u - 1, v), 1),
                ));
            }
            if u + 1 < h && b[u + 1][v] == b'.' {
                edges.push((
                    u,
                    v,
                    u + 1,
                    v,
                    b'v',
                    b'^',
                    dinic.add_edge(enc(u, v), enc(u + 1, v), 1),
                ));
            }
            if v > 0 && b[u][v - 1] == b'.' {
                edges.push((
                    u,
                    v,
                    u,
                    v - 1,
                    b'<',
                    b'>',
                    dinic.add_edge(enc(u, v), enc(u, v - 1), 1),
                ));
            }
            if v + 1 < w && b[u][v + 1] == b'.' {
                edges.push((
                    u,
                    v,
                    u,
                    v + 1,
                    b'>',
                    b'<',
                    dinic.add_edge(enc(u, v), enc(u, v + 1), 1),
                ));
            }
        }
    }
    println!("{}", dinic.max_flow(s, t).0);
    for (u, v, uu, vv, c, cc, e) in edges {
        if dinic.get_flow(&e) != 0 {
            b[u][v] = c;
            b[uu][vv] = cc;
        }
    }
    for line in b {
        println!("{}", String::from_utf8(line).unwrap());
    }
}
