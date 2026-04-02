# luduvo-rs

*luduvo-rs* is a rust library for interacting with the [luduvo](luduvo.com) api.

## disclaimers

- this crate is a completely fanmade wrapper around the luduvo api.
- i'm not good at rust; all contributions (pull requests, issues) are welcomed!

## need help?

ping me in the [luduvo discord](https://discord.gg/luduvo) (my discord username is @primiti_ve) or [create an issue](https://github.com/Primiti-ve/luduvo-rs/issues)!

## example

```rust
use luduvo_rs::users::profile::ProfileWrapper;

#[tokio::main]
async fn main() {
    let mut wrapper = ProfileWrapper::new(None);
    let id = "1";
    
    let profile = wrapper.get_profile(id).await;

    match profile {
        Ok(profile) => {
            println!("{:#?}", profile);
        },
        
        Err(e) => {
            eprintln!("error caught while attempting to get profile: '{}'", e);
        },
    }
}
```