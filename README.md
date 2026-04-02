# luduvo-rs

![Crates.io License](https://img.shields.io/crates/l/luduvo-rs)
![Crates.io Version](https://img.shields.io/crates/v/luduvo-rs)
![GitHub Repo stars](https://img.shields.io/github/stars/Primiti-ve/luduvo-rs)

*luduvo-rs* is a rust library for interacting with the [luduvo](luduvo.com) api.

## disclaimers

- this crate is a completely fanmade wrapper around the luduvo api.
- i'm not good at rust; all contributions (pull requests, issues) are welcomed!

## features

- user profiles (friends coming soon:tm:)

## example

```rust
use luduvo_rs::users::profile::ProfileWrapper;

fn main() {
    let mut wrapper = ProfileWrapper::new(None);

    match wrapper.get_profile("1") {
        Ok(profile) => {
            println!("{:#?}", profile);
        },
        
        Err(e) => {
            eprintln!("error caught while attempting to get profile: '{}'", e);
        },
    }
}
```

## need help?

ping me in the [luduvo discord](https://discord.gg/luduvo) (my discord username is @primiti_ve) or [create an issue](https://github.com/Primiti-ve/luduvo-rs/issues)!