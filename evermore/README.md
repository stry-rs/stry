# (stry) evermore

Evermore is a library allows you to run an fixed number of asynchronous task repeatedly until a shutdown signal is sent out.

## Examples

Usage of Evermore with Tokio, ctrlc and tracing.

```rust
use stry_evermore::{Evermore, Worker};

#[derive(Clone, Debug, Default)]
struct Data {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(true)
        .init();

    let (tx, mut rx) = tokio::sync::broadcast::channel(1);

    ctrlc::set_handler(move || {
        if tx.send(()).is_err() {
            tracing::error!("Unable to send shutdown signal");
        }
    })
    .expect("Unable to set CTRL-C handler");

    let signal = async move { rx.recv().await.expect("Failed to listen for event") };

    Evermore::new(signal, 4, Data::default(), |data: Worker<Data>| {
        Box::pin(task(data))
    })
    .await;
}

#[tracing::instrument(skip(data))]
async fn task(worker: Worker<Data>) -> Result<(), std::io::Error> {
    loop {
        tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;

        if worker.should_stop() {
            tracing::info!("Received shutdown signal, shutting down");

            break;
        }
    }

    Ok(())
}
```