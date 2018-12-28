use crate::pong::constants::*;
use amethyst::prelude::*;
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    renderer::{SpriteRender, SpriteSheetHandle},
};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    //Assign the sprite...
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}
