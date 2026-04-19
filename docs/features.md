---
icon: lucide/hammer
---

# features

!!! tip
    most users will want to import the prelude, via `luduvo_rs::prelude::*`

<div class="grid cards" markdown>
- __user profile data__ (search by id, one result)
- __user friends data__ (search by id, multiple results)
- __user querying__ (search by username, multiple results)
- __places data__ (search by name, multiple results)
</div>

## todo

- [x] users api
    * [x] profile endpoint
    * [x] friends endpoint
    * [x] query endpoint
- [x] places api
- [ ] groups api (awaiting sample data)
- [ ] tags api (awaiting sample data)
- [ ] future stuff

## feature flags

luduvo-rs comes with _feature flags_ so you can pick and choose what features you'd like to use. these features are:

- `users` - which enables `friends`, `profile`, and `query`
- `prelude` - which enables the `luduvo_rs::prelude` exports
- `friends` - which enables the `luduvo_rs::users::friends` exports
- `profile` - which enables the `luduvo_rs::users::profile` exports
- `query` - which enables the `luduvo_rs::users::query` exports
- `places` - which enables the `luduvo_rs::places` exports

all features are enabled by default!
