use bevy::prelude::*;

use crate::components::*;

/// System to animate agents (pulse effect for now)
pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AnimationController, &Agent)>,
) {
    for (mut transform, mut anim, _agent) in query.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.frame = (anim.frame + 1) % 4;
        }

        // Simple pulse animation based on state
        let scale = match anim.state {
            AnimationState::Idle => {
                // Gentle breathing effect
                let pulse = (time.elapsed_secs() * 2.0).sin() * 0.05 + 1.0;
                pulse
            }
            AnimationState::WalkingRight
            | AnimationState::WalkingLeft
            | AnimationState::WalkingUp
            | AnimationState::WalkingDown => {
                // Bouncy walk effect
                let bounce = (time.elapsed_secs() * 10.0).sin().abs() * 0.1 + 1.0;
                bounce
            }
            AnimationState::Working => {
                // Busy vibration effect
                let vibrate = (time.elapsed_secs() * 20.0).sin() * 0.02 + 1.0;
                vibrate
            }
        };

        transform.scale = Vec3::splat(scale);
    }
}

/// System plugin for animations
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animation_system);
    }
}
