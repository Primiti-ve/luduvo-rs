use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

/// base url for the luduvo friends api.
pub const BASE_URL: &str = "https://api.luduvo.com/users";

/// errors that can occur when fetching the friends data.
#[derive(Error, Debug)]
pub enum FriendsError {
    /// the result with the specified id was not found.
    #[error("result with id `{0}` not found")]
    ResultNotFound(String),

    /// the id is invalid.
    #[error("invalid id `{0}`")]
    InvalidId(String),

    /// the user has sent too many requests
    #[error("too many requests")]
    TooManyRequests(),

    /// an internal http client error occurred.
    #[error("request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
}

/// represents a single friend
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
    /// the list of friends the user has
    pub friends: Vec<Friend>,

    /// the total amount of friends the user has
    pub total: u64,

    /// the total amount of friends the user can have at a time
    pub limit: u64,

    /// the current page of friends
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

/// a client for interacting with the luduvo friends api.
///
/// this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.
#[derive(Clone)]
pub struct FriendsWrapper {
    client: Client,
    cache: FriendsCache,
    base_url: String,
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
    /// * `cache_timeout` - the cache timeout in seconds. if `None`, defaults to 30 seconds.
    ///
    /// # returns
    ///
    /// - a new [`FriendsWrapper`] instance if successful
    pub fn new(cache_timeout: Option<u64>) -> Self {
        let cache_timeout = cache_timeout.unwrap_or(30);
        let cache = FriendsCache::new(cache_timeout);

        Self {
            client: Client::new(),
            cache,
            base_url: BASE_URL.to_string(),
        }
    }

    /// creates a new [`FriendsWrapper`] with a provided reqwest client.
    ///
    /// # notes
    ///
    /// - the user is responsible for managing the http client.
    ///
    /// # arguments
    ///
    /// * `client` - the reqwest client to use for HTTP requests.
    /// * `cache_timeout` - the cache timeout in seconds.
    ///
    /// # returns
    ///
    /// - a new [`FriendsWrapper`] instance if successful
    pub fn new_with_client(client: Client, cache_timeout: Option<u64>) -> Self {
        let cache_timeout = cache_timeout.unwrap_or(30);
        let cache = FriendsCache::new(cache_timeout);

        Self {
            client,
            cache,
            base_url: BASE_URL.to_string(),
        }
    }

    /// creates a new [`FriendsWrapper`] with a provided base url.
    ///
    /// # notes
    ///
    /// - the user is responsible for making sure the url follows the schema of the luduvo api.
    ///
    /// # arguments
    ///
    /// * `cache_timeout` - the cache timeout in seconds.
    ///
    /// # returns
    ///
    /// - a new [`FriendsWrapper`] instance if successful
    pub fn new_with_base_url(cache_timeout: Option<u64>, base_url: String) -> Self {
        let cache_timeout = cache_timeout.unwrap_or(30);
        let cache = FriendsCache::new(cache_timeout);

        Self {
            client: Client::new(),
            cache,
            base_url,
        }
    }

    /// fetch a users friends data by its id.
    ///
    /// # disclaimers
    ///
    /// - this function is async
    ///
    /// # arguments
    ///
    /// * `id` - the user id as a string.
    ///
    /// # returns
    ///
    /// - [`FriendsError::ResultNotFound`] if the result does not exist (HTTP 404)
    /// - [`FriendsError::RequestFailed`] for network or decoding errors
    /// - [`FriendsError::InvalidId`] if the id is not a valid string
    /// - [`ProfileError::TooManyRequests`] if the user has sent too many requests within a short timespan
    /// - [`Friends`] if successful
    ///
    /// # examples
    ///
    /// ```no_run
    /// use luduvo_rs::users::friends::FriendsWrapper;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = FriendsWrapper::new(None);
    ///
    ///     match wrapper.get_friends("1").await {
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
    pub async fn get_friends(&mut self, id: &str) -> Result<Friends, FriendsError> {
        let id_num: u64 = id
            .parse()
            .map_err(|_| FriendsError::InvalidId(id.to_string()))?;

        {
            if let Some(friends) = self.cache.get(id_num) {
                return Ok(friends);
            }
        }

        let url = format!("{}/{}/friends", self.base_url, id);
        let response = self.client.get(&url).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(FriendsError::ResultNotFound(id.to_string()));
        } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(FriendsError::TooManyRequests());
        }

        let response = response.error_for_status()?;
        let friends = response.json::<Friends>().await?;

        {
            let mut cache = &mut self.cache;

            cache.insert(id_num, friends.clone());
        }

        Ok(friends)
    }
}
