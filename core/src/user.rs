// When UserId is 0, it doesn't exist
pub type UserId = u64;

pub struct User<'a> {
    id: UserId,
    name: &'a str,
    // TODO: add options for user (profile img, ranking, etc.)
}

impl<'a> User<'a> {
    pub fn new(id: UserId, name: &str) -> User {
        User { id, name }
    }

    pub fn get_id(&self) -> UserId {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name
    }
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
