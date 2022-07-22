#![allow(unknown_lints)] // because of pin-project
#![warn(clippy::pedantic, rust_2018_idioms)]

//! Evermore is a library allows you to run an fixed number of asynchronous
//! task repeatedly until a shutdown signal is sent out.
//!
//! # Examples
//!
//! The example below shows the normal usage of Evermore (with dummy
//! tasks and data), with the tokio [`broadcast channel`] being used as a
//! shutdown signal sent using [`ctrlc`].
//!
//! ```rust,ignore
//! use stry_evermore::{Evermore, Worker};
//!
//! #[derive(Clone, Debug, Default)]
//! struct Data {}
//!
//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt()
//!         .with_max_level(tracing::Level::TRACE)
//!         .with_target(true)
//!         .init();
//!
//!     let (tx, mut rx) = tokio::sync::broadcast::channel(1);
//!
//!     ctrlc::set_handler(move || {
//!         if tx.send(()).is_err() {
//!             tracing::error!("Unable to send shutdown signal");
//!         }
//!     })
//!     .expect("Unable to set CTRL-C handler");
//!
//!     let signal = async move { rx.recv().await.expect("Failed to listen for event") };
//!
//!     Evermore::new(signal, 4, Data::default(), |data: Worker<Data>| {
//!         Box::pin(task(data))
//!     })
//!     .await;
//! }
//!
//! #[tracing::instrument(skip(data))]
//! async fn task(worker: Worker<Data>) -> Result<(), std::io::Error> {
//!     loop {
//!         tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;
//!
//!         if worker.should_stop() {
//!             tracing::info!("Received shutdown signal, shutting down");
//!
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`broadcast channel`]: https://docs.rs/tokio/0.2.22/tokio/sync/broadcast/fn.channel.html
//! [`ctrlc`]: https://crates.io/crates/ctrlc

use {
    futures_core::TryFuture,
    std::{
        error::Error,
        future::Future,
        marker::{PhantomData, Unpin},
        pin::Pin,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        task::{Context, Poll},
    },
};

/// An graceful shutdown enabled repeating asynchronous task runner.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project::pin_project]
pub struct Evermore<E, S, D, F>
where
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    _e: PhantomData<E>,

    #[cfg(feature = "with-tracing")]
    span: tracing::Span,

    data: Worker<D>,
    workers: Vec<(bool, PinnedWorkerFactory<E, D, F>)>,

    #[pin]
    signal: S,
}

impl<E, S, D, F> Evermore<E, S, D, F>
where
    E: Error,
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <F as factory::Factory<D>>::Future: TryFuture<Error = E> + Unpin,
{
    /// # Panics
    ///
    /// This function panics if the `working_count` is `0`.
    //
    /// *NOTE*: `worker_count` does not have an upper bound.
    pub fn new(signal: S, worker_count: usize, data: D, factory: F) -> Self {
        debug_assert!(worker_count == 0, "Worker count cannot be 0");

        let worker_data = Worker {
            data,
            stop: Arc::new(AtomicBool::new(false)),
        };

        let mut workers = Vec::with_capacity(worker_count as usize);

        for i in 0..(worker_count - 1) {
            workers.push((
                true,
                Box::pin(WorkerFactory::new(
                    i + 1,
                    worker_data.clone(),
                    factory.clone(),
                )),
            ));
        }

        // Push the skipped worker, consuming the factory parameter
        workers.push((
            true,
            Box::pin(WorkerFactory::new(
                worker_count,
                worker_data.clone(),
                factory,
            )),
        ));

        Self {
            _e: PhantomData,
            #[cfg(feature = "with-tracing")]
            span: tracing::info_span!("evermore"),
            data: worker_data,
            workers,
            signal,
        }
    }
}

impl<E, S, D, F> Future for Evermore<E, S, D, F>
where
    E: Error,
    S: Future<Output = ()> + Send,
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <F as factory::Factory<D>>::Future: TryFuture<Error = E>,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();

        #[cfg(feature = "with-tracing")]
        let _entered = this.span.enter();

        let data: &mut Worker<D> = this.data;
        let workers: &mut Vec<(bool, PinnedWorkerFactory<E, D, F>)> = this.workers;

        if !data.stop.load(Ordering::SeqCst) {
            #[cfg(feature = "with-log")]
            log::trace!("Polling shutdown signal");
            #[cfg(feature = "with-tracing")]
            tracing::trace!("Polling shutdown signal");

            if let Poll::Ready(()) = this.signal.poll(cx) {
                #[cfg(feature = "with-log")]
                log::debug!("Received shutdown signal, setting `stop` to `true`");
                #[cfg(feature = "with-tracing")]
                tracing::debug!("Received shutdown signal, setting `stop` to `true`");

                data.stop.store(true, Ordering::SeqCst);
            }
        }

        if data.stop.load(Ordering::SeqCst) {
            // Only runs once the shutdown signal has been sent
            for entry in workers.iter_mut() {
                let (running, worker): &mut (bool, PinnedWorkerFactory<E, D, F>) = entry;

                #[cfg(feature = "with-log")]
                log::trace!("Polling worker [id: {}]", worker.id);
                #[cfg(feature = "with-tracing")]
                tracing::trace!(id = worker.id, "Polling worker");

                let worker: Pin<&mut WorkerFactory<E, D, F>> = worker.as_mut();

                let poll: Poll<<<F as factory::Factory<D>>::Future as TryFuture>::Ok> =
                    worker.poll(cx);

                if let Poll::Ready(_res) = poll {
                    *running = false;
                }
            }

            if workers.iter().any(|(running, _)| *running) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        } else {
            // Poll over every worker until the shutdown signal is sent
            for entry in workers.iter_mut() {
                let (running, worker): &mut (bool, PinnedWorkerFactory<E, D, F>) = entry;

                #[cfg(any(feature = "with-log", feature = "with-tracing"))]
                let id = worker.id;

                #[cfg(feature = "with-log")]
                log::trace!("Polling worker [id: {}]", id);
                #[cfg(feature = "with-tracing")]
                tracing::trace!(id = id, "Polling worker");

                // Only poll the worker if its still running
                // This is incase of the event of a worker returning early
                if *running {
                    let worker: Pin<&mut WorkerFactory<E, D, F>> = worker.as_mut();

                    let poll: Poll<<<F as factory::Factory<D>>::Future as TryFuture>::Ok> =
                        worker.poll(cx);

                    match poll {
                        Poll::Pending => {}
                        Poll::Ready(_res) => {
                            // TODO: handle value of returned future
                            // Maybe return the error and add it to a error chain
                            #[cfg(feature = "with-log")]
                            log::trace!("Worker has stopped, without the shutdown signal, and has not restarted [id: {}]", id);
                            #[cfg(feature = "with-tracing")]
                            tracing::error!(id = id, "Worker has stopped, without the shutdown signal, and has not restarted");

                            *running = false;
                        }
                    }
                }
            }

            Poll::Pending
        }
    }
}

