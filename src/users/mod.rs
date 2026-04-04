//! # users api
//!
//! this module contains endpoints related to luduvo user data.
//!
//! ## available endpoints
//!
//! - [`profile`] - fetch user profile data
//! - [`friends`] - fetch a user's friends list
//!
//! each endpoint provides a dedicated wrapper struct for interacting with the luduvo api.

pub mod friends;
pub mod profile;
