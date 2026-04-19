---
icon: lucide/handshake
---

# friends

this module contains structs related to luduvo friends data.

---

## Client

a client for interacting with the luduvo friends api.

__the configuration and client aren't publicly exposed through the struct!__

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

#### get_friends

```rust
pub async fn get_friends(&mut self, id: String) -> Result<Friends, Error>
```

fetches a users friends by id.

##### arguments

- `id` - the user id as a `String`.

##### returns

- `Error::ResultNotFound` if the result does not exist (HTTP 404).
- `Error::RequestFailed` for network or decoding errors.
- `Error::InvalidId` if the id is not a valid string.
- `Error::TooManyRequests` if the user has sent too many requests within a short timespan.
- if successful, a `Friends` instance.

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

the list of errors that can occur when fetching the friends data.

```rust
pub enum Error {
    ResultNotFound(String),
    InvalidId(String),
    TooManyRequests(),
    RequestFailed(Error),
    InternalError(String),
}
```

### variants

- `ResultNotFound(String)` - the result with the specified id was not found.
- `InvalidId(String)` - the id is invalid and not found in luduvo's database.
- `TooManyRequests()` - the user has sent too many requests to the api.
- `RequestFailed(reqwest::Error)` - an internal http client error occurred.
- `InternalError(String)` - an error with luduvo servers occurred.

---

## Friend

represents a single friend.

```rust
pub struct Friend {
    pub user_id: u64,
    pub username: String,
}
```

### fields

- `user_id` - a `u64` detailing the user id of the friend.
- `username` - a `String` detailing the username of the friend.

---

## Friends

represents the list of a user’s friends returned by the luduvo api.

```rust
pub struct Friends {
    pub friends: Vec<Friend>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}
```

### fields

- `friends` - a `Vec<Friend>` of the friends the user has.
- `total` - a `u64` detailing the total amounts of friends the user has.
- `limit` - a `u64` detailing the total amount of friends the user can have at a time.
- `offset` - a `u64` detailing the current page of friends returned.

---

## CacheEntry

a cached friends entry, containing the user’s friends data and its last updated timestamp.

```rust
pub struct CacheEntry {
    pub result: Friends,
    pub last_updated: u64,
}
```

### fields

- `result` - a `Friends` struct with the cached friends.
- `last_updated` - a `u64` with the unix timestamp of when it was last updated.

---

## Cache

a cache of user friends data, keyed by user id.

this is used internally by `Client` to cache friends.

```rust
pub struct Cache {
    /* private fields */
}
```

### methods

#### new
```rust
pub fn new(cache_timeout: u64) -> Cache
```
creates a new `Cache` with the specified cache timeout.

##### arguments
  - `cache_timeout` - a `u64` detailing the cache timeout in seconds.

##### returns
  - a new `Cache` instance

#### get
```rust
pub fn get(&mut self, id: u64) -> Option<Friends>
```

retrieves a result from the cache by its id.

##### arguments
  - `id` - a `u64` detailing the id of the result to retrieve.

##### returns
  - the result if it is still valid and not expired
  - a `None` if the result is expired or missing

#### insert
```rust
pub fn insert(&mut self, id: u64, result: Friends)
```

inserts a result into the cache.

##### arguments
  - `id` - a `u64` detailing the id of the result to insert.
  - `result` - a `Friends` detailing the result to insert.

#### remove
```rust
pub fn remove(&mut self, id: u64)
```

removes a result from the cache by its id.

##### arguments
  - `id` - the id of the result to remove.
