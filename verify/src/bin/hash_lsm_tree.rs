use proconio::input;
use ralgo::data_structures::hash_lsm_tree::HashLSMTree;

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array

fn main() {
    input! {
        q: usize
    };

    let mut tree = HashLSMTree::new();

    for _ in 0..q {
        input! {
            q_type: usize
        }
        if q_type == 0 {
            input! {
                k: usize,
                v: usize,
            }
            tree.insert(k, v);
        } else {
            input! {
                k: usize,
            }
            println!("{}", tree.get(&k).unwrap_or(&0));
        }
    }
}
