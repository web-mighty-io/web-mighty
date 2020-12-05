#[cfg(test)]
mod lifetime_test {
    use config::Config;

    #[derive(Config)]
    pub struct A<'a> {
        a: &'a str,
        b: Vec<u8>,
    }

    impl<'a> A<'a> {
        pub fn new() -> Self {
            A { a: "", b: vec![] }
        }
    }

    #[test]
    fn lifetime_test() {
        let a = A::new().set_a("hello").set_b(vec![2, 3]);
        assert_eq!(a.a, "hello");
        assert_eq!(a.b, vec![2, 3]);
    }
}
