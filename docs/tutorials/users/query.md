---
icon: lucide/users
---

# query

these are basic examples for the `QueryWrapper` struct.

all of the examples below are explained using zensical's code annotations feature.

---

## basic usage

``` rust
use luduvo_rs::users::query::QueryWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = QueryWrapper::new(None); // (1)!
    let result = wrapper.get_user("luduvo".to_string(), None).await.unwrap(); // (2)!

    for user in result.users {
        println!("{} ({})", user.username, user.id);
    }
}
```

1. this is the struct used to fetch multiple users from the luduvo api. the first argument is an _optional_ `QueryConfig`.
2. this is an example of how to get friends using `QueryWrapper.get_user`, which takes in a _required_ username (which is a `String`) and an _optional_ limit (which is a `String`), and returns a `Result<Query, QueryError>`.

---

## using a custom limit

``` rust
use luduvo_rs::users::query::QueryWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = QueryWrapper::new(None);

    let result = wrapper
        .get_user("dev".to_string(), Some("5".to_string())) // (1)!
        .await
        .unwrap();

    println!("fetched {} users", result.users.len());
}
```

1. this example would return a list with 5 __(or less)__ users with the string "dev" in their username.

---

## cache behaviour

``` rust
use luduvo_rs::users::query::QueryWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = QueryWrapper::new(None);

    let _ = wrapper.get_user("builder".to_string(), None).await.unwrap(); // (1)!
    let cached = wrapper.get_user("builder".to_string(), None).await.unwrap(); // (2)!

    println!("Cached results: {}", cached.users.len());
}
```

1. the first call goes through to the luduvo api.
2. the second call retrieves the result from the cache, saving an api call.
