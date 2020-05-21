///
/// Preload assets, before heading into the game states

use amethyst::{
    assets::{ProgressCounter},
    ecs::prelude::{Entity},
    prelude::*,
    shrev::{EventChannel},
    window::ScreenDimensions,
    ui::{UiCreator, UiEvent, UiFinder, UiEventType},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
};

use crate::{
    states::{MainAppState},    
    bindings::{ActionBinding}, 
    commands::{Command},   
};

use log::{error};

#[cfg(not(target_os = "macos"))]
mod const_menus {
    pub const FILE_MENU: &'static str = "file_menu.ron";
    pub const FILE_MENU_2XDPI: &'static str = "file_menu_2xdpi.ron";
    pub const EDIT_MENU: &'static str = "edit_menu.ron";
    pub const EDIT_MENU_2XDPI: &'static str = "edit_menu_2xdpi.ron";
    pub const VIEW_MENU: &'static str = "view_menu.ron";
    pub const VIEW_MENU_2XDPI: &'static str = "view_menu_2xdpi.ron";
    pub const LAYERS_MENU: &'static str = "layers_menu.ron";
    pub const LAYERS_MENU_2XDPI: &'static str = "layers_menu_2xdpi.ron";
    pub const STYLE_MENU: &'static str = "style_menu.ron";
    pub const STYLE_MENU_2XDPI: &'static str = "style_menu_2xdpi.ron";
    pub const STROKE_MENU: &'static str = "stroke_menu.ron";
    pub const STROKE_MENU_2XDPI: &'static str = "stroke_menu_2xdpi.ron";
    pub const CONTROL_MENU: &'static str = "control_menu.ron";
    pub const CONTROL_MENU_2XDPI: &'static str = "control_menu_2xdpi.ron";
}

#[cfg(target_os = "macos")]
mod const_menus {
    pub const FILE_MENU: &'static str = "file_menu_macos.ron";
    pub const FILE_MENU_2XDPI: &'static str = "file_menu_2xdpi_macos.ron";
    pub const EDIT_MENU: &'static str = "edit_menu_macos.ron";
    pub const EDIT_MENU_2XDPI: &'static str = "edit_menu_2xdpi_macos.ron";
    pub const VIEW_MENU: &'static str = "view_menu_macos.ron";
    pub const VIEW_MENU_2XDPI: &'static str = "view_menu_2xdpi_macos.ron";
    pub const LAYERS_MENU: &'static str = "layers_menu_macos.ron";
    pub const LAYERS_MENU_2XDPI: &'static str = "layers_menu_2xdpi_macos.ron";
    pub const STYLE_MENU: &'static str = "style_menu_macos.ron";
    pub const STYLE_MENU_2XDPI: &'static str = "style_menu_2xdpi_macos.ron";
    pub const STROKE_MENU: &'static str = "stroke_menu.ron";
    pub const STROKE_MENU_2XDPI: &'static str = "stroke_menu_2xdpi.ron";
    pub const CONTROL_MENU: &'static str = "control_menu.ron";
    pub const CONTROL_MENU_2XDPI: &'static str = "control_menu_2xdpi.ron";
}

use const_menus::*;

/// 
#[derive(Default)]
pub struct SubMenu {
    /// name of 1 and 2 dpi menu RONs
    pub files: (String, String),
    /// name and action/event for each menu element 
    /// the name must correspond to a button element in the menu RON
    pub names: Vec<(String, ActionBinding)>,
}

impl SubMenu {
    pub fn _new(file_hidpi1: String, file_hidpi2: String) -> Self {
        Self {
            files: (file_hidpi1, file_hidpi2),
            names: Vec::new(),
        }
    }

    /// push a name and action/event for sub menu
    pub fn _push(&mut self, name: String, action: ActionBinding) {
        self.names.push((name, action));
    }
}

/// SubMenu for Dot sub menu
pub fn dot_menu() -> SubMenu {
    SubMenu {
        files: ("dot_menu.ron".to_owned(), "dot_menu_2xdpi.ron".to_owned()),
        names: vec![
            ("open_theme".to_owned(), ActionBinding::OpenTheme),
            ("reset_theme".to_owned(), ActionBinding::ResetTheme),
        ],
    }
}

/// SubMenu for File sub menu
pub fn file_menu() -> SubMenu {
    println!("{}", FILE_MENU);
    SubMenu {
        files: (FILE_MENU.to_string(), FILE_MENU_2XDPI.to_string()),
        names: vec![
            ("file_new".to_owned(), ActionBinding::FileNew),
            ("file_open".to_owned(), ActionBinding::FileOpen),
            ("file_save".to_owned(), ActionBinding::FileSave),
            ("file_export_vector".to_owned(), ActionBinding::FileExportVector),
            ("file_export_image".to_owned(), ActionBinding::FileExportImage),
        ],
    }
}

/// SubMenu for File sub menu
pub fn edit_menu() -> SubMenu {
    SubMenu {
        files: (EDIT_MENU.to_string(), EDIT_MENU_2XDPI.to_string()),
        names: vec![
            ("edit_undo".to_owned(), ActionBinding::EditUndo),
            ("edit_redo".to_owned(), ActionBinding::EditRedo),
        ],
    }
}

