
use amethyst::{
    ecs::prelude::{Entity, Write, ReadStorage, WriteStorage},
    core::ecs::{Component, DenseVecStorage},
    ui::{UiText},
    shrev::{EventChannel},
};

use amethyst_lyon::{
    utils::{Mesh, VertexType}
 };

 use hex::{FromHex};
 
extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale};
use lyon::path::Path;
use lyon::tessellation::*;

use palette::{Pixel, Srgba};

use crate::{
    bindings::{ActionBinding},
    commands::{Command, HoverMode},
};

extern crate math;
use math::{round};

use log::{info};

/// Names for each of the menu buttons
pub const DOT_MENU_ID: &str = "dot_menu";
pub const FILE_MENU_ID: &str = "file_menu";
pub const EDIT_MENU_ID: &str = "edit_menu";
pub const VIEW_MENU_ID: &str = "view_menu";
pub const LAYERS_MENU_ID: &str = "layers_menu";
pub const STROKE_MENU_ID: &str = "stroke_menu";
pub const CONTROL_MENU_ID: &str = "control_menu";
pub const STYLE_MENU_ID: &str = "style_menu";

// ICON menu

pub const VECTOR_MENU_COLOUR: [f32; 4] = [0.50097686, 0.42692366, 0.58036935, 1.0]; 
pub const VECTOR_MENU_HOVER_COLOUR: [f32; 4] = [0.5721227, 0.3220596, 0.92656577, 1.0];

pub const LINE_ICON_ID: &str = "line_icon";
pub const ARC_ICON_ID: &str = "arc_icon";
pub const ARC_REV_ICON_ID: &str = "arc_rev_icon";
pub const BEZIER_ICON_ID: &str = "bezier_icon";
pub const CLOSE_ICON_ID: &str = "close_icon";
pub const LINECAP_ICON_ID: &str = "linecap_icon";
pub const LINEJOIN_ICON_ID: &str = "linejoin_icon";
pub const THICKNESS_ICON_ID: &str = "thickness_icon";
pub const MIRROR_ICON_ID: &str = "mirror_icon";
pub const FILL_ICON_ID: &str = "fill_icon";
pub const COLOUR_INPUT_ID: &str = "colour_input";

//------------------------------------------------------------------------------------

const GRID_X_OFFSET: f32 = 20.;
const GRID_Y_OFFSET: f32 = 40.0;
const GRID_SPACE: f32    = 10.0;
const GRID_SPACE_DIV_2: usize = 5;  
const WINDOW_WIDTH: f32 = 720.;
const WINDOW_HEIGHT: f32 = 720.;
const GRID_DOT_COLOUR: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const MOUSE_CURSOR_COLOUR: [f32;4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Clone)]
pub struct MainUI {
    // sub menus
    pub submenu_active: bool,
    pub dot_menu: Option<Entity>,
    pub file_menu: Option<Entity>,
    pub edit_menu: Option<Entity>,
    pub view_menu: Option<Entity>,
    pub layers_menu: Option<Entity>,
    pub stroke_menu: Option<Entity>,
    pub control_menu: Option<Entity>,
    pub style_menu: Option<Entity>,
    pub root: Option<Entity>,
    
    // now handle the ICON menu at the bottom of window
    
    /// Mesh that ICONs are tesselated to and then rendered
    pub mesh: Option<Entity>,

    /// Entity for line icon (button)
    pub line_icon: Option<Entity>,
    /// Entity for arc icon (button)
    pub arc_icon: Option<Entity>,
    /// Entity for arc icon (button)
    pub arc_rev_icon: Option<Entity>,
    /// Entity for bezier icon (button)
    pub bezier_icon: Option<Entity>,
    /// Entity for close icon (button)
    pub close_icon: Option<Entity>,
    /// Entity for line cap icon (button)
    pub linecap_icon: Option<Entity>,
    /// Entity for line join icon (button)
    pub linejoin_icon: Option<Entity>,
    /// Entity for thinkness icon (button)
    pub thickness_icon: Option<Entity>,
    /// Entity for mirror icon (button)
    pub mirror_icon: Option<Entity>,
    /// Entity for fill icon (button)
    pub fill_icon: Option<Entity>,
    /// Icon currently being hovered over (if any)
    pub hover: Option<Entity>,
    /// Entity for fill icon (button)
    pub colour_input: Option<Entity>,
    /// is grid displayed
    pub display_grid: bool,
    /// is tools displayed
    pub display_tools: bool,
    pub mouse_position: (f32,f32),
    pub dpi: f64,
    /// current draw colour, displayed in UI
    pub draw_colour: [f32;4],
}

