use stry_evermore::{Evermore, Worker};

#[derive(Debug, Default, Clone)]
struct Data {}

fn main() {
    let _ = Evermore::new(async {}, 4, Data::default(), |worker| {
        Box::pin(task(worker))
    });
}

async fn task(worker: Worker<Data>) -> Result<(), std::io::Error> {
    loop {
        if worker.should_stop() {
            break;
        }
    }

    Ok(())
}