/// SubMenu for File sub menu
pub fn view_menu() -> SubMenu {
    SubMenu {
        files: (VIEW_MENU.to_string(), VIEW_MENU_2XDPI.to_string()),
        names: vec![
            ("view_colour_picker".to_owned(), ActionBinding::ViewColourPicker),
            ("view_toggle_grid".to_owned(), ActionBinding::ViewToggleGrid),
            ("view_toggle_tools".to_owned(), ActionBinding::ViewToggleTools),
        ],
    }
}

/// SubMenu for File sub menu
pub fn layers_menu() -> SubMenu {
    SubMenu {
        files: (LAYERS_MENU.to_string(), LAYERS_MENU_2XDPI.to_string()),
        names: vec![
            ("layers_foreground".to_owned(), ActionBinding::LayersForeground),
            ("layers_middleground".to_owned(), ActionBinding::LayersMiddleground),
            ("layers_background".to_owned(), ActionBinding::LayersBackground),
            ("layers_mergelayers".to_owned(), ActionBinding::LayersMergeLayers),
        ],
    }
}

/// SubMenu for Stroke sub menu
pub fn stroke_menu() -> SubMenu {
    SubMenu {
        files: (STROKE_MENU.to_string(), STROKE_MENU_2XDPI.to_string()),
        names: vec![
            ("stroke_line".to_owned(), ActionBinding::StrokeLine),
            ("stroke_arc".to_owned(), ActionBinding::StrokeArc),
            ("stroke_arc_rev".to_owned(), ActionBinding::StrokeArcRev),
            ("stroke_bezier".to_owned(), ActionBinding::StrokeBezier),
            ("stroke_close".to_owned(), ActionBinding::StrokeClose),
            ("stroke_arc_full".to_owned(), ActionBinding::StrokeArcFull),
            ("stroke_arc_rev_full".to_owned(), ActionBinding::StrokeArcRevFull),
            ("stroke_clear_selection".to_owned(), ActionBinding::StrokeClearSelection),
            ("stroke_erase_segment".to_owned(), ActionBinding::StrokeEraseSegment),
        ],
    }
}

/// SubMenu for Control sub menu
pub fn control_menu() -> SubMenu {
    SubMenu {
        files: (CONTROL_MENU.to_string(), CONTROL_MENU_2XDPI.to_string()),
        names: vec![
            ("control_add_point".to_owned(), ActionBinding::ControlAddPoint),
            ("control_move_up".to_owned(), ActionBinding::ControlMoveUp),
            ("control_move_right".to_owned(), ActionBinding::ControlMoveRight),
            ("control_move_down".to_owned(), ActionBinding::ControlMoveDown),
            ("control_move_left".to_owned(), ActionBinding::ControlMoveLeft),
            ("control_remove_point".to_owned(), ActionBinding::ControlRemovePoint),
        ],
    }
}

/// SubMenu for Stroke sub menu
pub fn style_menu() -> SubMenu {
    SubMenu {
        files: (STYLE_MENU.to_string(), STROKE_MENU_2XDPI.to_string()),
        names: vec![
            ("style_line_cap".to_owned(), ActionBinding::StyleLineCap),
            ("style_line_join".to_owned(), ActionBinding::StyleLineJoin),
            ("style_mirror".to_owned(), ActionBinding::StyleMirror),
            ("style_fill".to_owned(), ActionBinding::StyleFill),
            ("style_thinker".to_owned(), ActionBinding::StyleThicker),
            ("style_thinner".to_owned(), ActionBinding::StyleThinner),
            ("style_thicker5".to_owned(), ActionBinding::StyleThicker5),
            ("style_thinner5".to_owned(), ActionBinding::StyleThinner5),
        ],
    }
}


//-----------------------------------------------------------------------------

#[derive(Default)]
pub struct SubMenuState {
    pub progress_counter: ProgressCounter,
    bindings: Vec<(Option<Entity>, ActionBinding)>, 
    root: Option<Entity>,
}

impl SubMenuState {
   
}

impl SimpleState for SubMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let files = (*world.read_resource::<SubMenu>()).files.clone();

        self.root = world.exec(|mut creator: UiCreator<'_>| {
            if dimensions.hidpi_factor() == 1. {
                Some(creator.create(files.0, &mut self.progress_counter))
            }
            else if dimensions.hidpi_factor() == 2. {
                Some(creator.create(files.1, &mut self.progress_counter))
            }
            else {
                error!("Only 1 and 2x DPI scalling supported");
                None
            }
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;

        //ßinfo!("{:?}", event);
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
                // if Some(target) == self.sub_menu {
                //     info!("hover start");
                // }
                state
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                let mut commands = world.write_resource::<EventChannel<Command>>();
                // check if the click corresponded to a menu item
                for b in &self.bindings {
                    if Some(target) == b.0 {
                        // hit menu item, so send its corresponding event
                        commands.single_write(Command::Input(b.1.clone()));
                    }
                }
                // swtich back to main app
                Trans::Switch(Box::new(MainAppState::default()))
            }
            StateEvent::Input(input) => {
                match input {
                    _ => { }
                }
                
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.bindings.clear();
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // add sub menu entities, now once they are loaded
        if self.progress_counter.is_complete() {
            // only do it once... load with a loading state, but does not seem worth it
            if self.bindings.is_empty() {
                let names = (*world.read_resource::<SubMenu>()).names.clone();
                world.exec(|ui_finder: UiFinder<'_>| {            
                    // get entities for each menu element
                    for n in  names {
                        self.bindings.push((ui_finder.find(&n.0), n.1));
                    }
                });
            }
        }

        Trans::None
    }
}