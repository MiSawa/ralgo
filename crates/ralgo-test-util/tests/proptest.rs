use proptest::prelude::*;
use ralgo_test_util::*;

use std::io::Write as _;

#[path = "../examples/ac.rs"]
mod ac;
#[path = "../examples/re.rs"]
mod re;
#[path = "../examples/wa.rs"]
mod wa;

proptest! {
    #[test]
    fn ac_vs_ac(a: i32, b: i32) {
        prop_assert_eq_output_for_input!(format!("{} {}", a, b), ac::main, ac::main);
    }
}

#[test]
#[should_panic]
fn ac_vs_wa() {
    std::env::set_var("PROPTEST_DISABLE_FAILURE_PERSISTENCE", "true");
    proptest!(move |(a: i32, b: i32)| {
        prop_assert_eq_output_for_input!(format!("{} {}", a, b), ac::main, wa::main);
    });
}

#[test]
#[should_panic]
fn locate_re() {
    std::env::set_var("PROPTEST_DISABLE_FAILURE_PERSISTENCE", "true");
    let mut lock = std::io::stderr().lock();
    lock.flush().unwrap();

    let stderr = tempfile::NamedTempFile::new().unwrap();
    let _guard = stdio_override::StderrOverride::override_file(&stderr).unwrap();
    proptest!(move |(a: i32, b: i32)| {
        prop_assert_success_with_input!(format!("{} {}", a, b), re::main);
    });
}
