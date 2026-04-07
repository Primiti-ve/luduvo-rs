//! # friends api
//!
//! this module contains structs related to luduvo friends data.

use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

use super::BASE_URL;

/// errors that can occur when fetching the friends data.
#[derive(Error, Debug)]
pub enum FriendsError {
    /// the result with the specified id was not found.
    #[error("result with id `{0}` not found")]
    ResultNotFound(String),

    /// the id is invalid.
    #[error("invalid id `{0}`")]
    InvalidId(String),

    /// the user has sent too many requests to the api.
    #[error("too many requests")]
    TooManyRequests(),

    /// an internal http client error occurred.
    #[error("request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// an error with luduvo servers occurred.
    #[error("there was an error with the luduvo servers: `{0}`")]
    InternalError(String),
}

/// represents a single friend.
#[derive(Clone, Debug, Deserialize)]
pub struct Friend {
    /// the user id of the friend
    pub user_id: u64,

    /// the username of the friend
    pub username: String,
}

/// represents a user's friends returned by the luduvo api.
#[derive(Clone, Debug, Deserialize)]
pub struct Friends {
    /// the list of friends the user has.
    pub friends: Vec<Friend>,

    /// the total amount of friends the user has.
    pub total: u64,

    /// the total amount of friends the user can have at a time.
    pub limit: u64,

    /// the current page of friends.
    pub offset: u64,
}

/// a cached friends entry, containing the user's friends data and its last updated timestamp.
///
/// this is used internally by [`FriendsCache`] to store friends data.
#[derive(Clone)]
pub struct CachedFriends {
    pub result: Friends,
    pub last_updated: u64,
}

/// a cache of user friends data, keyed by user id.
///
/// this is used internally by [`FriendsWrapper`] to cache friends.
#[derive(Clone)]
pub struct FriendsCache {
    cache: HashMap<u64, CachedFriends>,
    cache_timeout: u64,
}

/// the implementation for the friendscache struct.
impl FriendsCache {
    /// creates a new [`FriendsCache`] with the specified cache timeout.
    ///
    /// # arguments
    ///
    /// * `cache_timeout` - the cache timeout in seconds.
    ///
    /// # returns
    ///
    /// - a new [`FriendsCache`] instance
    pub fn new(cache_timeout: u64) -> Self {
        Self {
            cache: HashMap::new(),
            cache_timeout,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// retrieves a result from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the result to retrieve.
    ///
    /// # returns
    ///
    /// - the result if it is still valid (not expired)
    /// - `None` if the result is expired or missing
    pub fn get(&mut self, id: u64) -> Option<Friends> {
        if let Some(entry) = self.cache.get(&id) {
            let age = Self::now() - entry.last_updated;

            if age <= self.cache_timeout {
                return Some(entry.result.clone());
            }
        }

        // expired or missing result entry
        // remove stale entry
        self.cache.remove(&id);

        None
    }

    /// inserts a result into the cache.
    ///
    /// # arguments
    ///
    /// * `result` - the result to insert.
    pub fn insert(&mut self, id: u64, result: Friends) {
        let cached = CachedFriends {
            result,
            last_updated: Self::now(),
        };

        self.cache.insert(id, cached);
    }

    /// removes a result from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the result to remove.
    pub fn remove(&mut self, id: u64) {
        self.cache.remove(&id);
    }
}

/// the configuration for the [`FriendsWrapper`] struct
///
/// # arguments
///
/// * `client` - the [`reqwest::Client`] to use
/// * `base_url` - the base url of the api
/// * `cache_timeout` - the amount of time it takes for cache entries to go stale
#[derive(Clone)]
pub struct FriendsConfig {
    client: Client,
    base_url: String,
    cache_timeout: u64,
}

impl FriendsConfig {
    pub fn new(
        client: Option<Client>,
        base_url: Option<String>,
        cache_timeout: Option<u64>,
    ) -> FriendsConfig {
        FriendsConfig {
            client: client.unwrap_or_default(),
            base_url: base_url.unwrap_or_default(),
            cache_timeout: cache_timeout.unwrap_or_default(),
        }
    }
}

impl Default for FriendsConfig {
    fn default() -> FriendsConfig {
        FriendsConfig {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
            cache_timeout: 30_u64, // clippy was complaining about me using "30 as u64"
        }
    }
}

/// a client for interacting with the luduvo friends api.
///
/// this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.
#[derive(Clone)]
pub struct FriendsWrapper {
    config: FriendsConfig,
    cache: FriendsCache,
}

impl FriendsWrapper {
    /// creates a new [`FriendsWrapper`].
    ///
    /// # notes
    ///
    /// - this internally initializes a reusable [`reqwest::Client`] to perform HTTP requests, which is **not** publicly exposed.
    /// - this internally manages the cache for friends data. the cache is not publicly exposed.
    ///
    /// # arguments
    ///
    /// * `config` - the [`FriendsConfig`] to use.
    ///
    /// # returns
    ///
    /// - a new [`FriendsWrapper`] instance if successful
    pub fn new(config: Option<FriendsConfig>) -> Self {
        let config = config.unwrap_or_default();
        let cache = FriendsCache::new(config.cache_timeout);

        Self { config, cache }
    }

    /// fetches a users friends by id.
    ///
    /// # notes
    ///
    /// - this function is asynchronous.
    ///
    /// # arguments
    ///
    /// * `id` - the user id as a string.
    ///
    /// # errors
    ///
    /// returns:
    /// - [`FriendsError::ResultNotFound`] if the result does not exist (HTTP 404)
    /// - [`FriendsError::RequestFailed`] for network or decoding errors
    /// - [`FriendsError::InvalidId`] if the id is not a valid string
    /// - [`FriendsError::TooManyRequests`] if the user has sent too many requests within a short timespan
    /// - [`Friends`] if successful
    ///
    /// # example
    ///
    /// ```no_run
    /// use luduvo_rs::users::friends::FriendsWrapper;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = FriendsWrapper::new(None);
    ///
    ///     match wrapper.get_friends("1".to_string()).await {
    ///         Ok(friends) => {
    ///             println!("{:#?}", friends);
    ///         }
    ///
    ///         Err(e) => {
    ///             eprintln!("error caught while attempting to get friends: '{}'", e);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_friends(&mut self, id: String) -> Result<Friends, FriendsError> {
        let id_num: u64 = id
            .parse()
            .map_err(|_| FriendsError::InvalidId(id.clone()))?;

        if let Some(friends) = self.cache.get(id_num) {
            return Ok(friends);
        }

        let url = format!("{}/{}/friends", self.config.base_url, id);
        let response = self.config.client.get(&url).send().await?;

        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Err(FriendsError::ResultNotFound(id));
        } else if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(FriendsError::TooManyRequests());
        } else if status == StatusCode::INTERNAL_SERVER_ERROR {
            let reason = status.canonical_reason().unwrap_or("no error supplied");

            return Err(FriendsError::InternalError(reason.to_string()));
        }

        let response = response.error_for_status()?;
        let result = response.json::<Friends>().await?;

        self.cache.insert(id_num, result.clone());

        Ok(result)
    }
}
