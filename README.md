<img src="https://github.com/Primiti-ve/luduvo-rs/blob/main/gh-assets/wordmark.png?raw=true" alt="luduvo-rs" height=256/>

*luduvo-rs* is a rust library designed for interacting with the [luduvo](luduvo.com) api.

[crates.io](https://crates.io/crates/luduvo-rs) | [docs.rs](https://docs.rs/luduvo-rs/latest/luduvo_rs) | [luduvo development hub](https://discord.gg/FcjTvuWKRk)

---

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 stage.
> 
> expect breaking changes between versions!

> [!TIP]
> most users will want to import the prelude, via `luduvo_rs::prelude::*`

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed.

---

## features

- user profile data (search by id, one result)
- user friends data (search by id, multiple results)
- user querying (search by username, multiple results)
- places data (search by name, multiple results)

## quick start

> [!NOTE]
> the code snippet below uses the prelude, which re-exports commonly used structs!

```rust
use luduvo_rs::prelude::*;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);

    let id = "1".to_string();
    let profile = wrapper.get_user(id).await.unwrap();

    println!("hello, {}!", profile.username);
}
```

## contributors

- [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/Primiti-ve/luduvo-rs/pull/1) (Small changes)

## need help?

- contact me on discord! my discord username is `@primiti_ve`.
  - ping me in the [luduvo discord](https://discord.gg/luduvo).
  - join the [luduvo development hub](https://discord.gg/FcjTvuWKRk).
- [create an issue](https://github.com/Primiti-ve/luduvo-rs/issues).
