use bevy::prelude::*;

pub fn run() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Conway's Game of Life".to_string(),
            height: 600.,
            width: 900.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
