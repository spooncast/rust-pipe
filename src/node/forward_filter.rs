use crate::error::Error;
use crate::frame::MediaFrame;
use crate::{Async, AsyncSink, Poll};
use super::{MediaFilter, MediaSource};

#[derive(PartialEq, Debug)]
pub struct ForwardFilter<T: MediaSource, U> {
    source: T,
    filter: U,
    buffered: Option<Option<MediaFrame>>,
}

impl<T, U> ForwardFilter<T, U>
where
    T: MediaSource,
    U: MediaFilter,
{
    pub fn new(source: T, filter: U) -> ForwardFilter<T, U> {
        ForwardFilter {
            source: source,
            filter: filter,
            buffered: None,
        }
    }

    fn try_start_send(&mut self, frame: Option<MediaFrame>) -> Result<(), Error> {
        debug_assert!(self.buffered.is_none());
        if let AsyncSink::NotReady(frame) = self.filter.start_send(frame)? {
            self.buffered = Some(frame);
        }
        Ok(())
    }
}

impl<T, U> MediaSource for ForwardFilter<T, U>
where
    T: MediaSource,
    U: MediaFilter,
{
    fn poll(&mut self) -> Poll<Option<MediaFrame>, Error> {
        if let Some(frame) = self.buffered.take() {
            self.try_start_send(frame)?;
        } else {
            match self.source.poll()? {
                Async::Ready(frame) => self.try_start_send(frame)?,
                Async::NotReady => {},
            }
        }
        self.filter.poll_complete()
    }
}
