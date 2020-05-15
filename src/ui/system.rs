//! Description: 
//! 
//! System for UI IO
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
  //ecs::prelude::{System, SystemData, Write, Read},
   derive::SystemDesc,
   ecs::{Read, Write, ReadStorage, System, SystemData, WriteStorage},
   shrev::{EventChannel, ReaderId},
   ui::{UiEvent, UiEventType, UiText},
   input::{InputEvent, InputHandler},
};

use crate::{
   ui::mainui::{MainUI},  
   bindings::{InputBindingTypes, ActionBinding},
   commands::{Command, Draw, HoverMode},
};

use lyon::{ 
   math::{vector},
};

use amethyst_lyon::{
   utils::{Mesh}
};

use log::info;

#[derive(SystemDesc)]
#[system_desc(name(RUIEventHandlerSystemDesc))]
pub struct RUIEventHandlerSystem {
   #[system_desc(event_channel_reader)]
   reader_id: ReaderId<UiEvent>,
   #[system_desc(event_channel_reader)]
   input_event_rid: ReaderId<InputEvent<InputBindingTypes>>,
}

impl RUIEventHandlerSystem {
   pub fn new(reader_id: ReaderId<UiEvent>, input_event_rid: ReaderId<InputEvent<InputBindingTypes>>) -> Self {
       Self { 
          reader_id,
          input_event_rid: input_event_rid,
         }
   }
}

impl<'a> System<'a> for RUIEventHandlerSystem {
   type SystemData = (
       Read<'a, EventChannel<UiEvent>>,
       Read<'a, EventChannel<InputEvent<InputBindingTypes>>>,
       ReadStorage<'a, UiText>,
       Write<'a, MainUI>,
       WriteStorage<'a, Mesh>,
       Write<'a, EventChannel<Command>>,
       Read<'a, InputHandler<InputBindingTypes>>,
    );

   fn run(
      &mut self, 
      (ui_events, input_events, ui_text, mut menu, mesh_storage, mut commands, input): Self::SystemData) {
      
      // update UI to know the current mouse position
      if let Some(mouse_position) = input.mouse_position() {
         menu.mouse_position(mouse_position);
      }
     
      for ev in input_events.read(&mut self.input_event_rid) {
         match (*ev).clone() {
            InputEvent::MouseButtonPressed(Left) => {
               // process grid clicks, e.g. setting active points
               if !menu.submenu_active {
                  menu.grid_click(&mut commands);
               }
            }
            InputEvent::MouseMoved { delta_x, delta_y }  => {
               commands.single_write(Command::MouseMoved(vector(delta_x, delta_y)));
            }
            InputEvent::MouseButtonReleased(Left) => {
               commands.single_write(Command::MouseReleased);
            }
            InputEvent::ActionPressed(action) => {
               if !menu.colour_input_focused {
                  commands.single_write(Command::Input(action));
               }
            }
            _ => {
               //info!("[RUI SYSTEM] You just interacted with a io element: {:?} {:?}", ev, input.mouse_position());
            }
         }
      }
     
      for ev in ui_events.read(&mut self.reader_id) {
         //info!("{:?}", ev);
         if ev.event_type == UiEventType::HoverStart {
            let e = Some(ev.target);
            // handle hover start events for ICONs
            menu.hover(e);
            menu.hover_event(HoverMode::Start, e, &mut commands);
         }
         if ev.event_type == UiEventType::HoverStop {
            // handle hover events for ICONs
            menu.hover(None);
            menu.hover_event(HoverMode::End, Some(ev.target), &mut commands);
         }
         if ev.event_type == UiEventType::Click {
            // handle menu ICON click events
            menu.click(Some(ev.target), &mut commands);
            // we have to defocus colour text input to allow keyboard messages to be sent to sub-system again
            menu.colour_focus(Some(ev.target));
         }
         // handle colour input UI events
         if ev.event_type == UiEventType::ValueCommit {
            menu.colour_input(Some(ev.target), &ui_text, &mut commands);
         }
         // are we focused in the colour 'text' box? need to account for this other wise UI messages are sent
         // through to draw system, when they should not be!
         if ev.event_type ==  UiEventType::Focus {
            menu.colour_focus(Some(ev.target));
         }
      }

      // now we have handled input we call menu update to update vecor UI, 
      // if necessary
      menu.update(mesh_storage);
   }
}