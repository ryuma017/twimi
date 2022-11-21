use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Username(String);

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
