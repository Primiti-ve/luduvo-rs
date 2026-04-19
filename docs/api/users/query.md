---
icon: lucide/users
---

# query

this module contains structs related to fetching multiple luduvo users at a time.

_this is for searching users by username._ for searching users by id, see the `profile` api.

---

## Client

a client for interacting with the luduvo profile querying api.

```rust
pub struct Client {
    /* private fields */
}
```

### methods

#### new

creates a new `Client`.

```rust
pub fn new(config: Option<Config>) -> Client
```

##### arguments

- `config` - an _optional_ `Config` to use.

##### returns

- if successful, a `Client` instance.

#### get_user

fetches a user profile by username.

```rust
pub async fn get_user(
    &mut self,
    query: String,
    limit: Option<String>,
) -> Result<Query, Error>
```

##### arguments

- `query` - a `String` detailing the username to fetch.
- `limit` - an _optional_ `String` detailing the maximum number of profiles to fetch.

##### returns

- `Error::UserNotFound` if the user does not exist (HTTP 404)
- `Error::RequestFailed` for network or decoding errors
- `Error::TooManyRequests` if the user has sent too many requests within a short timespan
- if successful, a `Query` instance.

---

## Config

the configuration for the `Client` struct.

__no configuration is publicly exposed through the struct!__

```rust
pub struct Config {
    /* private fields */
}
```

### arguments

- `client` - a `reqwest::Client` detailing the http client to use for requests.
- `base_url` - a `String` detailing the base url for the api. by default, this is set to `https://api.luduvo.com/users`.
- `cache_timeout` - a `u64` detailing the time it takes for a cached entry to invalidate. by default, this is set to `30` seconds.

### methods

#### new

```rust
pub fn new(
    client: Option<Client>,
    base_url: Option<String>,
    cache_timeout: Option<u64>,
) -> Config
```

##### arguments

- `client` - an _optional_ `reqwest::Client` detailing the http client to use for requests.
- `base_url` - an _optional_ `String` detailing the base url for the api.
- `cache_timeout` - an _optional_ `u64` detailing the time it takes for a cached entry to invalidate.

#### default

```rust
pub fn default() -> Config
```

---

## Error

a list of errors that can occur when fetching profiles.

```rust
pub enum Error {
    TooManyRequests(),
    RequestFailed(Error),
    InternalError(String),
}
```

### variants

- `TooManyRequests()` - the user has sent too many requests to the api.
- `RequestFailed(reqwest::Error)` - an internal http client error occurred.
- `InternalError(String)` - an error with luduvo servers occurred.

---

## User

represents a user profile returned by the luduvo api.

```rust
pub struct User {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub head_color: String,
    pub torso_color: String,
    pub created_at: u64,
}
```

### fields

- `id` - a `u64` detailing the users id. this is unique per user.
- `username` - a `String` detailing the users username. this is unique per user.
- `display_name` - a `String` detailing the name that other users see. the user can change this at any time.
- `role` - a `String` detailing the users role.
- `head_color` - a `String` detailing the users head colour in a hex code.
- `torso_color` - a `String` detailing the users torso colour in a hex code.
- `created_at` - a `u64` detailing the unix timestamp of when the account was created.

---

## Query

a user query containing a list of `User`s.

```rust
pub struct Query {
    pub users: Vec<User>,
}
```

### fields

- `users` - a `Vec<Users>` detailing a list of users.

---

## CacheEntry

a cached profile entry, containing a profile and its last updated timestamp.

this is used internally by `Cache` to store profile data.

```rust
pub struct CacheEntry {
    pub users: Query,
    pub last_updated: u64,
}
```

### fields

- `users` - a `Query` detailing the cached query.
- `last_updated` - a `u64` detailing the last time the cache entry was updated. this is a unix timestamp.

---

## Cache

a cache of user profiles, keyed by the query.

this is used internally by `Client` to cache profiles.

```rust
pub struct Cache {
    /* private fields */
}
```

### methods

#### new

creates a new `Cache` with the specified cache timeout.

```rust
pub fn new(cache_timeout: u64) -> Cache
```

##### arguments

- `cache_timeout` - a `u64` detailing the cache timeout in seconds.

##### returns

- a new `Cache` instance.

#### get

retrieves a profile from the cache by its id.

```rust
pub fn get(&mut self, query: &str) -> Option<Query>
```

##### arguments

- `query` - a `&str` detailing the id of the profile to retrieve.

##### returns

- the `Query` if it is still valid and not expired.
- `None` if the profile is expired or missing.

#### insert

inserts a profile into the cache.

```rust
pub fn insert(&mut self, query: String, users: Vec<User>)
```

##### arguments

- `query` - a `String` detailing the query to store.
- `users` - a `Vec<Users>` detailing the list of users to store.

#### remove

removes a result from the cache by its query.

```rust
pub fn remove(&mut self, query: &str)
```

##### arguments

- `query` - a `&str` detailing the query to remove from the cache.
