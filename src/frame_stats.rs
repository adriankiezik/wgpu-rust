use std::time::{Duration, Instant};

pub struct FrameStats {
    pub last_frame_time: Instant,
    pub frame_count: u64,
    pub fps: f32,
    pub fps_print_interval: Duration,
}

impl FrameStats {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            fps_print_interval: Duration::from_millis(350),
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        let elapsed_time = self.last_frame_time.elapsed();

        if elapsed_time >= self.fps_print_interval {
            let fps = self.frame_count as f32 / elapsed_time.as_secs_f32();

            self.fps = fps;
            self.frame_count = 0;
            self.last_frame_time = Instant::now();
        }
    }
}
