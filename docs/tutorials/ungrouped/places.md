---
icon: lucide/map
---

# places

these are basic examples for the `PlacesWrapper` struct.

all of the examples below are explained using zensical's code annotations feature.

---

## basic usage

``` rust
use luduvo_rs::places::PlacesWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = PlacesWrapper::new(None); // (1)!

    let places = wrapper
        .get_places("obby".to_string(), None) // (2)!
        .await
        .unwrap();

    for place in places.places {
        println!("{} (visits: {})", place.title, place.visit_count);
    }
}
```

1. this is the struct used to fetch multiple places from the luduvo api. the first argument is an _optional_ `PlacesConfig`.
2. this is an example of how to get places using `PlacesWrapper.get_places`, which takes in a _required_ query (which is a `String`) and an _optional_ limit (which is a `String`), and returns a `Result<Places, PlacesError>`.

---

## error handling

``` rust
use luduvo_rs::places::{PlacesWrapper, PlacesError};

#[tokio::main]
async fn main() {
    let mut wrapper = PlacesWrapper::new(None);

    match wrapper.get_places("test".to_string(), None).await { // (1)!
        Ok(result) => println!("found {} places!", result.total),

        Err(PlacesError::TooManyRequests()) => { // (2)!
            eprintln!("rate limited! try again later.");
        }

        Err(e) => {
            eprintln!("unhandled error: {}", e);
        }
    }
}
```

1. this would find 20 __(or less)__ places with the string "test" in their name.
2. this error is thrown when the luduvo api returns a `429 Too Many Requests` error, indicating the user is ratelimited.

---

## inspecting data

``` rust
use luduvo_rs::places::PlacesWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = PlacesWrapper::new(None);

    let places = wrapper
        .get_places("city".to_string(), Some("10".to_string())) // (1)!
        .await
        .unwrap();

    for place in places.places {
        println!(
            "{} by {} ({} players active)",
            place.title, // (2)!
            place.owner_username, // (3)!
            place.active_players // (4)!
        );
    }
}
```

1. this would find 10 __(or less)__ places with the string "city" in their name.
2. this shows the name of the place.
3. this shows the username of the place's owner.
4. this shows the number of people currently playing the place.
