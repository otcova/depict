mod chrono;
use std::{cell::RefCell, rc::Rc};

pub use chrono::*;

use crate::Depict;

struct Average<const N: usize> {
    data: [f32; N],
    len: usize,
    current_index: usize,
    average: f32,
}

impl<const N: usize> Average<N> {
    fn new() -> Self {
        Self {
            data: [0.; N],
            len: 0,
            current_index: N - 1,
            average: 0.,
        }
    }
    fn current(&self) -> f32 {
        self.data[self.current_index]
    }
    fn average(&self) -> f32 {
        self.average
    }
    fn push(&mut self, n: f32) {
        self.current_index = (self.current_index + 1) % N;
        if self.len < N {
            self.average *= self.len as f32;
            self.len += 1;
            self.average /= self.len as f32;
            self.average += n / self.len as f32;
        } else {
            self.average += (n - self.data[self.current_index]) / self.len as f32;
        }
        self.data[self.current_index] = n;
    }
}

pub struct PauseFrameTime {
    pause_chrono: Rc<RefCell<Option<Chrono>>>,
}

pub struct FrameTime {
    pause_chrono: Rc<RefCell<Option<Chrono>>>,
    frame_chrono: Chrono,
    frame_count: f32,
    render_seconds: Average<60>,
    delta_seconds: Average<60>,
    seconds: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        Self {
            pause_chrono: Rc::new(RefCell::new(None)),
            frame_chrono: Chrono::start(),
            frame_count: 0.,
            render_seconds: Average::new(),
            delta_seconds: Average::new(),
            seconds: 0.,
        }
    }
    pub fn start_frame(&mut self) {
        self.frame_count += 1.;

        if self.frame_count > 1. {
            let mut time = self.frame_chrono.seconds();
            if let Some(chrono) = self.pause_chrono.take() {
                time -= chrono.seconds();
            }
            self.delta_seconds.push(time);
            self.seconds += time;
        }

        self.frame_chrono = Chrono::start();
    }

    pub fn end_frame(&mut self) {
        self.render_seconds.push(self.frame_chrono.seconds());
    }

    pub fn pause_handle(&self) -> PauseFrameTime {
        PauseFrameTime {
            pause_chrono: self.pause_chrono.clone(),
        }
    }
}
impl PauseFrameTime {
    pub fn pause_time(&self) {
        if self.pause_chrono.borrow().is_none() {
            *self.pause_chrono.borrow_mut() = Some(Chrono::start());
        }
    }
}

impl Depict {
    pub fn frame_count(&self) -> f32 {
        self.time.frame_count
    }
    pub fn seconds(&self) -> f32 {
        self.time.seconds
    }
    pub fn render_seconds(&self) -> f32 {
        self.time.render_seconds.current()
    }
    pub fn average_render_seconds(&self) -> f32 {
        self.time.render_seconds.average()
    }
    pub fn delta_seconds(&self) -> f32 {
        self.time.delta_seconds.current()
    }
    pub fn average_delta_seconds(&self) -> f32 {
        self.time.delta_seconds.average()
    }
    pub fn fps(&self) -> f32 {
        1. / self.time.delta_seconds.current()
    }
    pub fn average_fps(&self) -> f32 {
        1. / self.time.delta_seconds.average()
    }
}
