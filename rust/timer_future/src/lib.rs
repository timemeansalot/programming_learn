use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone()); // set waker
            Poll::Pending // return pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        // init shared state
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None, // initialize waker to None
        }));

        // TODO: what the hell does spawn do?
        // spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true; // set completed to true
            if let Some(waker) = shared_state.waker.take() {
                waker.wake(); // wake the waker
            }
        });

        TimerFuture { shared_state } // return instance
    }
}
