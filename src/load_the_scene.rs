use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn load_the_scene(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
}
