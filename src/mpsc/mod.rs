pub mod receiver;
pub use receiver::MediaReceiver;

pub mod sender;
pub use sender::MediaSender;

pub fn media_channel(buffer: usize) -> (MediaSender, MediaReceiver) {
    let (sender, receiver) = futures::sync::mpsc::channel(buffer);
    (MediaSender::new(sender), MediaReceiver::new(receiver))
}
