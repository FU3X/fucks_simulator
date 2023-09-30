use bevy::prelude::*;

pub mod load_the_scene;
pub mod player_controller;
use load_the_scene::load_the_scene;
use player_controller::{manage_cursor, player_controller, respawn};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player_controller)
        .add_systems(Update, (manage_cursor, load_the_scene, respawn))
        .run();
}
