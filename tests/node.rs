use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use futures::{Future, task};

use rusty_pipe::error::Error;
use rusty_pipe::frame::MediaFrame;
use rusty_pipe::node::forward_filter::ForwardFilter;
use rusty_pipe::node::{MediaFilter, MediaSink, MediaSource};
use rusty_pipe::{Async, AsyncSink, Poll, StartSend};

struct DummySource {
    ptss: Vec<u64>,
    running: bool,
    frames: Arc<Mutex<VecDeque<Option<MediaFrame>>>>,
}

impl PartialEq for DummySource {
    fn eq(&self, other: &Self) -> bool {
        self.ptss == other.ptss
    }
}

impl fmt::Debug for DummySource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DummySource {{ ptss: {:?} }}", self.ptss)
    }
}

impl DummySource {
    pub fn new<I>(ptss: I) -> Self
    where
        I: IntoIterator<Item = u64>
    {
        DummySource {
            ptss: ptss.into_iter().collect(),
            running: false,
            frames: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn run(&mut self, task: task::Task) {
        let frames = self.frames.clone();
        let ptss = self.ptss.to_owned();
        thread::spawn(move|| {
            ptss.iter().for_each(|pts| {
                thread::sleep(Duration::from_millis(10));
                {
                    let mut frames = frames.lock().unwrap();
                    frames.push_back(Some(MediaFrame::builder()
                                          .pts(Duration::from_secs(*pts))
                                          .build()));
                }
                task.notify();
            });

            thread::sleep(Duration::from_millis(10));
            {
                let mut frames = frames.lock().unwrap();
                frames.push_back(None);
            }
            task.notify();
        });
    }
}

impl MediaSource for DummySource {
    fn poll(&mut self) -> Poll<Option<MediaFrame>, Error> {
        if !self.running {
            self.run(task::current());
            self.running = true;
        }

        {
            let mut frames = self.frames.lock().unwrap();
            if let Some(frame) = frames.pop_front() {
                Ok(Async::Ready(frame))
            } else {
                Ok(Async::NotReady)
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct FixedDurationFilter {
    dur: Duration,
    next_pts: Duration,
    buffer: Option<Option<MediaFrame>>,
}

impl FixedDurationFilter {
    pub fn new(sec: u64) -> Self {
        FixedDurationFilter {
            dur: Duration::from_secs(sec),
            next_pts: Duration::from_secs(0),
            buffer: None,
        }
    }
}

impl MediaFilter for FixedDurationFilter {
    fn start_send(&mut self, frame: Option<MediaFrame>) -> StartSend<Option<MediaFrame>, Error> {
        if self.buffer.is_none() {
            self.buffer = Some(frame);
            Ok(AsyncSink::Ready)
        } else {
            Ok(AsyncSink::NotReady(frame))
        }
    }

    fn poll_complete(&mut self) -> Poll<Option<MediaFrame>, Error> {
        if let Some(item) = self.buffer.take() {
            if let Some(frame) = item {
                if frame.pts < self.next_pts {
                    Ok(Async::NotReady)
                } else {
                    let item = if frame.pts < self.next_pts + self.dur {
                        Some(frame) // XXX fixed duration
                    } else {
                        self.buffer = Some(Some(frame));
                        Some(MediaFrame::builder()
                             .pts(self.next_pts)
                             .build())
                    };
                    self.next_pts = self.next_pts + self.dur;
                    Ok(Async::Ready(item))
                }
            } else {
                Ok(Async::Ready(None))
            }
        } else {
            Ok(Async::NotReady)
        }
    }
}

#[derive(PartialEq, Debug)]
struct DummySink {
    frames: Vec<MediaFrame>,
}

impl DummySink {
    pub fn new() -> Self {
        DummySink { frames: Vec::new() }
    }
}

impl MediaSink for DummySink {
    fn start_send(&mut self, frame: MediaFrame) -> StartSend<MediaFrame, Error> {
        self.frames.push(frame);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), Error> {
        Ok(().into())
    }
}

#[test]
fn forward_sink() {
    assert_done(
        || {
            DummySource::new(vec![0, 10, 15, 30])
                .forward_filter(FixedDurationFilter::new(10))
                .forward_sink(DummySink::new())
        },
        Ok((
            ForwardFilter::new(
                DummySource {
                    ptss: vec![0, 10, 15, 30],
                    running: true,
                    frames: Arc::new(Mutex::new(VecDeque::new())),
                },
                FixedDurationFilter {
                    dur: Duration::from_secs(10),
                    next_pts: Duration::from_secs(40),
                    buffer: None
                },
            ),
            DummySink {
                frames: vec![
                    MediaFrame::builder()
                        .pts(Duration::from_secs(0))
                        .build(),
                    MediaFrame::builder()
                        .pts(Duration::from_secs(10))
                        .build(),
                    MediaFrame::builder()
                        .pts(Duration::from_secs(20))
                        .build(),
                    MediaFrame::builder()
                        .pts(Duration::from_secs(30))
                        .build(),
                ],
            },
        )),
    );
}

use std::fmt;

fn assert_done<T, F>(f: F, result: Result<T::Item, T::Error>)
where
    T: Future,
    T::Item: PartialEq + fmt::Debug,
    T::Error: PartialEq + fmt::Debug,
    F: FnOnce() -> T,
{
    assert_eq!(f().wait(), result);
}
