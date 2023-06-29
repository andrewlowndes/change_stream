use change_stream::StreamChanged;
use futures::{stream::iter, StreamExt};
use multi_stream::multi_stream;

#[tokio::main]
async fn main() {
    let a = iter([1, 1, 2]);
    let b = iter([5, 5, 4]);

    //values are only emitted when their differ
    multi_stream!(a.changed(), b.changed())
        .map(|(a, b)| a.unwrap_or_default() + b.unwrap_or_default())
        .changed()
        .for_each(|c| async move {
            dbg!(c); //this is only called where the sum has also changed
        })
        .await;
}
