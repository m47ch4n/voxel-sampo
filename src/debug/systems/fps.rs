use bevy::prelude::*;

pub struct FpsInfo {
    pub fps: f32,
}

impl FpsInfo {
    pub fn from_time(time: &Time) -> Self {
        Self {
            fps: 1.0 / time.delta().as_secs_f32(),
        }
    }

    pub fn format(&self) -> String {
        format!("FPS: {:.1}", self.fps)
    }
}
