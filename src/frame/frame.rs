use std::time::Duration;
use super::MediaFrameBuilder;

#[derive(PartialEq, Debug)]
pub struct MediaFrame {
    pub pts: Duration,
}

impl MediaFrame {
    pub fn new(pts: Duration) -> Self {
        MediaFrame {
            pts: pts,
        }
    }

    pub fn builder() -> MediaFrameBuilder {
        MediaFrameBuilder::default()
    }
}
