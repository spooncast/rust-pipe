use futures::sync::mpsc::Sender;
use futures::Sink;

use crate::error::Error;
use crate::frame::MediaFrame;
use crate::node::MediaSink;
use crate::{Poll, StartSend};

pub struct MediaSender {
    sender: Sender<MediaFrame>
}

impl MediaSender {
    pub(crate) fn new(sender: Sender<MediaFrame>) -> Self {
        MediaSender { sender }
    }
}

impl MediaSink for MediaSender {
    fn start_send(&mut self, frame: MediaFrame) -> StartSend<MediaFrame, Error> {
        self.sender.start_send(frame).map_err(|_| Error::Unknown)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sender.poll_complete().map_err(|_| Error::Unknown)
    }

    fn close(&mut self) -> Poll<(), Error> {
        self.sender.close().map_err(|_| Error::Unknown)
    }
}
