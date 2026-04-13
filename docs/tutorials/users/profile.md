---
icon: lucide/user
---

# profile

these are basic examples for the `ProfileWrapper` struct.

all of the examples below are explained using zensical's code annotations feature.

---

## basic usage

``` rust
use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None); // (1)!
    let profile = wrapper.get_user("1".to_string()).await.unwrap(); // (2)!

    println!("username: {}", profile.username);
    println!("display name: {}", profile.display_name);
}
```

1. this is the struct used to fetch friends data from the luduvo api. the first argument is an _optional_ `ProfileConfig`.
2. this is an example of how to get friends using `ProfileWrapper.get_user`, which takes in a `String` and returns a `Result<Friends, FriendsError>`.

---

## working with optional fields

``` rust
use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);
    let profile = wrapper.get_user("1".to_string()).await.unwrap();

    match &profile.bio { // (1)!
        Some(bio) => println!("bio: {}", bio),
        None => println!("no bio set!!!!"),
    }

    println!("Last active: {:?}", profile.last_active);
}
```

1. the `bio` field of a `Profile` can be a `None` value if the user has not set it.

---

## custom config

``` rust
use luduvo_rs::users::profile::{ProfileWrapper, ProfileConfig};

#[tokio::main]
async fn main() {
    let config = ProfileConfig::new(
        None,
        Some("https://api.luduvo.com/users".to_string()),
        Some(120),
    ); // (1)!

    let mut wrapper = ProfileWrapper::new(Some(config)); // (2)!
    let profile = wrapper.get_user("5".to_string()).await.unwrap();

    println!("user id: {}", profile.user_id);
}
```

1. this shows the creation of a `ProfileConfig` struct. if you want the defaults, you can use `ProfileConfig::default()`.
2. the first argument of `ProfileWrapper` is an _optional_ `ProfileConfig`, which means you have to wrap it in `Some`.
