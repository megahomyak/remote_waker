use std::{
    future::Future,
    sync::{Arc, Mutex, Weak},
    task::Poll,
};

#[derive(Debug)]
pub enum SnoozingError {
    WakerIsDead,
}

struct Context {
    sleeping: bool,
    waker: Option<std::task::Waker>,
}

pub struct Snoozer {
    context: Weak<Mutex<Context>>,
}

pub struct Waiter<'a> {
    snoozer: &'a mut Snoozer,
}

impl Snoozer {
    pub fn snooze(&mut self) -> Result<Waiter, SnoozingError> {
        let context = self.context.upgrade().ok_or(SnoozingError::WakerIsDead)?;
        context.lock().unwrap().sleeping = true;
        Ok(Waiter { snoozer: self })
    }
}

impl Future for Waiter<'_> {
    type Output = Result<(), SnoozingError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let context = self
            .snoozer
            .context
            .upgrade()
            .ok_or(SnoozingError::WakerIsDead)?;
        let mut context = context.lock().unwrap();
        context.waker = Some(cx.waker().clone());
        if context.sleeping {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

pub struct Waker {
    context: Arc<Mutex<Context>>,
}

impl Waker {
    pub fn wake(&self) {
        let mut context = self.context.lock().unwrap();
        context.waker.take().map(|waker| waker.wake());
        context.sleeping = false;
    }
}

pub fn new() -> (Waker, Snoozer) {
    let task_waker = Arc::new(Mutex::new(Context {
        waker: None,
        sleeping: false,
    }));
    let snoozer = Snoozer {
        context: Arc::downgrade(&task_waker),
    };
    let waker = Waker { context: task_waker };
    (waker, snoozer)
}
