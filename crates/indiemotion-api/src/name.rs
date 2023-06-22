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

impl From<String> for Name {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

#[macro_export]
macro_rules! name {
    ($name:expr) => {{
        use $crate::Name;
        Name::from($name)
    }};
}
