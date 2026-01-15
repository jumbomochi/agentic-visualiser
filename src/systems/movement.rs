use bevy::prelude::*;

use crate::components::*;

/// System to move agents toward their targets
pub fn movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &MovementTarget,
        &Speed,
        &mut AnimationController,
    )>,
) {
    for (entity, mut transform, target, speed, mut anim) in query.iter_mut() {
        let current = Vec2::new(transform.translation.x, transform.translation.y);
        let direction = target.position - current;
        let distance = direction.length();

        // If close enough, stop moving
        if distance < 5.0 {
            // Remove movement target
            commands.entity(entity).remove::<MovementTarget>();
            anim.state = AnimationState::Idle;
            continue;
        }

        // Move toward target
        let velocity = direction.normalize() * speed.0 * time.delta_secs();

        // Don't overshoot
        let movement = if velocity.length() > distance {
            direction
        } else {
            velocity
        };

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;

        // Update animation state based on movement direction
        if movement.x.abs() > movement.y.abs() {
            if movement.x > 0.0 {
                anim.state = AnimationState::WalkingRight;
            } else {
                anim.state = AnimationState::WalkingLeft;
            }
        } else if movement.y > 0.0 {
            anim.state = AnimationState::WalkingUp;
        } else {
            anim.state = AnimationState::WalkingDown;
        }
    }
}

/// System plugin for movement
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system);
    }
}
