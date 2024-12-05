use crate::error::{NokhwaError, NokhwaResult};
use crate::frame_buffer::FrameBuffer;
use flume::Receiver;
use std::sync::Arc;

pub trait StreamInnerTrait {
    fn receiver(&self) -> Arc<Receiver<FrameBuffer>>;
    fn stop(&mut self) -> NokhwaResult<()>;
}

pub struct Stream {
    inner: Box<dyn StreamInnerTrait>,
}

impl Stream {
    pub fn new(inner: Box<dyn StreamInnerTrait>) -> Self {
        Self {
            inner,
        }
    }

    // pub unsafe fn erase_lifetime(self) -> Stream<'static> {
    //     Self {
    //         inner: self.inner,
    //         phantom_data: Default::default(),
    //     }
    // }

    pub fn poll_frame(&self) -> NokhwaResult<FrameBuffer> {
        if self.inner.receiver().is_disconnected() {
            return Err(NokhwaError::ReadFrameError(
                "stream is disconnected!".to_string(),
            ));
        }

        self.inner
            .receiver()
            .recv()
            .map_err(|why| NokhwaError::ReadFrameError(why.to_string()))
    }

    pub fn try_poll_frame(&self) -> Option<NokhwaResult<FrameBuffer>> {
        if self.inner.receiver().is_disconnected() {
            return Some(Err(NokhwaError::ReadFrameError(
                "stream is disconnected!".to_string(),
            )));
        }

        if self.inner.receiver().is_empty() {
            return None;
        }

        Some(
            self.inner
                .receiver()
                .try_recv()
                .map_err(|why| NokhwaError::ReadFrameError(why.to_string())),
        )
    }

    #[cfg(feature = "async")]
    pub async fn await_frame(&self) -> NokhwaResult<FrameBuffer> {
        use futures::TryFutureExt;

        if self.inner.receiver().is_disconnected() {
            return Err(NokhwaError::ReadFrameError(
                "stream is disconnected!".to_string(),
            ));
        }

        self.inner
            .receiver()
            .recv_async()
            .map_err(|why| NokhwaError::ReadFrameError(why.to_string())).await
    }

    pub fn stop_stream(mut self) -> NokhwaResult<()> {
        self.inner.stop()?;
        Ok(())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        let _ = self.inner.stop();
    }
}
