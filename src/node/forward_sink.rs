use futures::try_ready;
use crate::error::Error;
use crate::frame::MediaFrame;
use crate::{Async, AsyncSink, Future, Poll};
use super::{MediaSink, MediaSource};

#[derive(Debug)]
pub struct ForwardSink<T: MediaSource, U> {
    source: Option<T>,
    sink: Option<U>,
    buffered: Option<MediaFrame>,
}

impl<T, U> ForwardSink<T, U>
where
    T: MediaSource,
    U: MediaSink,
{
    pub fn new(source: T, sink: U) -> ForwardSink<T, U> {
        ForwardSink {
            source: Some(source),
            sink: Some(sink),
            buffered: None,
        }
    }

    pub fn source_ref(&self) -> Option<&T> {
        self.source.as_ref()
    }

    pub fn source_mut(&mut self) -> Option<&mut T> {
        self.source.as_mut()
    }

    pub fn sink_ref(&self) -> Option<&U> {
        self.sink.as_ref()
    }

    pub fn sink_mut(&mut self) -> Option<&mut U> {
        self.sink.as_mut()
    }

    fn take_result(&mut self) -> (T, U) {
        let sink = self
            .sink
            .take()
            .expect("Attempted to poll ForwardSink after completion");
        let source = self
            .source
            .take()
            .expect("Attempted to poll ForwardSink after completion");
        (source, sink)
    }

    fn try_start_send(&mut self, item: MediaFrame) -> Poll<(), Error> {
        debug_assert!(self.buffered.is_none());
        if let AsyncSink::NotReady(item) = self
            .sink_mut()
            .expect("Attempted to poll ForwardSink after completion")
            .start_send(item)?
        {
            self.buffered = Some(item);
            return Ok(Async::NotReady);
        }
        Ok(Async::Ready(()))
    }
}

impl<T, U> Future for ForwardSink<T, U>
where
    T: MediaSource,
    U: MediaSink,
{
    type Item = (T, U);
    type Error = Error;

    fn poll(&mut self) -> Poll<(T, U), Error> {
        if let Some(item) = self.buffered.take() {
            try_ready!(self.try_start_send(item))
        }

        loop {
            match self
                .source_mut()
                .expect("Attempted to poll ForwardSink after completion")
                .poll()?
            {
                Async::Ready(Some(item)) => try_ready!(self.try_start_send(item)),
                Async::Ready(None) => {
                    try_ready!(self
                        .sink_mut()
                        .expect("Attempted to poll ForwardSink after completion")
                        .close());
                    return Ok(Async::Ready(self.take_result()));
                }
                Async::NotReady => {
                    try_ready!(self
                        .sink_mut()
                        .expect("Attempted to poll ForwardSink after completion")
                        .poll_complete());
                    return Ok(Async::NotReady);
                }
            }
        }
    }
}
