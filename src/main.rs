use bevy::prelude::*;

pub mod player_controller;
use player_controller::{manage_cursor, player_controller};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player_controller)
        .add_systems(Update, manage_cursor)
        .run();
}
