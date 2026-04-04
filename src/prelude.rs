//! # prelude
//!
//! this module re-exports commonly used types in luduvo-rs.
//!
//! ## why use the prelude
//!
//! instead of importing individual types/structs like:
//!
//! ```rust
//! use luduvo_rs::users::profile::{Profile, ProfileWrapper};
//! use luduvo_rs::users::friends::{Friends, FriendsWrapper};
//! ```
//!
//! you can simply do:
//!
//! ```rust
//! use luduvo_rs::prelude::*;
//! ```
//!
//! this is especially useful in small scripts, examples, or when you are using multiple parts of the crate at once.
//!
//! ## re-exported items
//!
//! ### profile api
//! - [`Profile`]
//! - [`ProfileWrapper`]
//! - [`ProfileError`]
//!
//! ### friends api
//! - [`Friends`]
//! - [`FriendsWrapper`]
//! - [`FriendsError`]
//!
//! ## example
//!
//! ```no_run
//! use luduvo_rs::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut wrapper = ProfileWrapper::new(None);
//!     let id = "1";
//!
//!     match wrapper.get_profile(id).await {
//!         Ok(profile) => {
//!             println!("profile for id `{id}`: {:#?}", profile);
//!         }
//!
//!         Err(e) => {
//!             eprintln!(
//!                 "error caught while attempting to get profile for id `{id}`: '{:#?}'",
//!                 e
//!             );
//!         }
//!     }
//! }
//! ```
//!
//! ## when not to use prelude
//!
//! if you prefer more explicit imports (recommended for large projects), you may want to import items directly from their modules instead. this avoids namespace pollution and makes dependencies clearer.

use super::users;

pub use users::friends::{Friends, FriendsError, FriendsWrapper};
pub use users::profile::{Profile, ProfileError, ProfileWrapper};
