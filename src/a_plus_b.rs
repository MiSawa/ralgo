use std::ops::Add;

pub fn a_plus_b<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(a_plus_b(2, 3), 5)
    }
}
