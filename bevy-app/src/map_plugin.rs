use crate::components::{Coordinates, Tile};
use crate::resources::{CellSize, MapOptions, MapPosition};
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use gof_plugin::resources::{Board, Cell};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_map);
        app.register_inspectable::<Tile>();
        info!("Loaded Map Plugin");
    }
}

impl MapPlugin {
    pub fn create_map(
        mut commands: Commands,
        map_options: Option<Res<MapOptions>>,
        window: Option<Res<WindowDescriptor>>,
    ) {
        let window = match window {
            Some(w) => w,
            None => panic!("Must supply WindowDescriptor"),
        };

        let options = match map_options {
            Some(o) => o.clone(),
            None => MapOptions::default(),
        };

        let mut board = Board::new(options.map_size.0, options.map_size.1);

        // NOTE: this is with values set already
        // let mut board = Board::from(BOARD);
        #[cfg(feature = "debug")]
        info!("{}", board.to_string());

        let cell_size = match options.cell_size {
            CellSize::Fixed(v) => v,
            CellSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (board.width(), board.height()))
            }
        };

        let board_size = Vec2::new(
            board.width() as f32 * cell_size,
            board.height() as f32 * cell_size,
        );
        info!("Board size: {}", board_size);

        let map_position = match options.position {
            MapPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            MapPosition::Custom(p) => p,
        };

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(map_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_cells(parent, &board, cell_size, options.cell_padding, Color::GRAY);
            });
    }

    fn spawn_cells(
        parent: &mut ChildBuilder,
        board: &Board,
        size: f32,
        padding: f32,
        color: Color,
    ) {
        for (y, line) in board.map().iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let coords = Coordinates {
                    x: x as u8,
                    y: y as u8,
                };
                let mut cmd = parent.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Cell ({}, {})", x, y)))
                .insert(coords);

                match cell {
                    Cell::Empty => {
                        cmd.insert(Tile).with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GREEN,
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                ..default()
                            });
                        });
                    }
                    Cell::Full => (),
                }
            }
        }
    }

    fn adaptive_tile_size(
        window: Res<WindowDescriptor>,
        (min, max): (f32, f32),
        (width, height): (u8, u8),
    ) -> f32 {
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}

// Premade board
/*
const BOARD: Vec<Vec<Cell>> = vec![
    vec![
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    vec![
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    vec![
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    vec![
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
    vec![
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
        Cell::Empty,
    ],
];
 */
