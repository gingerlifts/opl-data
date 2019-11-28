extern crate kana; // For conversion between Japanese writing systems

mod usernames;
pub use crate::usernames::make_username;
pub use crate::usernames::is_eastasian;