impl Default for MainUI {
    fn default() -> Self {
        MainUI {
            submenu_active: false,
            dot_menu: None,
            file_menu: None,
            edit_menu: None,
            view_menu: None,
            layers_menu: None,
            stroke_menu: None,
            control_menu: None,
            style_menu: None,
            root: None,
            mesh: None,
            line_icon: None,
            arc_icon: None,
            arc_rev_icon: None,
            bezier_icon: None,
            close_icon: None,
            linecap_icon: None,
            linejoin_icon: None,
            thickness_icon: None,
            mirror_icon: None,
            fill_icon: None,
            hover: None,
            colour_input: None,
            display_grid: true,
            display_tools: true,
            mouse_position: (0.,0.),
            dpi: 0.,
            draw_colour: [0.0,0.0,0.0,1.0],
        }
    }
}

impl MainUI {
    /// set current mouse position, input not normalized for DPI (due to Amethyst UI not handling DPI... ah)
    /// this is due to the fact that Amethyst does not handle DPI
    pub fn mouse_position(&mut self, pos: (f32,f32)) {
        // as the mouse position is intended to follow the grid, we clamp
        // so that it is.
        let (mut x, mut y) =  (
            round::half_to_even(pos.0 as f64 / self.dpi, -1) as usize, 
            round::half_to_even(pos.1 as f64 / self.dpi, -1) as usize);
        
        let r = x % GRID_SPACE as usize;
        if  r != 0 {
            if r < GRID_SPACE_DIV_2 {
                x = x - r;
            }
            else {
                x = x + r;
            }
        }
        let r = y % GRID_SPACE as usize;
        if  r != 0 {
            if r < GRID_SPACE_DIV_2 {
                y = y - r;
            }
            else {
                y = y + r;
            }
        }
        self.mouse_position = (x as f32, y as f32);
    }

    /// set current dpi
    pub fn _set_dpi(&mut self, dpi: f64) {
        self.dpi = dpi;
    }

    /// set entity currently being hovered over, which might be none
    pub fn hover(&mut self, entity: Option<Entity>) {
        self.hover = entity;
    }

    /// toggle grid display
    pub fn toggle_grid(&mut self) {
        self.display_grid = !self.display_grid;
    }

    /// toggle grid display
    pub fn toggle_tools(&mut self) {
        self.display_tools = !self.display_grid;
    }

