---
icon: lucide/handshake
---

# friends

these are basic examples for the `FriendsWrapper` struct.

all of the examples below are explained using zensical's code annotations feature.

---

## basic usage

``` rust
use luduvo_rs::users::friends::FriendsWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = FriendsWrapper::new(None); // (1)!
    let friends = wrapper.get_friends("42".to_string()).await.unwrap(); // (2)!

    println!("total friends: {}", friends.total);

    for friend in friends.friends {
        println!("{} ({})", friend.username, friend.user_id);
    }
}
```

1. this is the struct used to fetch friends data from the luduvo api. the first argument is an _optional_ `FriendsConfig`.
2. this is an example of how to get friends using `FriendsWrapper.get_friends`, which takes in a `String` and returns a `Result<Friends, FriendsError>`.

---

## error handling

``` rust
use luduvo_rs::users::friends::{FriendsWrapper, FriendsError};

#[tokio::main]
async fn main() {
    let mut wrapper = FriendsWrapper::new(None);

    match wrapper.get_friends("invalid_id".to_string()).await { // (1)!
        Ok(_) => println!("unexpected success"),

        Err(FriendsError::InvalidId(id)) => { // (2)!
            eprintln!("invalid id provided: {}", id);
        }

        Err(FriendsError::TooManyRequests()) => { // (3)!
            eprintln!("rate limited!! try again later");
        }

        Err(e) => {
            eprintln!("unhandled error: {}", e);
        }
    }
}
```

1. this is an invalid example of how to get friends using `FriendsWrapper.get_friends`. this will __always fail__ and return a `FriendsError`.
2. this error is thrown when the id is not found in the luduvo database.
3. this error is thrown when the luduvo api returns a `429 Too Many Requests` error, indicating the user is ratelimited.

---

## cache behaviour

``` rust
use luduvo_rs::users::friends::FriendsWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = FriendsWrapper::new(None);
    
    let _ = wrapper.get_friends("1".to_string()).await.unwrap();
    let cached = wrapper.get_friends("1".to_string()).await.unwrap(); // (1)!

    println!("cached friends count: {}", cached.total);
}
```

1. this shows the caching in place for the `FriendsWrapper`. the cache is not publicly exposed.
