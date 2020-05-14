//! Description: 
//! 
//! System for Commands
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
     derive::SystemDesc,
     ecs::{Read, Write, System, SystemData, Entities, WriteStorage, ReadStorage},
     shrev::{EventChannel, ReaderId},
     window::{ScreenDimensions},
  };
  
  use crate::{
    ui::mainui::{MainUI},  
    commands::{Command, Draw, HoverMode},
    bindings::{ActionBinding},
    commands::svg_path::*,
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
         WriteStorage<'a, LineTo>,
         WriteStorage<'a, QuadraticBeizer>,
         WriteStorage<'a, EllipticalArc>,
         WriteStorage<'a, Close>
     );
  
     fn run(&mut self, 
        (commands, mut menu, mut draw, entities,
            mut move_to, mut line_to, mut quad_beizer, mut arc, mut close): Self::SystemData) {
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
                    menu.set_colour(draw.get_colour());
                }
                Command::Input(ActionBinding::LayersMiddleground) => {
                    draw.layer_middleground();
                    menu.set_colour(draw.get_colour());
                }
                Command::Input(ActionBinding::LayersForeground) => {
                    draw.layer_foreground();
                    menu.set_colour(draw.get_colour());
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
                Command::Input(ActionBinding::StyleLineCap) => {
                    draw.linecap();
                }
                Command::Input(ActionBinding::StyleLineJoin) => {
                    draw.linejoin();
                }
                // set draw colour for current layer
                Command::DrawColour(colour) => {
                    draw.set_colour(colour);
                }
                // add point
                Command::AddControlPoint(point) => {
                    draw.add_point(point);
                }
                // draw beizer
                Command::Input(ActionBinding::StrokeBezier) => {
                    draw.quad_beizer(&entities, &mut move_to, &mut quad_beizer);
                }
                // draw line
                Command::Input(ActionBinding::StrokeLine) => {
                    draw.line(&entities, &mut move_to, &mut line_to);
                }
                // draw arc
                Command::Input(ActionBinding::StrokeArc) => {
                    draw.arc(true, &entities, &mut move_to, &mut line_to, &mut arc);
                }
                // draw rev arc
                Command::Input(ActionBinding::StrokeArcRev) => {
                    draw.arc(false, &entities, &mut move_to, &mut line_to, &mut arc);
                }
                // close path
                Command::Input(ActionBinding::StrokeClose) => {
                    draw.close(&entities, &mut close);
                }
                // toggle fill mode
                Command::Input(ActionBinding::StyleFill) => {
                    draw.fill();
                }
                // hover action begin
                Command::Hover(HoverMode::Start, action) => {
                    draw.hover_start((*action).clone());
                }
                // hover action end
                Command::Hover(HoverMode::End, _) => {
                    draw.hover_end();
                }
                // merge layers into active layer
                Command::Input(ActionBinding::LayersMergeLayers) => {
                    draw.merge_layers();
                }
                // file save
                Command::Input(ActionBinding::FileSave) => {
                    draw.save(menu.dimensions(), &move_to, &line_to, &quad_beizer, &arc, &close);
                }
                // file open
                Command::Input(ActionBinding::FileOpen) => {
                    draw.load(&entities, &mut move_to, &mut line_to, &mut quad_beizer, &mut arc, &mut close);
                }
                _ => {
                    info!("{:?}", event);
                }
            }
        }
     }
  }