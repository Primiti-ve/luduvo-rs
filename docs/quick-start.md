---
icon: lucide/rocket
---

# quick start

thank you for showing interest in luduvo-rs! :heart:

but before we get started:

!!! danger
    the state of luduvo-rs is incredibly unstable right now. this is to be expected, and will gradually stabilise with a post-1.0.0 release!
    
    unless you're __really__ willing to withstand breaking changes, do not use luduvo-rs. you will encounter:
    
    - missing/incomplete features
    - changes in how features work
    - features being removed

but with all of that out the way, you're now good to go!

## example code

this code showcases the `ProfileWrapper`, a quick and easy way to get user data from id.

```rust
use luduvo_rs::prelude::*;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None); // (1)!

    let id = "1".to_string(); // (2)!
    let profile = wrapper.get_user(id).await.unwrap(); // (3)!

    println!("hello, {}!", profile.username); // (4)!
}
```

1. this creates the `ProfileWrapper`. the first argument is an _optional_ `ProfileConfig` instance, which gets set to the default if it isn't supplied.
2. `ProfileWrapper.get_user` expects a `String` as a first argument. because `"1"` by itself is a `&str` and not a `String`, we have to manually convert it via `.to_string()`!
3. `ProfileWrapper.get_user` is asynchronous, and returns a `Result<Profile, ProfileError>`. this is why we `.unwrap()` it before doing anything with it, so the code panics if something goes wrong.
4. this shows getting data from the profile via `profile.username`, which is a `String`. some fields are optional, so make sure to check the api reference!
