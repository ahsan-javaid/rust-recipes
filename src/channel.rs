use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender <T> {
    fn clone(&self) -> Self {
        Sender{
            inner: Arc::clone(&self.inner)
        }
    }
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let mut q = self.inner.queue.lock().unwrap();
        q.push_back(t);
        drop(q);
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> T {
        loop {
            let q = self.inner.queue.lock().unwrap();
            match q.pop_front() {
                Some(t) => return t,
                None => {
                    q= self.available.wait(q).unwrap();,
                }
            }
        }
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

fn channel() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::new(Vec::new()),
        available: Condvar::new()
    };

    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

fn main() {}
