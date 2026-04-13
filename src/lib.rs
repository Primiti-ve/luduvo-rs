//! # luduvo-rs
//!
//! *luduvo-rs* is a rust library for interacting with the [luduvo](luduvo.com) api.
//!
//! ## links
//!
//! - [crates.io](https://crates.io/crates/luduvo-rs)
//! - [docs.rs](https://docs.rs/luduvo-rs/latest/luduvo_rs)
//! - [support server](https://discord.gg/FcjTvuWKRk)
//!
//! ## disclaimers
//!
//! - this crate is a completely fanmade wrapper around the luduvo api.
//! - this crate is MIT-licensed. do whatever with it!
//! - i'm not good at rust; all contributions (pull requests, issues) are welcomed!
//! - you need an asynchronous runtime in order to use this.
//!   - tokio is the intended runtime for this crate!
//! - most users will want to import the prelude, via `luduvo_rs::prelude::*`
//!
//! ## features
//!
//! - user profile data (search by id)
//! - user friends data
//! - user querying (search by username + multiple results)
//!
//! ## quick start
//!
//! ```no_run
//! use luduvo_rs::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut wrapper = ProfileWrapper::new(None);
//!
//!     let id = "1".to_string();
//!     let profile = wrapper.get_user(id).await.unwrap();
//!
//!     println!("hello, {}!", profile.username);
//! }
//! ```
//!
//! ## example
//!
//! the code snippet below is taken from `examples/get_user.rs`!
//!
//! ```no_run
//! use luduvo_rs::users::profile::ProfileWrapper;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut wrapper = ProfileWrapper::new(None);
//!     let id = "1".to_string();
//!
//!     match wrapper.get_user(id.clone()).await {
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
//! ## contributors
//!
//! - [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/Primiti-ve/luduvo-rs/pull/1) (Small changes)
//!
//! ## need help?
//!
//! - contact me on discord! my discord username is `@primiti_ve`.
//!   - ping me in the [luduvo discord](https://discord.gg/luduvo).
//!   - join the [support server](https://discord.gg/FcjTvuWKRk).
//! - create an issue (https://github.com/Primiti-ve/luduvo-rs/issues)

#![allow(unused)]

#[cfg(feature = "prelude")]
pub mod prelude;

#[cfg(feature = "users")]
pub mod users;

#[cfg(feature = "places")]
pub mod places;
