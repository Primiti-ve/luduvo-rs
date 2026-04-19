---
icon: lucide/arrow-left
---

# prelude

this module re-exports commonly used structs in luduvo-rs.

## why use the prelude?

instead of importing individual types/structs like:

```rust
use luduvo_rs::users::profile::{Profile, Client};
use luduvo_rs::users::friends::{Friends, Client};
use luduvo_rs::places::{Places, Client};
```

you can simply do:

```rust
use luduvo_rs::prelude::*;
```

this is especially useful in small scripts, examples, or when you are using multiple parts of the crate at once.

## re-exported items

### profile api

- `Profile`
- `Client`
- `Error`

### friends api

- `Friends`
- `Client`
- `Error`

### query api

- `Query`
- `Client`
- `Error`
