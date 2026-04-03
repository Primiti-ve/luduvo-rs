# luduvo-rs

![docs status](https://img.shields.io/docsrs/luduvo-rs)
![version](https://img.shields.io/crates/v/luduvo-rs)
![license](https://img.shields.io/crates/l/luduvo-rs)

*luduvo-rs* is a rust library for interacting with the [luduvo](luduvo.com) api.

## disclaimers

- this crate is a completely fanmade wrapper around the luduvo api.
- i'm not good at rust; all contributions (pull requests, issues) are welcomed!
- you need an asynchronous runtime in order to use this.
  - tokio is the intended runtime for this crate!

## features

- user profile data
- user friends data

## example

the code snippet below is taken directly from `examples/get_profile.rs`!

```rust
use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);

    match wrapper.get_profile("1").await {
        Ok(profile) => {
            println!("{:#?}", profile);
        }

        Err(e) => {
            eprintln!("error caught while attempting to get profile: '{}'", e);
        }
    }
}
```

## need help?

- ping me on discord! my discord username is `@primiti_ve`.
  - ping me in the [luduvo discord](https://discord.gg/luduvo)
  - join the unofficial [luduvo dev community](https://discord.gg/FcjTvuWKRk)
- create an issue (https://github.com/Primiti-ve/luduvo-rs/issues)
