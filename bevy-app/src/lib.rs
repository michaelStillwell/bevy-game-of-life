use crate::resources::MapOptions;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use map_plugin::MapPlugin;

mod components;
mod map_plugin;
mod resources;
mod systems;

pub fn run() {
    let mut app = App::new()
        .insert_resource(WindowDescriptor {
            title: "Conway's Game of Life".to_string(),
            height: 600.,
            width: 900.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(MapOptions {
            map_size: (10, 10),
            cell_padding: 3.0,
            ..default()
        })
        .add_plugin(MapPlugin)
        .add_startup_system(camera_setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
