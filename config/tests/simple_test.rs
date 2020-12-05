#[cfg(test)]
mod simple_test {
    use config::Config;

    #[derive(Config)]
    pub struct A {
        a: u8,
        b: Vec<u8>,
    }

    impl A {
        pub fn new() -> Self {
            A { a: 0, b: vec![] }
        }
    }

    #[test]
    fn simple_set_test() {
        let a = A::new().set_a(4).set_b(vec![2, 3]);
        assert_eq!(a.a, 4);
        assert_eq!(a.b, vec![2, 3]);
    }

    #[test]
    fn simple_map_test() {
        let a = A::new().map_a(|x| x + 1).map_b(|_| vec![2, 3]);
        assert_eq!(a.a, 1);
        assert_eq!(a.b, vec![2, 3]);
    }
}
