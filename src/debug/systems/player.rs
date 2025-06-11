use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerInfo {
    pub position: Vec3,
    pub velocity: Option<Vec3>,
    pub speed: Option<f32>,
    pub present: bool,
}

impl PlayerInfo {
    pub fn from_queries(
        player_query: &Query<&Transform, With<Player>>,
        rigidbody_query: &Query<&Velocity, With<RigidBody>>,
    ) -> Self {
        if let Ok(player_transform) = player_query.single() {
            let position = player_transform.translation;
            
            let (velocity, speed) = if let Ok(velocity) = rigidbody_query.single() {
                let vel = velocity.linvel;
                (Some(vel), Some(vel.length()))
            } else {
                (None, None)
            };

            Self {
                position,
                velocity,
                speed,
                present: true,
            }
        } else {
            Self {
                position: Vec3::ZERO,
                velocity: None,
                speed: None,
                present: false,
            }
        }
    }

    pub fn format(&self) -> String {
        if !self.present {
            return "Player: Not found".to_string();
        }

        let mut info = format!(
            "XYZ: {:.3} / {:.3} / {:.3}\nBlock: {} {} {}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.position.x.floor() as i32,
            self.position.y.floor() as i32,
            self.position.z.floor() as i32
        );

        if let (Some(velocity), Some(speed)) = (self.velocity, self.speed) {
            info.push_str(&format!(
                "\nVelocity: {:+07.3} / {:+07.3} / {:+07.3}\nSpeed: {:.3} m/s",
                velocity.x, velocity.y, velocity.z, speed
            ));
        }

        info
    }
}