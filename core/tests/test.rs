#[cfg(test)]
mod tests {
    use core::basic;
    use core::base::GameTrait;
    #[test]
    fn Basicgame_process_test1() {
        let game = basic::BasicGame {
            users : vec![0, 1, 2, 3, 4],
            state : basic::BasicState::NotStarted   
        };
        let mut s = Vec::new();
        s.push("1".to_string());
        s.push("2".to_string());
        let t = format!("{:?}", game.process(s));
        assert_eq!(t, "Err(game state is not same. expected: 'n', actual: 2)".to_string());
    }

    #[test]
    fn Basicgame_process_test2() {
        let not_started = basic::BasicGame {
            users : vec![0, 1, 2, 3, 4],
            state : basic::BasicState::NotStarted   
        };
        let mut s1 = Vec::new();
        s1.push("1".to_string());
        s1.push("n".to_string());
        let start_state = not_started.process(s1).unwrap();
        match &start_state {
            basic::BasicState::Start {
                done,
                deck,
                left,
            } => {
                assert_eq!(done.len(), 5);
                assert_eq!(deck.len(), 5);
                assert_eq!(left.len(), 4);
            }
            _ => {
                assert_eq!(1, 2);
            }
        }
        let start = basic::BasicGame {
            users : vec![0, 1, 2, 3, 4],
            state : start_state 
        };
        let mut s2 = Vec::new();
        s2.push("1".to_string());
        s2.push("2".to_string());
        s2.push("2".to_string());
        let t = format!("{:?}", start.process(s2));
        assert_eq!(t, "Err(game state is not same. expected: 's', actual: 2)".to_string());

        let mut s3 = Vec::new();
        s3.push("1".to_string());
        s3.push("s".to_string());
        s3.push("x".to_string());
        let start_state = start.process(s3).unwrap();
        match &start_state {
            basic::BasicState::Start {
                done,
                deck,
                left,
            } => {
                assert_eq!(done.len(), 5);
                assert_eq!(done[1], true);
                assert_eq!(deck.len(), 5);
                assert_eq!(left.len(), 4);
            }
            _ => {
                assert_eq!(1, 2);
            }
        }
    }
}