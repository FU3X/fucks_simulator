use std::f32::consts::TAU;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;

pub fn player_controller(mut commands: Commands) {
    commands.spawn((
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true }, // Prevent clipping when going fast
        TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
        LogicalPlayer(0),
        FpsControllerInput {
            yaw: TAU * 5.0 / 8.0,
            pitch: -TAU / 12.0,
            ..default()
        },
        FpsController { ..default() },
    ));

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            ..default()
        },
        RenderPlayer(0),
    ));
}

pub fn manage_cursor(
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}