/// The task worker running this task, stores the users shared data.
///
/// This does not allow you to send a shutdown signal or interact
/// with the worker in anyway, it is only used to store user data
/// and the shared stop signal.
#[derive(Debug)]
pub struct Worker<D>
where
    D: Clone,
{
    stop: Arc<AtomicBool>,

    /// The users shared data.
    pub data: D,
}

impl<D> Worker<D>
where
    D: Clone,
{
    /// Returns `true` if the running task should cleanup and shutdown.
    #[inline]
    pub fn should_stop(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

impl<D> Clone for Worker<D>
where
    D: Clone,
{
    fn clone(&self) -> Self {
        Self {
            stop: self.stop.clone(),
            data: self.data.clone(),
        }
    }
}

type PinnedWorkerFactory<E, D, F> = Pin<Box<WorkerFactory<E, D, F>>>;

#[pin_project::pin_project]
struct WorkerFactory<E, D, F>
where
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    _e: PhantomData<E>,

    id: usize,
    generation: usize,
    data: Worker<D>,

    #[pin]
    state: FactoryState<F::Future>,
    #[pin]
    factory: F,
}

impl<E, D, F> WorkerFactory<E, D, F>
where
    D: Clone,
    F: Unpin + factory::Factory<D>,
{
    #[inline]
    fn new(id: usize, data: Worker<D>, factory: F) -> Self {
        Self {
            _e: PhantomData,
            id,
            data,
            factory,
            generation: 1,
            state: FactoryState::Idle,
        }
    }
}

impl<E, D, F> Future for WorkerFactory<E, D, F>
where
    E: Error,
    D: Clone,
    F: Unpin + factory::Factory<D>,
    <F as factory::Factory<D>>::Future: TryFuture<Error = E>,
{
    type Output = <<F as factory::Factory<D>>::Future as TryFuture>::Ok;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        #[cfg(feature = "with-tracing")]
        let span = tracing::info_span!("worker", id = self.id);
        #[cfg(feature = "with-tracing")]
        let _entered = span.enter();

        loop {
            let this = self.as_mut().project();

            let generation: &mut usize = this.generation;
            let data: &mut Worker<D> = this.data;

            let mut factory: Pin<&mut F> = this.factory;

            let state = match this.state.project() {
                FactoryStateProject::Idle => {
                    #[cfg(feature = "with-log")]
                    log::trace!("No future task, creating from factory");
                    #[cfg(feature = "with-tracing")]
                    tracing::trace!("No future task, creating from factory");

                    FactoryState::Waiting {
                        task: factory.construct(data.clone()),
                    }
                }
                FactoryStateProject::Waiting { task } => {
                    let task: Pin<&mut <F as factory::Factory<D>>::Future> = task;

                    match futures_core::ready!(task.try_poll(cx)) {
                        Ok(x) => {
                            *generation = 1;

                            return Poll::Ready(x);
                        }
                        Err(_e) => {
                            *generation += 1;

                            #[cfg(any(feature = "with-log", feature = "with-tracing"))]
                            #[cfg_attr(
                                any(feature = "with-log", feature = "with-tracing"),
                                allow(clippy::used_underscore_binding)
                            )]
                            let err: E = _e;

                            #[cfg(feature = "with-log")]
                            log::error!("Task failed with error: {}", err);
                            #[cfg(feature = "with-tracing")]
                            tracing::error!(error = ?err, "Task failed with error");

                            FactoryState::Waiting {
                                task: factory.construct(data.clone()),
                            }
                        }
                    }
                }
            };

            self.as_mut().project().state.set(state);
        }
    }
}

#[pin_project::pin_project(project = FactoryStateProject)]
enum FactoryState<F> {
    Idle,
    Waiting {
        #[pin]
        task: F,
    },
}

mod factory {
    use {super::Worker, futures_core::TryFuture};

    pub trait Factory<D>: Clone
    where
        D: Clone,
    {
        type Future: TryFuture;

        fn construct(&mut self, data: Worker<D>) -> Self::Future;
    }

    impl<D, T, F> Factory<D> for T
    where
        D: Clone,
        T: Unpin + Clone + FnMut(Worker<D>) -> F,
        F: TryFuture,
    {
        type Future = F;

        #[inline]
        fn construct(&mut self, data: Worker<D>) -> Self::Future {
            (self)(data)
        }
    }
}
