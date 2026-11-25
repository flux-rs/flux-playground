use std::sync::Arc;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Bounds the number of flux processes running at once, and the number of requests waiting for a
/// slot. Requests beyond that are rejected right away instead of piling up.
pub struct Limiter {
    /// Requests running or waiting: `max_running + max_queued` permits.
    admitted: Arc<Semaphore>,
    /// Requests running flux: `max_running` permits.
    running: Arc<Semaphore>,
}

/// Held while a request runs flux; releases its slot when dropped (including when the request is
/// cancelled while waiting).
pub struct Permit {
    _admitted: OwnedSemaphorePermit,
    _running: OwnedSemaphorePermit,
}

impl Limiter {
    pub fn new(max_running: usize, max_queued: usize) -> Self {
        Limiter {
            admitted: Arc::new(Semaphore::new(max_running + max_queued)),
            running: Arc::new(Semaphore::new(max_running)),
        }
    }

    /// Waits for a slot to run flux. Returns `None` without waiting if the queue is already full.
    pub async fn acquire(&self) -> Option<Permit> {
        let admitted = self.admitted.clone().try_acquire_owned().ok()?;
        let running = self
            .running
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore is never closed");
        Some(Permit {
            _admitted: admitted,
            _running: running,
        })
    }
}
