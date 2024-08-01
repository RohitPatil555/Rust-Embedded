use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Task {
    entry: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(f: impl Future<Output = ()> + 'static) -> Self {
        Task { entry: Box::pin(f) }
    }

    pub fn poll(&mut self, ctx: &mut Context) -> Poll<()> {
        self.entry.as_mut().poll(ctx)
    }
}
