extern crate amethyst;

mod ball;
mod constants;
mod paddles;
mod scoreboard;
mod sprites;
pub use crate::pong::{
    ball::{initialise_ball, Ball},
    constants::*,
    paddles::*,
    scoreboard::{initialise_scoreboard, ScoreBoard, ScoreText},
    sprites::load_sprite_sheet,
};

use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Paddle>();
        world.register::<Ball>();

        initialise_scoreboard(world);
        initialise_ball(world, sprite_sheet_handle.clone());
        initialise_paddles(world, sprite_sheet_handle.clone());
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}
