//! Description: 
//! 
//! Loading state, preloads UI, meshes, and so on.
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//! 
use amethyst::{
    assets::{ProgressCounter},
    ecs::prelude::{Entity},
    prelude::*,
    window::{ScreenDimensions},
    ui::{UiCreator, UiFinder},
    //controls::{HideCursor},
};

use amethyst_lyon::{
    utils::{Mesh}
};

use crate::{
    states::MainAppState,    
    ui::mainui::*,  
    vector_meshes::*,
};

use log::{error};

#[derive(Default)]
pub struct LoadState {
    pub progress_counter: ProgressCounter,
    root: Option<Entity>,
    _mesh: Option<Entity>,
    menu_mesh: Option<Entity>,
    dpi: f64,
}

impl LoadState {
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        self.dpi = dimensions.hidpi_factor();
        // the main menu is always visible so added here
        self.root = world.exec(|mut creator: UiCreator<'_>| {
            if dimensions.hidpi_factor() == 1. {
                Some(creator.create("menu.ron", &mut self.progress_counter))
            }
            else if dimensions.hidpi_factor() == 2. {
                Some(creator.create("menu_2xdpi.ron", &mut self.progress_counter))
            }
            else {
                error!("Only 1 and 2x DPI scalling supported");
                None
            }
        });

        // {
        //     let mut hide_cursor = world.write_resource::<HideCursor>();
        //     hide_cursor.hide = false;

        //     // let window = world.write_resource::<Window>();
        //     // window.set_cursor_position(LogicalPosition::new(100.0, 100.0)).unwrap();
        //     //window.set_cursor_visible(false);
        // }

        self.menu_mesh = Some(world
            .create_entity()
            .with(Mesh::default())
            .build());

        // setup a Mesh for Lyon rendering
        let meshes = Meshes::new(world);
        world.insert(meshes);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // assets loaded?
        if self.progress_counter.is_complete() {
            // create main menu...
            let mut main_menu = MainUI {
                root: self.root,
                mesh: self.menu_mesh,
                dpi: self.dpi,
                ..MainUI::default()
            };

            world.exec(|ui_finder: UiFinder<'_>| {
                main_menu.dot_menu = ui_finder.find(DOT_MENU_ID);
                main_menu.file_menu = ui_finder.find(FILE_MENU_ID);
                main_menu.edit_menu = ui_finder.find(EDIT_MENU_ID);
                main_menu.view_menu = ui_finder.find(VIEW_MENU_ID);
                main_menu.layers_menu = ui_finder.find(LAYERS_MENU_ID);
                main_menu.stroke_menu = ui_finder.find(STROKE_MENU_ID);
                main_menu.control_menu = ui_finder.find(CONTROL_MENU_ID);
                main_menu.style_menu = ui_finder.find(STYLE_MENU_ID);
                main_menu.line_icon = ui_finder.find(LINE_ICON_ID);
                main_menu.arc_icon = ui_finder.find(ARC_ICON_ID);
                main_menu.arc_rev_icon = ui_finder.find(ARC_REV_ICON_ID);
                main_menu.bezier_icon = ui_finder.find(BEZIER_ICON_ID);
                main_menu.close_icon = ui_finder.find(CLOSE_ICON_ID);
                main_menu.linecap_icon = ui_finder.find(LINECAP_ICON_ID);
                main_menu.linejoin_icon = ui_finder.find(LINEJOIN_ICON_ID);
                main_menu.thickness_icon = ui_finder.find(THICKNESS_ICON_ID);
                main_menu.mirror_icon = ui_finder.find(MIRROR_ICON_ID);
                main_menu.fill_icon = ui_finder.find(FILL_ICON_ID);
                main_menu.colour_input = ui_finder.find(COLOUR_INPUT_ID);
            });

            world.insert(main_menu);
        
            Trans::Switch(Box::new(MainAppState::default()))
        } else {
            Trans::None
        }
    }
}