use amethyst::{
    ecs::prelude::{WorldExt},
    core::{
        transform::Transform,
    },
    ui::{UiEvent, UiEventType},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
};

use log::{info, error};

use crate:: {
    ui::mainui::{MainUI},
    ui::sub_menu::{
        SubMenuState, dot_menu, file_menu, edit_menu, 
        view_menu, layers_menu, stroke_menu, control_menu, style_menu},
};

#[derive(Default)]
pub struct MainAppState {
    //main_menu: MainUI,
}

impl SimpleState for MainAppState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //let world = data.world;
        let StateData { world, .. } = data;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        //self.main_menu = (*world.read_resource::<MainUI>()).clone();
        world.fetch_mut::<MainUI>().submenu_active = false;

        // Place the camera
        init_camera(world, &dimensions);
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        //self.main_menu = MainUI::default();
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;

        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::HoverStart,
                target,
            }) => {
                let state = Trans::None;
                // if Some(target) == self.main_menu.root {
                // }
                state
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                // handle transition to sub-menu state
                let mut state = Trans::None;
                let mut switch = false;
                if Some(target) == world.fetch::<MainUI>().dot_menu {
                    world.insert(dot_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().file_menu {
                    world.insert(file_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().edit_menu {
                    world.insert(edit_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().view_menu {
                    world.insert(view_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().layers_menu {
                    world.insert(layers_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().stroke_menu {
                    world.insert(stroke_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().control_menu {
                    world.insert(control_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                else if Some(target) == world.fetch::<MainUI>().style_menu {
                    world.insert(style_menu());
                    state = Trans::Switch(Box::new(SubMenuState::default()));
                    switch = true;
                }
                if switch {
                    let mut menu = world.fetch_mut::<MainUI>();
                    menu.submenu_active = true;
                }
                
                state
            }
            StateEvent::Input(input) => {
                match input {
                    _ => { 
                        info!("Unhandled Input Event detected: {:?}.", input);
                    }
                }
                
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, _state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //let StateData { world, .. } = state_data;

        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}