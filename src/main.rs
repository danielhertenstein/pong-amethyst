extern crate amethyst;

use amethyst::{LoggerConfig, StdoutLog, LogLevelFilter};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::utils::application_root_dir;

mod systems;
mod pong;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: LogLevelFilter::Warn,
        log_file: None,
        allow_env_override: true,
    });

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir(),
    );
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}