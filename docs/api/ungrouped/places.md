---
icon: lucide/map
---

# places

this module contains structs related to fetching luduvo place data.

_this is for searching places by name._ there is currently no known endpoint for searching by id.

---

## PlacesWrapper

a client for interacting with the luduvo places api.

__the configuration and client aren't publicly exposed through the struct!__

```rust
pub struct PlacesWrapper {
    /* private fields */
}
```

### methods

#### new

creates a new `PlacesWrapper`.

```rust
pub fn new(config: Option<PlacesConfig>) -> PlacesWrapper
```

##### arguments

- `config` - an _optional_ `PlacesConfig` to use.

##### returns

- if successful, a `PlacesWrapper` instance.

#### get_places

fetches a list of places by name.

```rust
pub async fn get_places(
    &mut self,
    query: String,
    limit: Option<String>,
) -> Result<Places, PlacesError>
```

##### arguments

- `query` - a `String` detailing the search query.
- `limit` - an _optional_ `String` detailing the maximum number of places to fetch. by default, this is `20`.

##### returns

- `PlacesError::TooManyRequests` if the user has sent too many requests within a short timespan.
- `PlacesError::RequestFailed` for network or decoding errors.
- `PlacesError::InternalError` if something went wrong within the luduvo api.
- if successful, a `Places` instance.

---

## PlacesConfig

the configuration for the `PlacesWrapper` struct.

__no configuration is publicly exposed through the struct!__

```rust
pub struct PlacesConfig {
    /* private fields */
}
```

### arguments

- `client` - a `reqwest::Client` detailing the http client to use for requests.
- `base_url` - a `String` detailing the base url for the api. by default, this is set to `https://api.luduvo.com/places`.
- `cache_timeout` - a `u64` detailing the time it takes for a cached entry to invalidate. by default, this is set to `30` seconds.

### methods

#### new

```rust
pub fn new(
    client: Option<Client>,
    base_url: Option<String>,
    cache_timeout: Option<u64>,
) -> PlacesConfig
```

##### arguments

- `client` - an _optional_ `reqwest::Client` detailing the http client to use for requests.
- `base_url` - an _optional_ `String` detailing the base url for the api.
- `cache_timeout` - an _optional_ `u64` detailing the time it takes for a cached entry to invalidate.

#### default

```rust
pub fn default() -> PlacesConfig
```

---

## PlacesError

the list of errors that can occur when fetching places.

```rust
pub enum PlacesError {
    TooManyRequests(),
    RequestFailed(Error),
    InternalError(String),
}```

### variants

- `TooManyRequests()` - the user has sent too many requests to the api.
- `RequestFailed(reqwest::Error)` - an internal http client error occurred.
- `InternalError(String)` - an error with luduvo servers occurred.

---

## Place

represents a single place.

```rust
pub struct Place {
    pub id: u64,
    pub owner_id: u64,
    pub owner_username: String,
    pub title: String,
    pub description: String,
    pub access: String,
    pub max_players: u64,
    pub visit_count: u64,
    pub thumbs_up: u64,
    pub thumbs_down: u64,
    pub active_players: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub thumbnail_url: String,
}
```

### fields

- `id` - a `u64` detailing the unique id of the place.
- `owner_id` - a `u64` detailing the id of the place owner.
- `owner_username` - a `String` detailing the username of the place owner.
- `title` - a `String` detailing the title of the place.
- `description` - a `String` detailing the description of the place.
- `access` - a `String` detailing the access level of the place (e.g. public/private).
- `max_players` - a `u64` detailing the maximum number of players allowed.
- `visit_count` - a `u64` detailing the total number of visits.
- `thumbs_up` - a `u64` detailing the number of positive ratings.
- `thumbs_down` - a `u64` detailing the number of negative ratings.
- `active_players` - a `u64` detailing the number of currently active players.
- `created_at` - a `u64` detailing the creation timestamp in unix seconds.
- `updated_at` - a `u64` detailing the last updated timestamp in unix seconds.
- `thumbnail_url` - a `String` detailing the url for the place thumbnail.

---

## Places

represents the list of places returned by the luduvo api.

```rust
pub struct Places {
    pub places: Vec<Place>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}
```

### fields

- `places` - a `Vec<Place>` detailing the list of returned places.
- `total` - a `u64` detailing the total amount of places available.
- `limit` - a `u64` detailing the number of places requested.
- `offset` - a `u64` detailing the current page offset.

---

## CachedPlaces

a cached places entry, containing a `Places` struct and its last updated timestamp.

this is used internally by `PlacesCache` to store place data.

### fields

- `places` - the `Places` that is being cached.
- `last_updated` - a `u64` that shows the last time the cache has been updated. this is a unix timestamp.

---

## PlacesCache

a cache of places, keyed by query string.

this is used internally by `PlacesWrapper` to cache place data.

### methods

#### new

creates a new `PlacesCache` with the specified cache timeout.

```rust
pub fn new(cache_timeout: u64) -> PlacesCache
```

##### arguments

- `cache_timeout` - a `u64` detailing the cache timeout in seconds.

##### returns

- a new `PlacesCache` instance.

#### get

retrieves a cached entry by its query.

```rust
pub fn get(&mut self, id: &str) -> Option<Places>
```

##### arguments

- `id` - a `&str` detailing the query key to retrieve.
 
##### returns

- the cached `Places` if it is still valid and not expired.
- `None` if the entry is expired or missing.

#### insert

inserts a places entry into the cache.

##### arguments

- `id` - the query key as a `String`.
- `places` - a `Places` detailing the data to cache.

#### remove

removes a cached entry by its query key.

```rust
pub fn remove(&mut self, id: &str)
```

##### arguments

- `id` - the query key to remove.
