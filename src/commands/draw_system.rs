//! Description: 
//! 
//! System for Commands
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
     //derive::SystemDesc,
     core::SystemDesc,
     ecs::{Read, Write, System, SystemData, Entities, ReadStorage, World, Join},
     shrev::{EventChannel, ReaderId},
  };
  
  use crate::{
    ui::mainui::{MainUI},  
    commands::{Command, Draw},
    bindings::{ActionBinding},
    commands::svg_path::*,
  };
  
  use log::info;
  
//#[derive(SystemDesc)]
// #[system_desc(name(DrawSystemDesc))]
pub struct DrawSystem {

}

#[derive(Default, Debug)]
pub struct DrawSystemDesc;


impl<'a, 'b> SystemDesc<'a, 'b, DrawSystem> for DrawSystemDesc {
    fn build(self, world: &mut World) -> DrawSystem {
        <DrawSystem as System<'_>>::SystemData::setup(world);

        //world.insert(self.output.clone());

        DrawSystem{}
    }
}
  
//   impl DrawSystem {
//     pub fn new(reader_id: ReaderId<Command>) -> Self {
//         Self { 
//             reader_id,
//         }
//      }
//   }
  

  impl<'a> System<'a> for DrawSystem {
     type SystemData = (
         Read<'a, Draw>,
         ReadStorage<'a, MoveTo>,
         ReadStorage<'a, LineTo>,
         Entities<'a>,
     );
  
     fn run(&mut self, 
        (draw, move_to, line_to, entities): Self::SystemData) { 

            //let mut j = (&move_to, &line_to).join();

            for e in &draw.layers[draw.active_layer].entities {
                if let Some(entity) = e.entity {
                    if let Some(m_to) = move_to.get(entity) {
                        println!("{}", m_to.gen_output());
                    }
                    else if let Some(l_to) = line_to.get(entity) {
                        println!("{}", l_to.gen_output());
                    }
                }
            }
     }
  }