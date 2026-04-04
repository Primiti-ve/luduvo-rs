<img src="https://github.com/Primiti-ve/luduvo-rs/blob/main/gh-assets/logo.png?raw=true" alt="luduvo-rs" height=256/>

### *luduvo-rs* is a rust library designed for interacting with the [luduvo](luduvo.com) api.

## links

- [crates.io](https://crates.io/crates/luduvo-rs)
- [docs.rs](https://docs.rs/luduvo-rs/latest/luduvo_rs)
- [luduvo development hub](https://discord.gg/FcjTvuWKRk)

## notes

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it!
> all contributions (pull requests, issues) are welcomed.

> [!IMPORTANT]
> you need an asynchronous runtime in order to use this. **tokio** is the intended runtime for this crate!

> [!TIP]
> most users will want to import the prelude, via `luduvo_rs::prelude::*`

## features

- user profile data
- user friends data

## quick start

> [!NOTE]
> the code snippet below uses the prelude, which re-exports commonly used structs!

```rust
use luduvo_rs::prelude::*;

#[tokio::main]
async fn main() {
    let mut api = ProfileWrapper::new(None);
    let profile = api.get_profile("1").await.unwrap();

    println!("hello {}!", profile.username);
}
```

## example

> [!NOTE]
> the code snippet below is taken directly from `examples/get_profile.rs`!

```rust
use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);
    let id = "1";

    match wrapper.get_profile(id).await {
        Ok(profile) => {
            println!("profile for id `{id}`: {:#?}", profile);
        }

        Err(e) => {
            eprintln!(
                "error caught while attempting to get profile for id `{id}`: '{:#?}'",
                e
            );
        }
    }
}
```

## contributors

- [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/Primiti-ve/luduvo-rs/pull/1) (Small changes)

## need help?

- contact me on discord! my discord username is `@primiti_ve`.
  - ping me in the [luduvo discord](https://discord.gg/luduvo).
  - join the [luduvo development hub](https://discord.gg/FcjTvuWKRk).
- create an issue (https://github.com/Primiti-ve/luduvo-rs/issues)
