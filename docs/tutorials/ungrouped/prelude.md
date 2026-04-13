---
icon: lucide/arrow-left
---

# prelude

this module re-exports commonly used structs in luduvo-rs.

## why use the prelude?

instead of importing individual types/structs like:

```rust
use luduvo_rs::users::profile::{Profile, ProfileWrapper};
use luduvo_rs::users::friends::{Friends, FriendsWrapper};
use luduvo_rs::places::{Places, PlacesWrapper};
```

you can simply do:

```rust
use luduvo_rs::prelude::*;
```

this is especially useful in small scripts, examples, or when you are using multiple parts of the crate at once.

## re-exported items

### profile api

- `Profile`
- `ProfileWrapper`
- `ProfileError`

### friends api

- `Friends`
- `FriendsWrapper`
- `FriendsError`

### query api

- `Query`
- `QueryWrapper`
- `QueryError`
