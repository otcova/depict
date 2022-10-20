mod chrono;
pub use chrono::*;

pub struct Average<const N: usize> {
    data: [f32; N],
    len: usize,
    current_index: usize,
    average: f32,
}

impl<const N: usize> Average<N> {
    pub fn new() -> Self {
        Self {
            data: [0.; N],
            len: 0,
            current_index: N - 1,
            average: 0.,
        }
    }
    pub fn current(&self) -> f32 {
        self.data[self.current_index]
    }
    pub fn average(&self) -> f32 {
        self.average
    }
    pub fn push(&mut self, n: f32) {
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

pub struct FrameTime {
    pub frame_chrono: Chrono,
    pub global_chrono: Chrono,
    pub frame_count: f32,
    pub render_seconds: Average<60>,
    pub delta_seconds: Average<60>,
    pub seconds: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        Self {
            frame_chrono: Chrono::start(),
            global_chrono: Chrono::start(),
            frame_count: 0.,
            render_seconds: Average::new(),
            delta_seconds: Average::new(),
            seconds: 0.,
        }
    }
    pub fn start_frame(&mut self) {
        self.delta_seconds.push(self.frame_chrono.seconds());
        self.frame_chrono = Chrono::start();

        self.frame_count += 1.;

        if self.frame_count == 1. {
            self.global_chrono = self.frame_chrono;
        } else {
            self.seconds = self.global_chrono.seconds();
        }
    }

    pub fn end_frame(&mut self) {
        self.render_seconds.push(self.frame_chrono.seconds());
    }
}
