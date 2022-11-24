use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct UserId(String);

impl From<UserId> for String {
    fn from(value: UserId) -> String {
        value.0
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for UserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
