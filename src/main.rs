struct DummyStream {
    buffer: Vec<u8>,
    frame_count: u32,
}

impl DummyStream {
    fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            frame_count: 0,
        }
    }

    // Returns a reference to a camera buffer owned by the camera driver.
    fn next(&mut self) -> &[u8] {
        // Update the buffer with a new "frame"
        for (i, byte) in self.buffer.iter_mut().enumerate() {
            *byte = ((i as u32 + self.frame_count) % 256) as u8;
        }
        self.frame_count += 1;
        &self.buffer
    }
}

struct TestCameraMono {
    left_stream: DummyStream,
}

impl TestCameraMono {
    fn new() -> Self {
        Self {
            left_stream: DummyStream::new(640, 480), // Example resolution
        }
    }

    fn next_frame(&mut self) -> &[u8] {
        {
            let l_buf = self.left_stream.next();

            // Sometimes, we want to skip this frame based on some condition that
            // can only be calculated when we have access to the buffer.
            // In the camera case, this will be when a frame is too old compared to the other camera.
            let delta = l_buf[0] - 127;

            let condition = delta < 80;
            if condition {
                return l_buf;
            }
        }
        self.left_stream.next()
    }
}

struct TestCameraStereo {
    left_stream: DummyStream,
    right_stream: DummyStream,
}

enum DeltaRes {
    LeftLagging,
    RightLagging,
    Synchronized,
}

impl DeltaRes {
    pub fn from_delta(delta: u8) -> Self {
        if delta < 80 {
            DeltaRes::LeftLagging
        } else if delta < 255 - 80 {
            DeltaRes::Synchronized
        } else {
            DeltaRes::RightLagging
        }
    }
}

impl TestCameraStereo {
    fn new() -> Self {
        Self {
            left_stream: DummyStream::new(640, 480), // Example resolution
            right_stream: DummyStream::new(640, 480), // Example resolution
        }
    }

    fn next_frame(&mut self) -> (&[u8], &[u8]) {
        let l_buf = self.left_stream.next();
        let r_buf = self.right_stream.next();

        // Sometimes, we want to skip this frame based on some condition that
        // can only be calculated when we have access to the buffer.
        // In the camera case, this will be when a frame is too old compared to the other camera.
        let delta = l_buf[0] - r_buf[0];

        match DeltaRes::from_delta(delta) {
            DeltaRes::Synchronized => {
                // Cameras are synchronized, and we do not need to update any of them
                (l_buf, r_buf)
            }
            DeltaRes::LeftLagging => {
                // Left is lagging behind, update it
                (self.left_stream.next(), r_buf)
            }
            DeltaRes::RightLagging => {
                // Right is lagging behind, update it
                (l_buf, self.right_stream.next())
            }
        }
    }
}

fn main() {
    let mut mono_cam = TestCameraMono::new();

    let l_buf = mono_cam.next_frame();
    println!("mono_cam: {}", l_buf[10]);

    let mut stereo_cam = TestCameraStereo::new();
    let (l_buf, r_buf) = stereo_cam.next_frame();
    println!("stereo_cam: {}, {}", l_buf[10], r_buf[10]);
}
