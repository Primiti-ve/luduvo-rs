use reqwest::Client;
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

/// base url for the luduvo profile api.
pub const BASE_URL: &str = "https://api.luduvo.com/users";

/// errors that can occur when fetching a profile.
#[derive(Error, Debug)]
pub enum ProfileError {
    /// the profile with the specified id was not found.
    #[error("profile with id `{0}` not found")]
    ProfileNotFound(String),

    /// the id is invalid.
    #[error("invalid id `{0}`")]
    InvalidId(String),

    /// an internal http client error occurred.
    #[error("request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
}

/// represents the color configuration of a user's avatar.
///
/// all fields are hex color strings. by default, they are all set to `#C8C8C8`.
#[derive(Clone, Debug, Deserialize)]
pub struct ProfileAvatar {
    pub head_color: String,
    pub torso_color: String,

    pub left_arm_color: String,
    pub right_arm_color: String,

    pub left_leg_color: String,
    pub right_leg_color: String,
}

/// represents a user profile returned by the luduvo api.
#[derive(Clone, Debug, Deserialize)]
pub struct Profile {
    /// the profiles user id.
    /// this is unique to each profile.
    pub user_id: u64,

    /// the profiles username.
    /// this is unique to each profile.
    pub username: String,

    /// display name shown to other users.
    ///
    /// when the account is first created, this defaults to [`username`](Self::username).
    ///
    /// it can be changed by the user at any time.
    pub display_name: String,

    /// optional long-form description of the profile.
    pub bio: Option<String>,

    /// optional short status message of the profile.
    pub status: Option<String>,

    /// the user's avatar appearance configuration.
    /// currently, it is just hex codes for the avatar's limbs.
    pub avatar: ProfileAvatar,

    /// a list of the user's equipped items.
    pub equipped_items: Vec<String>,

    /// a list of badge identifiers earned by the user.
    pub badges: Vec<String>,

    /// the total number of friends the user has.
    pub friend_count: u32,

    /// the total number of owned places the user has.
    pub place_count: u32,

    /// the total number of owned items the user has.
    pub item_count: u32,

    /// last active timestamp (in unix seconds).
    ///
    /// this is a `None` if the user has never logged in.
    pub last_active: Option<u64>,

    /// account creation timestamp (in unix seconds).
    pub member_since: u64,

    /// whether others are allowed to join this user.
    pub allow_joins: bool,

    /// whether the current viewer owns the resource being viewed.
    pub is_owner: bool,
}

/// a cached profile entry, containing a profile and its last updated timestamp.
///
/// this is used internally by [`ProfileCache`] to store profile data.
#[derive(Clone)]
pub struct CachedProfile {
    pub profile: Profile,
    pub last_updated: u64,
}

/// a cache of user profiles, keyed by user id.
///
/// this is used internally by [`ProfileWrapper`] to cache profiles.
#[derive(Clone)]
pub struct ProfileCache {
    cache: HashMap<u64, CachedProfile>,
    cache_timeout: u64,
}

/// the implementation for the profilecache struct.
impl ProfileCache {
    /// creates a new [`ProfileCache`] with the specified cache timeout.
    ///
    /// # arguments
    ///
    /// * `cache_timeout` - the cache timeout in seconds.
    ///
    /// # returns
    ///
    /// - a new [`ProfileCache`] instance
    pub fn new(cache_timeout: u64) -> Self {
        Self {
            cache: HashMap::new(),
            cache_timeout,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// retrieves a profile from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the profile to retrieve.
    ///
    /// # returns
    ///
    /// - the profile if it is still valid (not expired)
    /// - `None` if the profile is expired or missing
    pub fn get(&mut self, id: u64) -> Option<Profile> {
        if let Some(entry) = self.cache.get(&id) {
            let age = Self::now() - entry.last_updated;

            if age <= self.cache_timeout {
                return Some(entry.profile.clone());
            }
        }

        // expired or missing profile entry
        // remove stale entry
        self.cache.remove(&id);

        None
    }

    /// inserts a profile into the cache.
    ///
    /// # arguments
    ///
    /// * `profile` - the profile to insert.
    pub fn insert(&mut self, profile: Profile) {
        let id = profile.user_id;
        let cached = CachedProfile {
            profile,
            last_updated: Self::now(),
        };

        self.cache.insert(id, cached);
    }
    
    /// removes a profile from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the profile to remove.
    pub fn remove(&mut self, id: u64) {
        self.cache.remove(&id);
    }
}

/// a client for interacting with the luduvo profile api.
///
/// this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.
#[derive(Clone)]
pub struct ProfileWrapper {
    client: Client,
    cache: ProfileCache,
}

impl ProfileWrapper {
    /// creates a new [`ProfileWrapper`].
    ///
    /// # notes
    /// 
    /// - this internally initializes a reusable [`reqwest::Client`] to perform HTTP requests, which is **not** publicly exposed.
    /// - this internally manages the cache for profile data. the cache is not publicly exposed.
    /// 
    /// # arguments
    ///
    /// * `cache_timeout` - the cache timeout in seconds. if `None`, defaults to 30 seconds.
    /// 
    /// # returns
    ///
    /// - a new [`ProfileWrapper`] instance if successful
    pub fn new(cache_timeout: Option<u64>) -> Self {
        let cache_timeout = cache_timeout.unwrap_or(30);
        let cache = ProfileCache::new(cache_timeout);

        Self {
            client: Client::new(),
            cache,
        }
    }

    /// creates a new [`ProfileWrapper`] with a provided reqwest client.
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
    /// - a new [`ProfileWrapper`] instance if successful
    pub fn new_with_client(client: Client, cache_timeout: Option<u64>) -> Self {
        let cache_timeout = cache_timeout.unwrap_or(30);
        let cache = ProfileCache::new(cache_timeout);

        Self {
            client,
            cache,
        }
    }

    /// fetch a user profile by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the user id as a string.
    ///
    /// # returns
    ///
    /// - [`ProfileError::ProfileNotFound`] if the profile does not exist (HTTP 404)
    /// - [`ProfileError::RequestFailed`] for network or decoding errors
    /// - [`ProfileError::InvalidId`] if the id is not a valid string
    /// - [`Profile`] if successful
    ///
    /// # examples
    ///
    /// ```no_run
    /// use luduvo_rs::users::profile::ProfileWrapper;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = ProfileWrapper::new(None);
    ///     let profile = wrapper.get_profile("1").await;
    ///
    ///     match profile {
    ///         Ok(profile) => {
    ///             println!("{:#?}", profile);
    ///         },
    ///
    ///         Err(e) => {
    ///             eprintln!("error caught while attempting to get profile: '{}'", e);
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn get_profile(&mut self, id: &str) -> Result<Profile, ProfileError> {
        let id_num: u64 = id
            .parse()
            .map_err(|_| ProfileError::InvalidId(id.to_string()))?;

        {
            let mut cache = &mut self.cache;

            if let Some(profile) = cache.get(id_num) {
                return Ok(profile);
            }
        }

        let url = format!("{}/{}/profile", BASE_URL, id);
        let response = self.client.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ProfileError::ProfileNotFound(id.to_string()));
        }

        let response = response.error_for_status()?;
        let profile = response.json::<Profile>().await?;

        {
            let mut cache = &mut self.cache;

            cache.insert(profile.clone());
        }

        Ok(profile)
    }
}
