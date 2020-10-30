// When UserId is 0, it doesn't exist
pub type UserId = u64;

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    name: String,
    // TODO: add options for user (profile img, ranking, etc.)
}

impl User {
    pub fn new(id: UserId, name: String) -> User {
        User { id, name }
    }

    pub fn get_id(&self) -> UserId {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn user_test() {
        let a = User::new(1, "a".to_owned());
        let b = User::new(2, "b".to_owned());

        let c = a.clone();

        assert_eq!(a, c);
        assert_eq!(a.get_id(), 1);
        assert_eq!(b.get_name(), "b");
    }
}
