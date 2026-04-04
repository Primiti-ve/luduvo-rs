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
//! - user profile data
//! - user friends data
//! 
//! ## quick start
//!
//! ```no_run
//! use luduvo_rs::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut api = ProfileWrapper::new(None);
//!     let profile = api.get_profile("1").await.unwrap();
//! 
//!     println!("Hello, {}!", profile.username);
//! }
//! ```
//!
//! ## example
//!
//! the code snippet below is taken directly from `examples/get_profile.rs`!
//!
//! ```rust
//! use luduvo_rs::users::profile::ProfileWrapper;
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
//! ## need help?
//!
//! - contact me on discord! my discord username is `@primiti_ve`.
//!   - ping me in the [luduvo discord](https://discord.gg/luduvo).
//!   - join the [support server](https://discord.gg/FcjTvuWKRk).
//! - create an issue (https://github.com/Primiti-ve/luduvo-rs/issues)

#![allow(unused)]

pub mod prelude;
pub mod users;
