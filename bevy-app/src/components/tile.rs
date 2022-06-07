use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// TODO: fix these names plz
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Inspectable)]
pub struct Tile;
