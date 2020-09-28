use crate::error::Error;
use crate::frame::MediaFrame;
use crate::{Poll, StartSend};

pub trait MediaFilter {
    fn start_send(&mut self, frame: Option<MediaFrame>) -> StartSend<Option<MediaFrame>, Error>;
    fn poll_complete(&mut self) -> Poll<Option<MediaFrame>, Error>;
}
