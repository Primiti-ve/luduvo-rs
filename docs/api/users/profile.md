---
icon: lucide/user
---

# profile

this module contains structs related to fetching a single luduvo user at once.

_this is for searching users by id._ for searching users by username, use a `QueryWrapper` with a limit of `1`.

---

## ProfileWrapper

a client for interacting with the luduvo user profile api.

__the configuration and client aren't publicly exposed through the struct!__

```rust
pub struct ProfileWrapper {
    /* private fields */
}
```

### methods

#### new

creates a new `ProfileWrapper`.

```rust
pub fn new(config: Option<ProfileConfig>) -> ProfileWrapper
```

##### arguments

- `config` - an _optional_ `ProfileConfig` to use.

##### returns

- if successful, a `ProfileWrapper` instance.

#### get_user

```rust
pub async fn get_user(&mut self, id: String) -> Result<Profile, ProfileError>
```

fetches a user profile by id.

##### arguments

- `id` - the user id as a `String`.

##### returns

- `ProfileError::ProfileNotFound` if the profile does not exist (HTTP 404).
- `ProfileError::RequestFailed` for network or decoding errors.
- `ProfileError::InvalidId` if the id is not a valid string.
- `ProfileError::TooManyRequests` if the user has sent too many requests within a short timespan.
- if successful, a `Profile` instance.

---

## ProfileConfig

the configuration for the `ProfileWrapper` struct.

__no configuration is publicly exposed through the struct!__

```rust
pub struct ProfileConfig {
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
) -> ProfileConfig
```

##### arguments

- `client` - an _optional_ `reqwest::Client` detailing the http client to use for requests.
- `base_url` - an _optional_ `String` detailing the base url for the api.
- `cache_timeout` - an _optional_ `u64` detailing the time it takes for a cached entry to invalidate.

#### default

```rust
pub fn default() -> FriendsConfig
```

---

## ProfileError

the list of  errors that can occur when fetching a users profile.

```rust
pub enum ProfileError {
    ProfileNotFound(String),
    InvalidId(String),
    TooManyRequests(),
    RequestFailed(Error),
    InternalError(String),
}
```

### variants

- `ProfileNotFound(String)` - the profile with the specified id was not found.
- `InvalidId(String)` - the id is invalid and not found in luduvo's database.
- `TooManyRequests()` - the user has sent too many requests to the api.
- `RequestFailed(reqwest::Error)` - an internal http client error occurred.
- `InternalError(String)` - an error with luduvo servers occurred.

---

## Profile

```rust
pub struct Profile {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub status: Option<String>,
    pub avatar: ProfileAvatar,
    pub equipped_items: Vec<EquippedItem>,
    pub badges: Vec<Badge>,
    pub friend_count: u64,
    pub place_count: u64,
    pub item_count: u64,
    pub last_active: Option<u64>,
    pub member_since: Option<u64>,
    pub allow_joins: bool,
    pub is_owner: bool,
}
```

### fields

- `user_id` - a `u64` detailing the users id. _this is unique to each profile._
- `username` - a `String` detailing the users username. _this is unique to each profile._
- `display_name` - a `String` detailing the name shown to other users. when the account is first created, this defaults to `username`. it can be changed by the user at any time.
- `bio` - an `Option<String>` detailing an optional long-form description of the profile.
- `status` - an `Option<String>` detailing an optional status code of what the user is currently doing.
- `avatar` - a `ProfileAvatar` detailing the user’s avatar appearance configuration. _currently, it is just hex codes for the avatar’s limbs._
- `equipped_items` - a `Vec<EquippedItem>` detailing a list of the user’s equipped items.
- `badges` - a `Vec<Badge>` detailing a list of badge identifiers earned by the user.
- `friend_count` - a `u64` detailing the total number of friends the user has.
- `place_count` - a `u64` detailing the total number of owned places the user has.
- `item_count` - a `u64` detailing the total number of owned items the user has.
- `last_active` - an `Option<u64>` detailing the last active timestamp of the user in unix seconds. if the user has never logged in, this is a `None`.
- `member_since` - an `Option<u64>` detailing the users account creation timestamp in unix seconds.
- `allow_joins` - a `bool` detailing whether others are allowed to join this user.
- `is_owner` - a `bool` detailing whether the current viewer owns the resource being viewed.

---

## ProfileAvatar

represents the color configuration of a user’s avatar.

all fields are hex color strings. by default, they are all set to `#C8C8C8`.

```rust
pub struct ProfileAvatar {
    pub head_color: String,
    pub torso_color: String,
    pub left_arm_color: String,
    pub right_arm_color: String,
    pub left_leg_color: String,
    pub right_leg_color: String,
}
```

### fields

- `head_color` - a `String` detailing the user's head colour.
- `torso_color` - a `String` detailing the user's torso colour.
- `left_arm_color` - a `String` detailing the user's left arm colour.
- `right_arm_color` - a `String` detailing the user's right arm colour.
- `left_leg_color` - a `String` detailing the user's left leg colour.
- `right_leg_color` - a `String` detailing the user's right leg colour.

---

## EquippedItem

represents a user’s equipped item.

__this will be updated eventually!__

```rust
pub struct EquippedItem {}
```

---

## Badge

represents a user’s obtained badge.

```rust
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon_url: String,
}
```

### fields

- `id` - a `u64` detailing the id of the badge.
- `name` - a `String` detailing the internally stored name of the badge.
- `slug` - a `String` detailing a human-readable form of the `name`.
- `description` - a `String` detailing the description of the badge.
- `icon_url` - a `String` detailing the url for the shown icon of the badge.

---

## CachedProfile

a cached profile entry, containing a profile and its last updated timestamp.

this is used internally by `ProfileCache` to store profile data.

### fields

- `profile` - the `Profile` that is being cached.
- `last_updated` - a `u64` that shows the last time the cache has been updated. this is a unix timestamp.

---

## ProfileCache

a cache of user profiles, keyed by user id.

this is used internally by `ProfileWrapper` to cache profiles.

```rust
pub struct ProfileCache {
    /* private fields */
}
```

### methods

#### new
```rust
pub fn new(cache_timeout: u64) -> ProfileCache
```
creates a new `ProfileCache` with the specified cache timeout.

##### arguments
  - `cache_timeout` - a `u64` detailing the cache timeout in seconds.

##### returns
  - a new `ProfileCache` instance

#### get
```rust
pub fn get(&mut self, id: u64) -> Option<Friends>
```

retrieves a profile from the cache by its id.

##### arguments
  - `id` - a `u64` detailing the id of the profile to retrieve.

##### returns
  - the profile if it is still valid and not expired
  - a `None` if the profile is expired or missing

#### insert
```rust
pub fn insert(&mut self, id: u64, profile: Profile)
```

inserts a profile into the cache.

##### arguments
  - `profile` - a `Profile` detailing the profile to insert.

#### remove
```rust
pub fn remove(&mut self, id: u64)
```

removes a profile from the cache by its id.

##### arguments
  - `id` - the id of the profile to remove.
