//! Description: 
//! 
//! Main entry point for rebound app.
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//! 
//! 

use amethyst::{
    input::{
        InputBundle,
    },
    core::{
        transform::TransformBundle, 
        frame_limiter::FrameRateLimitStrategy,
    },
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow}, 
        types::DefaultBackend, 
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir},
};

use std::time::Duration;

use amethyst_lyon::{RenderLyon};

use palette::{Pixel, Srgba};

mod states;
mod bindings;
mod bundle;
mod ui;
mod vector_meshes;
mod commands;

use bindings::{InputBindingTypes};
use bundle::{ReboundBundle};

const DISPLAY: &'static str = "config/display.ron";
#[cfg(target_os = "macos")]
const BINDINGS: &'static str = "bindings_macos.ron";
#[cfg(not(target_os = "macos"))]
const BINDINGS: &'static str = "bindings.ron";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join(DISPLAY);
    let assets_dir = app_root.join("assets/");
    let bindings_config = app_root.join("config").join(BINDINGS);

    // let colour = Srgba::new(1.000, 0.687, 0.344, 1.000).into_linear();
    
    // // Encode the result back into sRGB and create a byte array
    // let pixel: [f32; 4] = colour
    //     .into_format()
    //     .into_raw();

    // println!("{:?}", pixel);

    let colour = Srgba::new(0.271, 0.208, 0.349, 1.000).into_linear();
    
    // Encode the result back into sRGB and create a byte array
    let pixel: [f32; 4] = colour
        .into_format()
        .into_raw();

    let game_data = GameDataBuilder::default()
        //.with_bundle(FlyControlBundle::<InputBindingTypes>::new(None, None, None))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<InputBindingTypes>::new()
                      .with_bindings_from_file(bindings_config)?)?
        .with_bundle(UiBundle::<InputBindingTypes>::new())?
        .with_bundle(ReboundBundle::<InputBindingTypes>::new())?
        //.with(Processor::<Source>::new(), "source_processor", &[])
        //.with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_system_desc(ui::RUIEventHandlerSystemDesc::default(), "rui_event_handler", &[])
        .with_system_desc(commands::CommandSystemDesc::default(), "command_handler", &[])
        .with_system_desc(commands::DrawSystemDesc::default(), "draw_handler", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear(pixel), // [0.183, 0.125, 0.241, 1.000]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderLyon::default()),     
        )?;

    // To keep CPU useage down a bit limited to 30 FPS, not an issue for this app
    let mut game = Application::build(assets_dir, states::LoadState::default())?
                    .with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 30)
                    .build(game_data)?;

    game.run();
    Ok(())
}