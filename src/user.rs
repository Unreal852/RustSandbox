use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub struct User
{
    pub uuid: Uuid,
    pub username: String
}

impl User
{
    pub fn new(uuid: Uuid, username: String) -> User
    {
        User{uuid: uuid, username: username}
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID: {}, Username: {}", self.uuid, self.username)
    }
}