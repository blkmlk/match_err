# match_err

Macro for quick matching errors against enum-like error types

Helps to avoid writing long and tedious structures like:
```rust
if let Err(e) = err {
    if let Some(e) = e.downcast_ref::<Error>() {
        match e {
            ...
        }
    }
}
```

## Examples

```rust
use match_err::*;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("not found")]
    NotFound,
    #[error("custom: {0}")]
    Custom(String),
}

let err: Result<(), _> = Err(anyhow!(Error::NotFound));

match_if_err!(err, Error, {
    NotFound => println!("not found"),
    Custom(msg) => println!("custom message: {}", msg),
    _ => println!("unknown")
})
```