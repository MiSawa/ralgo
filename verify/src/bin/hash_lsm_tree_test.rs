use proconio::input;
use ralgo::data_structure::hash_lsm_tree::HashLSMTree;

// verify-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array

fn main() {
    let mut source = proconio::STDIN_SOURCE.lock().unwrap();
    input! {
        from &mut *source,
        q: usize
    };

    let mut tree = HashLSMTree::new();

    for _ in 0..q {
        input! {
            from &mut *source,
            q_type: usize
        }
        if q_type == 0 {
            input! {
                from &mut *source,
                k: usize,
                v: usize,
            }
            tree.insert(k, v);
        } else {
            input! {
                from &mut *source,
                k: usize,
            }
            println!("{}", tree.get(&k).unwrap_or(&0));
        }
    }
}
