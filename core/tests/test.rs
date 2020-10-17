#[cfg(test)]
mod tests {
    use core::basic;
    use core::base::GameTrait;
    #[test]
    fn test() {
        let mut game = basic::BasicGame {
            users : vec![0, 1, 2, 3, 4],
            state : basic::BasicState::NotStarted   
        };
        let mut s = Vec::new();
        let mut s1 = "1".to_string();
        let mut s2 = "2".to_string();
        s.push(s1);
        s.push(s2);
        let t = format!("{:?}", game.process(s));
        assert_eq!(t, "Err(game state is not same. expected: 'n', actual: 2)".to_string());
    }
}