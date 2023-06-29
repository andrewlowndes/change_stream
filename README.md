# Change stream
A Stream that only emits if the value is different.

## Important
- The stream item is stored in the struct by Cloning so the stream Item must implement `Clone`
- The comparison for the items is done via an equality check so the Item must implement `PartialEq` with itself

## Example usage
- Add to your Cargo.toml file
```toml
[dependencies]
change_stream = "0.1.0"
```
- Import the `StreamChanged` trait and call `.changed()` on an existing stream:
```rust
use change_stream::StreamChanged;
use futures::{stream::iter, StreamExt};

#[tokio::main]
async fn main() {
    let a = iter([1, 1, 2]);

    a.changed().for_each(|a| async move {
        dbg!(a); //will only emit 1 and 2
    }).await;
}
```