    /// set the current draw colour, taken from user input within the UI
    /// input is CSS hex colour and is converted to GFX/Vulkan linear space
    /// 
    /// if colour is valid, then send out on the bus
    pub fn colour_input<'a>(
        &mut self, entity: Option<Entity>, 
        ui_text: &ReadStorage<'a, UiText>, 
        commands: &mut Write<'a,  EventChannel::<Command>>)  {
        if self.colour_input == entity {
            if let Some(colour_input) = self.colour_input.and_then(|entity| ui_text.get(entity)) {
                // decode hex and then encode CSS RGB into linear space for GFX/Vulkan 
                let decoded = <[u8; 3]>::from_hex(colour_input.text.clone());
                match decoded {
                    Ok(colour) => {
                        let colour = Srgba::new(
                            colour[0] as f32 / 255., 
                            colour[1] as f32 / 255., 
                            colour[2] as f32 / 255., 
                            1.0).into_linear();

                        // store colour to use when draw ui
                        self.draw_colour = colour
                            .into_format()
                            .into_raw();

                        // finally send out a draw colour change message on the bus
                        commands.single_write(Command::DrawColour(self.draw_colour));
                    }
                    Err(_) => {}
                }
            }
        }
    }

    /// set the current draw colour
    pub fn set_colour(&mut self, colour: [f32;4]) {
        self.draw_colour = colour;
    }

    /// Is one of the ICONs clicked? if so, then send corresponding message out on bus
    pub fn hover_event<'a>(
        &self, 
        mode: HoverMode,
        entity: Option<Entity>, 
        commands: &mut Write<'a,  EventChannel::<Command>>) {

        if entity == self.line_icon {    
            commands.single_write(Command::Hover(mode, ActionBinding::StrokeLine));
        }
        else if entity == self.arc_icon {
            commands.single_write(Command::Hover(mode, ActionBinding::StrokeArc));
        }
        else if entity == self.arc_rev_icon {
            commands.single_write(Command::Hover(mode, ActionBinding::StrokeArcRev));
        }
        else if entity == self.bezier_icon {
            commands.single_write(Command::Hover(mode, ActionBinding::StrokeBezier));
        }
    }

    /// Is one of the ICONs clicked? if so, then send corresponding message out on bus
    pub fn click<'a>(&self, entity: Option<Entity>, commands: &mut Write<'a,  EventChannel::<Command>>) {
        
        if entity == self.line_icon {
            commands.single_write(Command::Input(ActionBinding::StrokeLine));
        }
        else if entity == self.arc_icon {
            commands.single_write(Command::Input(ActionBinding::StrokeArc));
        }
        else if entity == self.arc_rev_icon {
            commands.single_write(Command::Input(ActionBinding::StrokeArcRev));
        }
        else if entity == self.bezier_icon {
            commands.single_write(Command::Input(ActionBinding::StrokeBezier));
        }
        else if entity == self.close_icon {
            commands.single_write(Command::Input(ActionBinding::StrokeClose));
        }
        else if entity == self.linecap_icon {
            commands.single_write(Command::Input(ActionBinding::StyleLineCap));
        }
        else if entity == self.linejoin_icon {
            commands.single_write(Command::Input(ActionBinding::StyleLineJoin));
        }
        else if entity == self.thickness_icon {
            commands.single_write(Command::Input(ActionBinding::StyleThicker));
        }
        else if entity == self.mirror_icon {
            commands.single_write(Command::Input(ActionBinding::StyleMirror));
        }
        else if entity == self.fill_icon {
            commands.single_write(Command::Input(ActionBinding::StyleFill));
        }
    }

    /// Update the mesh for drawn menu (at bottom of window)
    /// This needs to only be called if colour state for an icon has changed
    pub fn update<'a>(&self, mut meshes: WriteStorage<'a, Mesh>) {

        let mut stroke_options = StrokeOptions::tolerance(0.02)
            .with_line_width(4.0)
            .with_line_join(LineJoin::Round)
            .with_line_cap(LineCap::Round);
        let fill_options = FillOptions::default();
        let mut geometry: VertexBuffers<VertexType, u16> = VertexBuffers::new();
        let mut tessellator_stroke = StrokeTessellator::new();
        let mut tessellator_fill = FillTessellator::new();

        // tessellate each icon
        if self.display_tools {
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options,
                &mut tessellator_fill, &fill_options,
                self.line_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.arc_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.arc_rev_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.bezier_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.close_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.linecap_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.linejoin_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.thickness_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.mirror_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.fill_icon_render());
            Self::tessellate(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                &mut tessellator_fill, &fill_options,
                self.colour_icon_render());
        }
        
        // handle display grid
        self.tessellate_grid(
            &mut geometry, 
            &mut tessellator_stroke, &stroke_options, 
            &mut tessellator_fill, &fill_options);

         // handle display grid
         self.tessellate_mouse(
            &mut geometry, 
            &mut tessellator_stroke, &mut stroke_options);
        
        if let Some(entity) = self.mesh {
            if let Some(mesh) = meshes.get_mut(entity) {
               *mesh =  Mesh {
                    vertices: geometry.vertices,
                    indices: geometry.indices,
                    ..Mesh::default()
               };
            }
        }
    }

    /// Deterine if colour to draw for a given ICON, depending on if it is currently 
    /// in hover mode or not
    fn colour(&self, entity: Option<Entity>) -> [f32;4] {
        if self.hover == entity {
            VECTOR_MENU_HOVER_COLOUR
        } 
        else {
            VECTOR_MENU_COLOUR
        }
    }

    fn tessellate(
        mut geometry: &mut VertexBuffers<VertexType, u16>, 
        tessellator_stroke: &mut StrokeTessellator,
        stroke_options: &StrokeOptions,
        tessellator_fill: &mut FillTessellator,
        fill_options: &FillOptions,
        path_colour: (Path, [f32;4], bool)) {
        if !path_colour.2 {
            tessellator_stroke.tessellate_path(
                &path_colour.0,
                stroke_options,
                &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                    VertexType {
                        position: pos.to_array(),
                        colour: path_colour.1,
                    }
                }),
            ).unwrap();
        }
        else {
            tessellator_fill.tessellate_path(
                &path_colour.0,
                fill_options,
                &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
                    VertexType {
                        position: pos.to_array(),
                        colour: path_colour.1,
                    }
                }),
            ).unwrap();
        }
    }

    fn tessellate_mouse(
        &self,
        mut geometry: &mut VertexBuffers<VertexType, u16>, 
        tessellator_stroke: &mut StrokeTessellator,
        _stroke_options: &mut StrokeOptions) {
            // let (size_x, size_y) = Self::grid_size_cells();
            // // only draw mouse cursor if actually over the grid
            // if self.mouse_position.0 >= GRID_X_OFFSET && self.mouse_position.0 <= size_x as f32 &&
            //    self.mouse_position.1 >= GRID_Y_OFFSET && self.mouse_position.1 <= size_y as f32 {
            if self.in_grid() {
                let v: Vector = vector(self.mouse_position.0, self.mouse_position.1);
                let s: Scale  = Scale::new(7.);
                let p = Self::circle();

                let stroke_options = StrokeOptions::tolerance(0.02)
                    .with_line_width(0.25);

                tessellator_stroke.tessellate_path(
                    &p,
                    &stroke_options,
                    &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                        // scale and translate
                        let pos = s.transform_point(pos) + v;
                        VertexType {
                            position: pos.to_array(),
                            colour: MOUSE_CURSOR_COLOUR,
                        }
                    }),
                ).unwrap();
            }
    }

    fn grid_num_cells() -> (usize, usize) {
        ((WINDOW_WIDTH as usize - GRID_X_OFFSET as usize *2) / GRID_SPACE as usize, 
         (WINDOW_HEIGHT as usize - GRID_Y_OFFSET as usize *2) / GRID_SPACE as usize)
    }

    fn grid_size_cells() -> (usize, usize) {
        let (x,y) = Self::grid_num_cells();
        (x*GRID_SPACE as usize, y*GRID_SPACE as usize)
    }

    /// is mouse cursor currently within grid?
    pub fn in_grid(&self) -> bool {
        let (size_x, size_y) = Self::grid_size_cells();
        self.mouse_position.0 >= GRID_X_OFFSET && self.mouse_position.0 <= size_x as f32 &&
               self.mouse_position.1 >= GRID_Y_OFFSET && self.mouse_position.1 <= size_y as f32
    }

    /// handle grid click events
    pub fn grid_click<'a>(&mut self, commands: &mut Write<'a,  EventChannel::<Command>>) {
        // check event within grid
        if self.in_grid() {
            commands.single_write(Command::AddControlPoint(point(self.mouse_position.0, self.mouse_position.1)));       
        }
    }

    fn tessellate_grid(
        &self,
        mut geometry: &mut VertexBuffers<VertexType, u16>, 
        _tessellator_stroke: &mut StrokeTessellator,
        _stroke_options: &StrokeOptions,
        tessellator_fill: &mut FillTessellator,
        fill_options: &FillOptions) {

        if self.display_grid {
            let scale_big = Scale::new(2.0);
            let scale_small = Scale::new(1.0);

            let (num_cells_x, num_cells_y) = Self::grid_num_cells();

            for y in 0..num_cells_y {
                for x in 0..num_cells_x {
                    let v: Vector = vector(
                        GRID_X_OFFSET + x as f32 * GRID_SPACE, GRID_Y_OFFSET + y as f32 * GRID_SPACE);

                    let s:Scale = if y % 5 == 0 && x % 5 == 0 {
                        scale_big
                    }
                    else {
                        scale_small
                    };

                    let p = Self::circle();

                    tessellator_fill.tessellate_path(
                        &p,
                        fill_options,
                        &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
                            // scale and then translate
                            let pos = s.transform_point(pos) + v;
                            VertexType {
                                position: pos.to_array(),
                                colour: GRID_DOT_COLOUR,
                            }
                        }),
                    ).unwrap();
                }
            }
        }
        
    }

    fn line_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();
        builder.move_to(point(26., 687.8));
        builder.line_to(point(50., 711.2));

        (builder.build(), self.colour(self.line_icon), false)
    }

    fn arc_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(57.8,  687.8));
        builder.cubic_bezier_to(
            point(70.72346314604057, 687.8), 
            point(81.2, 698.2765368539594), 
            point(81.2, 711.2));

        (builder.build(), self.colour(self.arc_icon), false)
    }

    fn arc_rev_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(97.8, 687.8));
        builder.cubic_bezier_to(
            point(97.80000000000001, 700.7234631460406), 
            point(108.27653685395944, 711.2), 
            point(121.2, 711.2));
        
        (builder.build(), self.colour(self.arc_rev_icon), false)
    }

    fn bezier_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();
        
        builder.move_to(point(128.8, 687.8));
        builder.quadratic_bezier_to(point(128.8, 699.5), point(140.5, 699.5));
        builder.quadratic_bezier_to(point(152.2, 699.5), point(152.2, 711.2));

        (builder.build(), self.colour(self.bezier_icon), false) 
    }

    fn close_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();
        
        builder.move_to(point(159.8, 687.8));
        builder.cubic_bezier_to(
            point(172.72346314604056, 687.8), 
            point(183.2, 698.2765368539594), 
            point(183.2, 711.2));
            
        builder.move_to(point(159.8, 687.8));
        builder.cubic_bezier_to(
            point(159.8, 700.7234631460406 ), 
            point(170.27653685395944, 711.2), 
            point(183.2, 711.2));

        (builder.build(), self.colour(self.close_icon), false) 
    }

    fn linecap_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(191.8, 687.8));
        builder.line_to(point(191.8, 687.8));
        builder.line_to(point(207.4, 703.4));
        builder.line_to(point(215.2, 703.4));
        builder.line_to(point(215.2, 711.2));
        builder.line_to(point(207.4, 711.2));
        builder.line_to(point(207.4, 703.4));

        (builder.build(), self.colour(self.linecap_icon), false) 
    }

    fn linejoin_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(223.8, 687.8));
        builder.line_to(point(231.6, 695.6));
        builder.line_to(point(239.4, 695.6));

        builder.move_to(point(231.6, 703.4));
        builder.line_to(point(239.4, 703.4));
        builder.line_to(point(247.2, 711.2));

        (builder.build(), self.colour(self.linejoin_icon), false) 
    }

    fn thickness_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(265.6, 691.7));
        builder.line_to(point(265.6, 691.7));
        builder.line_to(point(261.7, 695.6));
        builder.line_to(point(273.4, 707.3));
        builder.line_to(point(277.3, 703.4));
        builder.close();

        builder.move_to(point(263.65, 693.65));
        builder.line_to(point(263.65, 693.65));
        builder.line_to(point(257.8, 687.8));
     
        builder.move_to(point(263.65, 693.65));
        builder.line_to(point(263.65, 693.65));
        builder.line_to(point(257.8, 687.8));

        builder.move_to(point(275.35, 705.35));
        builder.line_to(point(275.35, 705.35));
        builder.line_to(point(281.2, 711.2));
     
        (builder.build(), self.colour(self.thickness_icon), false) 
    }

    fn mirror_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();

        builder.move_to(point(289.8, 687.8));
        builder.line_to(point(289.8, 687.8));
        builder.line_to(point(297.6, 695.6));

        builder.move_to(point(305.4, 703.4));
        builder.line_to(point(305.4, 703.4));
        builder.line_to(point(313.2, 711.2));

        builder.move_to(point(309.3, 691.7));
        builder.line_to(point(309.3, 691.7));
        builder.line_to(point(305.4, 695.6));

        builder.move_to(point(297.6, 703.4));
        builder.line_to(point(297.6, 703.4));
        builder.line_to(point(293.7, 707.3));
     
        (builder.build(), self.colour(self.mirror_icon), false) 
    }

    fn fill_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();
     
        builder.move_to(point(319.8, 687.8));
        builder.line_to(point(319.8, 699.5));
        builder.line_to(point(331.5, 699.5));
        builder.line_to(point(343.2, 699.5));
        builder.line_to(point(343.2, 711.2));
        builder.close();
        
        (builder.build(), self.colour(self.fill_icon), false) 
    }

    fn colour_icon_render(&self) -> (Path, [f32;4], bool) {
        let mut builder = Path::builder();
        
        builder.move_to(point(375.0000,700.0000));
        builder.cubic_bezier_to(point(375.0000, 702.7615), point(374.0237, 705.1185), point(372.0711, 707.0710));
        builder.cubic_bezier_to(point(370.1185, 709.0237), point(367.7615, 710.0000), point(365.0000, 710.0000));
        builder.cubic_bezier_to(point(362.2367, 710.0000), point(359.8796, 709.0237), point(357.9289, 707.0710));
        builder.cubic_bezier_to(point(355.9763, 705.1203), point(355.0000, 702.7634), point(355.0000, 700.0000));
        builder.cubic_bezier_to(point(355.0000, 698.2146), point(355.4465, 696.5479), point(356.3398, 695.0000));
        builder.cubic_bezier_to(point(357.2340, 693.4519), point(358.4541, 692.2318), point(360.0000, 691.3397));
        builder.cubic_bezier_to(point(361.5462, 690.4465), point(363.2129, 690.0000), point(365.0000, 690.0000));
        builder.cubic_bezier_to(point(367.7610, 690.0000), point(370.1180, 690.9763), point(372.0711, 692.9290));
        builder.cubic_bezier_to(point(374.0237, 694.8820), point(375.0000, 697.2391), point(375.0000, 700.0000));
        builder.close();

        (builder.build(), self.draw_colour, true)
    }

    /// circle path c:(0,0) r:1.0 (which can be translated and scaled as necessary)
    fn circle() -> Path {
        let mut builder = Path::builder();

        let p1 = point(1.0000,0.0000) ;
        let (p2, p3, p4) = (point(1.0000, 0.2761), point(0.9024, 0.5118), point(0.7071, 0.7071));
        let (p5, p6, p7) = (point(0.5118, 0.9024), point(0.2761, 1.0000), point(-0.0000, 1.0000));
        let (p8, p9, p10) = (point(-0.1786, 1.0000), point(-0.3453, 0.9553), point(-0.5000, 0.8660));
        let (p11, p12, p13) = (point(-0.6547, 0.7767), point(-0.7767, 0.6547), point(-0.8660, 0.5000));
        let (p14, p15, p16) = (point(-0.9553, 0.3453), point(-1.0000, 0.1786), point(-1.0000, -0.0000));
        let (p17, p18, p19) = (point(-1.0000, -0.2761), point(-0.9024, -0.5118), point(-0.7071, -0.7071));
        let (p20, p21, p22) = (point(-0.5118, -0.9024), point(-0.2761, -1.0000), point(-0.0000, -1.0000));
        let (p23, p24, p25) = (point(0.1786, -1.0000), point(0.3453, -0.9553), point(0.5000, -0.8660));
        let (p26, p27, p28) = (point(0.6547, -0.7767), point(0.7767, -0.6547), point(0.8660, -0.5000));
        let (p29, p30, p31) = (point(0.9553, -0.3453), point(1.0000, -0.1786), point(1.0000, 0.0000));

        builder.move_to(p1);
        builder.cubic_bezier_to(p2, p3, p4);
        builder.cubic_bezier_to(p5, p6, p7);
        builder.cubic_bezier_to(p8, p9, p10);
        builder.cubic_bezier_to(p11, p12, p13);
        builder.cubic_bezier_to(p14, p15, p16);
        builder.cubic_bezier_to(p17, p18, p19);
        builder.cubic_bezier_to(p20, p21, p22);
        builder.cubic_bezier_to(p23, p24, p25);
        builder.cubic_bezier_to(p26, p27, p28);
        builder.cubic_bezier_to(p29, p30, p31);
        builder.close();

        builder.build()
    }
}


impl Component for MainUI {
    type Storage = DenseVecStorage<Self>;
}