#![cfg_attr(
    rust_comp_feature = "unstable_features",
    feature(internal_output_capture)
)]

use std::{
    io::{Read as _, Write as _},
    process::Termination,
};

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

static MARKER: &[u8] = &[
    255, 77, 65, 82, 75, 69, 82, 95, 70, 79, 82, 95, 82, 69, 68, 73, 82, 69, 67, 84, 95, 83, 84,
    68, 79, 85, 84,
];
pub fn redirect_stdout(f: impl FnOnce()) -> Result<Vec<u8>> {
    let mut lock = std::io::stdout().lock();
    lock.flush()?;

    let stdout = tempfile::NamedTempFile::new()?;

    let guard = stdio_override::StdoutOverride::override_file(&stdout)?;
    #[cfg(rust_comp_feature = "unstable_features")]
    let old_capture = std::io::set_output_capture(None);
    std::io::stdout().write_all(MARKER)?;
    std::io::stdout().flush()?;
    f();
    std::io::stdout().flush()?;
    #[cfg(rust_comp_feature = "unstable_features")]
    std::io::set_output_capture(old_capture);
    drop(guard);

    let mut res = Vec::new();
    stdout.as_file().read_to_end(&mut res)?;
    if let Some(ret) = res.strip_prefix(MARKER) {
        return Ok(ret.to_vec());
    }
    eprintln!("WARNING: Couldn't read stdout. Please run tests with --nocapture: cargo test -- --nocapture");
    Ok(b"Use `cargo test -- --nocapture` if you care the stdout".to_vec())
}
pub fn redirect_stdin<T>(input: impl AsRef<[u8]>, f: impl FnOnce() -> T) -> Result<T> {
    let mut lock = std::io::stdout().lock();
    lock.flush()?;

    let stdin = tempfile::NamedTempFile::new()?;
    stdin.as_file().write_all(input.as_ref())?;
    let _guard = stdio_override::StdinOverride::override_file(&stdin)?;
    #[cfg(feature = "proconio")]
    {
        // let proconio use line source
        proconio::input_interactive! {};
    }
    Ok(f())
}

pub fn redirect_stdio(input: impl AsRef<[u8]>, f: impl FnOnce()) -> Result<Vec<u8>> {
    redirect_stdin(input.as_ref(), || redirect_stdout(f))?
}

pub fn redirect_stdio_utf8(input: impl AsRef<[u8]>, f: impl FnOnce()) -> Result<String> {
    Ok(String::from_utf8(redirect_stdio(input.as_ref(), f)?)?)
}

pub fn wrap_assert_success<T: Termination>(f: impl FnOnce() -> T) -> impl FnOnce() {
    || {
        let exit_code = f().report();
        let exit_code: u8 = unsafe { std::mem::transmute(exit_code) };
        assert_eq!(exit_code, 0);
    }
}

#[macro_export]
macro_rules! assert_success_with_input {
    ($input: expr, $main: expr) => {{
        $crate::redirect_stdio($input, $crate::wrap_assert_success($main)).unwrap();
    }};
}
#[macro_export]
macro_rules! assert_eq_output_for_input {
    ($input: expr, $left: expr, $right: expr) => {
        let left = redirect_stdio(input, wrap_assert_success(left)).unwrap();
        let right = redirect_stdio(input, wrap_assert_success(right)).unwrap();
        if let (Ok(left), Ok(right)) = (std::str::from_utf8(&left), std::str::from_utf8(&right)) {
            assert_eq!(left, right)
        } else {
            assert_eq!(left, right)
        }
    };
}

#[cfg(feature = "proptest")]
#[macro_export]
macro_rules! prop_assert_success_with_input {
    ($input: expr, $main: expr) => {{
        let ret = $crate::redirect_stdio($input, $crate::wrap_assert_success($main));
        proptest::prop_assert!(ret.is_ok())
    }};
}
#[cfg(feature = "proptest")]
#[macro_export]
macro_rules! prop_assert_eq_output_for_input {
    ($input: expr, $left: expr, $right: expr) => {
        let left = $crate::redirect_stdio($input, $crate::wrap_assert_success($left)).unwrap();
        let right = $crate::redirect_stdio($input, $crate::wrap_assert_success($right)).unwrap();
        if let (Ok(left), Ok(right)) = (std::str::from_utf8(&left), std::str::from_utf8(&right)) {
            proptest::prop_assert_eq!(left, right)
        } else {
            proptest::prop_assert_eq!(left, right)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdin() {
        let s = redirect_stdin("This is input", || {
            std::io::stdin().lines().next().unwrap().unwrap()
        })
        .unwrap();
        assert_eq!(s, "This is input");
    }

    #[test]
    fn test_stdout() {
        let s = redirect_stdout(|| {
            println!("This is output");
        })
        .unwrap();
        assert_eq!(s, "This is output\n".as_bytes());
    }

    #[test]
    fn test_stdio() {
        let s = redirect_stdio_utf8("This is input", || {
            for line in std::io::stdin().lines() {
                println!("{}", line.unwrap());
            }
        })
        .unwrap();
        assert_eq!(s, "This is input\n");
    }

    #[test]
    fn test_success() {
        assert_success_with_input!("foo", || -> Result<(), ()> {
            println!("aaa");
            if std::io::read_to_string(std::io::stdin()).unwrap() == "foo" {
                Ok(())
            } else {
                Err(())
            }
        })
    }

    #[test]
    fn test_runtime_error() {
        let mut lock = std::io::stderr().lock();
        lock.flush().unwrap();

        let stderr = tempfile::NamedTempFile::new().unwrap();
        let _guard = stdio_override::StderrOverride::override_file(&stderr).unwrap();
        let result = std::panic::catch_unwind(|| {
            assert_success_with_input!("bar", || -> Result<(), ()> {
                println!("aaa");
                if std::io::read_to_string(std::io::stdin()).unwrap() == "foo" {
                    Ok(())
                } else {
                    Err(())
                }
            })
        });
        assert!(result.is_err());
    }

    #[cfg(feature = "proptest")]
    mod proptest_features {
        use proptest::prelude::*;
        proptest! {
            #[test]
            fn prop_assert(s: String) {
                prop_assert_success_with_input!(&s, || -> Result<(), ()> {
                    println!("aaa");
                    let input = std::io::read_to_string(std::io::stdin()).unwrap();
                    if input == s {
                        Ok(())
                    } else {
                        Err(())
                    }
                })
            }
        }

        fn wa() {
            let input = std::io::read_to_string(std::io::stdin()).unwrap();
            let mut input = input.split_whitespace();
            let a = input.next().unwrap().parse::<i32>().unwrap();
            let b = input.next().unwrap().parse::<i32>().unwrap();
            println!("{}", a.wrapping_add(b));
        }

        fn ac() {
            let input = std::io::read_to_string(std::io::stdin()).unwrap();
            let mut input = input.split_whitespace();
            let a = input.next().unwrap().parse::<i64>().unwrap();
            let b = input.next().unwrap().parse::<i64>().unwrap();
            println!("{}", a + b);
        }

        #[test]
        #[should_panic]
        fn test_failure() {
            std::env::set_var("PROPTEST_DISABLE_FAILURE_PERSISTENCE", "true");
            proptest!(move |(a: i32, b: i32)| {
                prop_assert_eq_output_for_input!(format!("{} {}", a, b), wa, ac);
            });
        }
    }
}
