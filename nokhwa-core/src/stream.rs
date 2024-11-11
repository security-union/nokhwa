use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    sync::mpsc::Receiver
};
use futures::Stream as AsyncStreamTrait;
use crate::{
    buffer::Buffer,
    error::NokhwaError
};

#[derive(Clone, Debug)]
pub enum ChannelState {
    Frame(Buffer),
    Error(NokhwaError),
    ClosedWithError(NokhwaError),
    Closed,
}

pub enum StreamType {
    Channel(Arc<Receiver<ChannelState>>),
    Callback(Arc<Mutex<Option<ChannelState>>>),
}

pub struct CaptureStream {}

impl Future for CaptureStream {
    type Output = ChannelState;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

impl AsyncStreamTrait for CaptureStream {
    type Item = ChannelState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}