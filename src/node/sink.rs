use crate::error::Error;
use crate::frame::MediaFrame;
use crate::{Poll, StartSend};

pub trait MediaSink {
    fn start_send(&mut self, frame: MediaFrame) -> StartSend<MediaFrame, Error>;
    fn poll_complete(&mut self) -> Poll<(), Error>;
    fn close(&mut self) -> Poll<(), Error>;
}
