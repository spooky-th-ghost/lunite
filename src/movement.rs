use crate::camera::PrimaryCamera;
use crate::input::PlayerAction;
use crate::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_player_move_direction, move_with_velocity))
            .register_type::<MoveDirection>()
            .register_type::<Speed>();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MoveDirection(Vec3);

impl MoveDirection {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value.normalize_or_zero();
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(f32);

impl Speed {
    pub fn new(value: f32) -> Self {
        Speed(value)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        self.0 = value;
    }
}

fn set_player_move_direction(
    mut player_query: Query<(&mut MoveDirection, &ActionState<PlayerAction>), With<Player>>,
    camera_query: Query<&Transform, With<PrimaryCamera>>,
) {
    if let Ok((mut move_direction, action)) = player_query.get_single_mut() {
        if let Ok(transform) = camera_query.get_single() {
            if action.pressed(PlayerAction::Move) {
                if let Some(axis_pair) = action.axis_pair(PlayerAction::Move) {
                    let mut move_vector =
                        (transform.forward() * axis_pair.y()) + (transform.right() * axis_pair.x());
                    move_vector.y = 0.0;
                    move_direction.set(move_vector);
                }
            } else {
                move_direction.set(Vec3::ZERO);
            }
        }
    }
}

fn move_with_velocity(time: Res<Time>, mut query: Query<(&mut Velocity, &MoveDirection, &Speed)>) {
    for (mut velocity, move_direction, speed) in &mut query {
        if move_direction.is_any() {
            velocity.linvel += move_direction.get() * speed.get() * time.delta_seconds();
        }
    }
}
