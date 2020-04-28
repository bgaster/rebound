//! Description: 
//! 
//! System for Commands
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
     derive::SystemDesc,
     ecs::{Read, Write, System, SystemData, Entities, WriteStorage},
     shrev::{EventChannel, ReaderId},
  };
  
  use crate::{
    ui::mainui::{MainUI},  
    commands::{Command, Draw},
    bindings::{ActionBinding},
    commands::svg_path::*,
    commands::svg::{ForegroundLayer, MiddlegroundLayer, BackgroundLayer},
  };
  
  use log::info;
  
  #[derive(SystemDesc)]
  #[system_desc(name(CommandSystemDesc))]
  pub struct CommandSystem {
     #[system_desc(event_channel_reader)]
     reader_id: ReaderId<Command>,
  }
  
  impl CommandSystem {
     pub fn new(reader_id: ReaderId<Command>) -> Self {
        Self { 
            reader_id,
        }
     }
  }
  
  impl<'a> System<'a> for CommandSystem {
     type SystemData = (
         Read<'a, EventChannel<Command>>,
         Write<'a, MainUI>,
         Write<'a, Draw>,
         Entities<'a>,
         WriteStorage<'a, MoveTo>,
         WriteStorage<'a, LineTo>
     );
  
     fn run(&mut self, 
        (commands, mut menu, mut draw, entities, 
            mut move_to, mut line_to): Self::SystemData) {
        // process any incoming commands
        for event in commands.read(&mut self.reader_id) {
            match event {
                // display/enable grid
                Command::Input(ActionBinding::ViewToggleGrid) => {
                    menu.toggle_grid();
                }
                // display/enable tools
                Command::Input(ActionBinding::ViewToggleTools) => {
                    menu.toggle_tools();
                }
                // set active layer
                Command::Input(ActionBinding::LayersBackground) => {
                    draw.layer_background();
                }
                Command::Input(ActionBinding::LayersMiddleground) => {
                    draw.layer_middleground();
                }
                Command::Input(ActionBinding::LayersForeground) => {
                    draw.layer_foreground();
                }
                // increment line thinkness for current layer
                Command::Input(ActionBinding::StyleThicker) => {
                    draw.inc_thickness(1.0);
                }
                Command::Input(ActionBinding::StyleThicker5) => {
                    draw.inc_thickness(5.0);
                }
                // decrement line thinkness for current layer
                Command::Input(ActionBinding::StyleThinner) => {
                    draw.inc_thickness(-1.0);
                }
                Command::Input(ActionBinding::StyleThinner5) => {
                    draw.inc_thickness(-5.0);
                }
                // set draw colour for current layer
                Command::DrawColour(colour) => {
                    draw.set_colour(colour);
                }
                // add point
                Command::AddControlPoint(point) => {
                    draw.add_point(point);
                }
                // draw line
                Command::Input(ActionBinding::StrokeLine) => {
                    draw.line(&entities, &mut move_to, &mut line_to);
                }
                _ => {
                    info!("{:?}", event);
                }
            }
        }
     }
  }