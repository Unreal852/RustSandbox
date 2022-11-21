use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub struct User
{
    pub uuid: Uuid,
    pub username: String
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID: {}, Username: {}", self.uuid, self.username)
    }
}