pub mod error;
pub mod controller;
pub mod controllers;

use async_trait::async_trait;
pub use error::{Result, Error};
pub use indiemotion_api as api;