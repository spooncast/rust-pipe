use std::time::Duration;
use super::MediaFrame;

pub struct MediaFrameBuilder {
    pts: Duration,
}

impl Default for MediaFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaFrameBuilder {
    pub(crate) fn new() -> Self {
        MediaFrameBuilder {
            pts: Duration::default(),
        }
    }

    pub fn pts(mut self, pts: Duration) -> Self {
        self.pts = pts;
        self
    }

    pub fn build(self) -> MediaFrame {
        MediaFrame::new(self.pts)
    }
}
