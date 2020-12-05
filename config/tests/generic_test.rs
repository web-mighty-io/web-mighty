#[cfg(test)]
mod generic_test {
    use config::Config;

    #[derive(Config)]
    pub struct A<T>
    where
        T: Clone,
    {
        a: T,
        b: Vec<u8>,
    }

    impl<T> A<T>
    where
        T: Clone,
    {
        pub fn new(a: T) -> Self {
            A { a, b: vec![] }
        }
    }

    #[test]
    fn generic_test_a() {
        let a = A::new(0).set_a(4).set_b(vec![2, 3]);
        assert_eq!(a.a, 4);
        assert_eq!(a.b, vec![2, 3]);
    }

    #[derive(Config)]
    pub struct B<T: Clone> {
        a: T,
        b: Vec<u8>,
    }

    impl<T: Clone> B<T> {
        pub fn new(a: T) -> Self {
            B { a, b: vec![] }
        }
    }

    #[test]
    fn generic_test_b() {
        let b = B::new(0).set_a(4).set_b(vec![2, 3]);
        assert_eq!(b.a, 4);
        assert_eq!(b.b, vec![2, 3]);
    }
}
