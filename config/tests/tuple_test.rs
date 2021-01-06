#[cfg(test)]
mod tuple_test {
    use config::Config;

    #[derive(Config)]
    pub struct A(u8, Vec<u8>);

    impl A {
        pub fn new() -> Self {
            A(0, vec![])
        }
    }

    #[test]
    fn tuple_test() {
        let a = A::new().set_0(4).set_1(vec![2, 3]);
        assert_eq!(a.0, 4);
        assert_eq!(a.1, vec![2, 3]);
    }
}
