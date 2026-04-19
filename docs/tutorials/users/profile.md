---
icon: lucide/user
---

# profile

these are basic examples for the `Client` struct.

all of the examples below are explained using zensical's code annotations feature.

---

## basic usage

``` rust
use luduvo_rs::users::profile::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new(None); // (1)!
    let profile = client.get_user("1".to_string()).await.unwrap(); // (2)!

    println!("username: {}", profile.username);
    println!("display name: {}", profile.display_name);
}
```

1. this is the struct used to fetch friends data from the luduvo api. the first argument is an _optional_ `Config`.
2. this is an example of how to get friends using `Client.get_user`, which takes in a `String` and returns a `Result<Friends, Error>`.

---

## working with optional fields

``` rust
use luduvo_rs::users::profile::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new(None);
    let profile = client.get_user("1".to_string()).await.unwrap();

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
use luduvo_rs::users::profile::{Client, Config};

#[tokio::main]
async fn main() {
    let config = Config::new(
        None,
        Some("https://api.luduvo.com/users".to_string()),
        Some(120),
    ); // (1)!

    let mut client = Client::new(Some(config)); // (2)!
    let profile = client.get_user("5".to_string()).await.unwrap();

    println!("user id: {}", profile.user_id);
}
```

1. this shows the creation of a `Config` struct. if you want the defaults, you can use `Config::default()`.
2. the first argument of `Client` is an _optional_ `Config`, which means you have to wrap it in `Some`.
