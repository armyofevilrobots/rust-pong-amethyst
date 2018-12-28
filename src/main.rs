extern crate amethyst;

mod pong;
mod systems;
use crate::pong::Pong;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    /* Let's grab the app root dir using the new method */
    let root_dir = application_root_dir()?;
    let root_dir = match root_dir.to_str() {
        None => ".",
        Some(ref somedir) => somedir,
    };
    // let root_dir = application_root_dir()?;
    // let root_dir = root_dir.to_str().unwrap();
    let path = format!("{}/resources/display_config.ron", root_dir);
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings_config.ron", root_dir);

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}
