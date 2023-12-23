use derive_more::Display;
use std::{ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Display, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Name(Arc<str>);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Create a new name from a string.
///
/// ```
/// use cinemotion::Name;
/// let name = "test".to_string();
/// let name = Name::from(name);
/// ```
///
impl From<String> for Name {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

/// Create a new name from a str.
///
/// ```
/// use cinemotion::Name;
/// let name = Name::from("test");
/// ```
///
impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

/// Create a new name from a string literal.
///
/// ```
/// use cinemotion::name;
/// let name = name!("test");
/// ```
///
#[macro_export]
macro_rules! name {
    ($name:expr) => {{
        $crate::Name::from($name)
    }};
}
