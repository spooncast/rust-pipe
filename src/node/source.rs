use crate::error::Error;
use crate::frame::MediaFrame;
use crate::Poll;

use super::forward_sink::ForwardSink;
use super::forward_filter::ForwardFilter;
use super::filter::MediaFilter;
use super::sink::MediaSink;

pub trait MediaSource {
    fn poll(&mut self) -> Poll<Option<MediaFrame>, Error>;

    fn forward_filter<T>(self, filter: T) -> ForwardFilter<Self, T>
    where
        T: MediaFilter,
        Self: Sized,
    {
        ForwardFilter::new(self, filter)
    }

    fn forward_sink<T>(self, sink: T) -> ForwardSink<Self, T>
    where
        T: MediaSink,
        Self: Sized,
    {
        ForwardSink::new(self, sink)
    }
}
