// use futures::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use futures::sync::mpsc::Receiver;
use futures::Stream;

use crate::error::Error;
use crate::frame::MediaFrame;
use crate::node::MediaSource;
use crate::Poll;

pub struct MediaReceiver {
    receiver: Receiver<MediaFrame>
}

impl MediaReceiver {
    pub(crate) fn new(receiver: Receiver<MediaFrame>) -> Self {
        MediaReceiver { receiver }
    }
}

impl MediaSource for MediaReceiver {
    fn poll(&mut self) -> Poll<Option<MediaFrame>, Error> {
        self.receiver.poll().map_err(|_| Error::Unknown)
    }
}
