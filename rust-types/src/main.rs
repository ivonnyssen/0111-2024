use std::time::SystemTime;

struct Camera {
    state: CameraState,
}


enum CameraState {
    Idle,
    Exposing { 
        start: SystemTime,
        duration: u64 
    },
}

impl Camera {
    fn new() -> Camera {
        Camera {
            state: CameraState::Idle,
        }
    }

    fn start_exposure(&mut self, duration: u64) {
        self.state = CameraState::Exposing {
            start: SystemTime::now(),
            duration,
        };
    }

    fn stop_exposure(&mut self) {
        self.state = CameraState::Idle;
    }
}


fn main() {
    let mut camera = Camera::new();
    camera.start_exposure(1000);
    match camera.state {
        CameraState::Exposing { start, duration } => {
            println!("Exposing started at {:?}", start);
            println!("Exposing for {} milliseconds", duration);
        }
        _ => println!("Camera is not exposing"),
    }
    //should wait for 1000 milliseconds, not implemented here
    camera.stop_exposure();
}
