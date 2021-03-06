//! Description: 
//! 
//! External commands processed by vector drawing system
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//! 

use std::fs;

use amethyst::{
    ecs::prelude::{Entity, Entities},
    core::ecs::{Component, DenseVecStorage, WriteStorage, Write},
};

use lyon::{ 
    math::{point, Point, Vector},
    tessellation::{LineCap, LineJoin},
};

extern crate tinyfiledialogs;
//use tinyfiledialogs::{YesNo, MessageBoxIcon, DefaultColorValue};

use palette::{Pixel, Srgb, LinSrgb};

use crate::{
    bindings::{ActionBinding},
    ui::mainui::{MainUI},
}; 
use crate::commands::{
    svg::{SVGEntity, svg_file},
    svg_path::*,
    draw_utils::*,
};

//-----------------------------------------------------------------------------

const DEFAULT_FILE_NAME_JSON: &str = "rebound.json";
const DEFAULT_FILE_NAME_SVG: &str = "rebound.svg";

//-----------------------------------------------------------------------------


/// Hover mouse over ICON modes
#[derive(Clone, Debug, PartialEq)]
pub enum HoverMode {
    Start,
    End,
}

/// Commands sent/generated to draw (command) system from IO interaction
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    /// keyboard/mouse input actions (this include draw actions)
    Input(ActionBinding),
    /// mouse hover mode, for certain action ICONs
    Hover(HoverMode, ActionBinding),
    /// Set draw colour for current layer
    DrawColour([f32;4]),
    /// Add a control point to set of possible points to use in next draw action
    AddControlPoint(Point),
    /// Drag events
    MouseMoved(Vector),
    /// Possible drag finished 
    MouseReleased,
    /// right mouse button clicked
    MouseClickRight,
}   

pub const LAYER_FOREGROUND: usize = 0;
pub const LAYER_MIDDLEGROUND: usize = 1;
pub const LAYER_BACKGROUND: usize = 2;
pub const NUMBER_LAYERS: usize = 3;

/// Data for each independent layer
#[derive(Clone, Debug)]
pub struct Layer {
    pub entities: Vec<SVGEntity>,
    pub colour: [f32;4],
    pub thickness: f32,
    pub linecap: LineCap,
    pub linejoin: LineJoin,
    pub fill: bool,
}

/// default instance for a layer
impl Default for Layer {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            colour: [0.0, 0.0, 0.0, 1.0],
            thickness: 1.0,
            linecap: LineCap::Butt,
            linejoin: LineJoin::Miter,
            fill: false,
        }
    }
}

/// Data for overall drawing
#[derive(Clone, Debug)]
pub struct Draw {
    /// current active layer
    pub active_layer: usize,
    /// points that have been added but not turned into a command
    pub points: Vec<Point>,
    /// data for each layer
    pub layers: [Layer; NUMBER_LAYERS],
    /// is mouse currently hovering an action ICON, if so which one 
    pub hover: Option<ActionBinding>,
    /// control point that is currently being dragged
    pub control_drag: Option<Entity>,
    /// previous actions that undo can apply to (level, number of commands)
    pub undo: Vec<(usize, usize)>,
    /// entites that have been redone (level, svg command)
    pub redo: Vec<(usize, Vec<SVGEntity>)>,
}

/// default instance of draw data
impl Default for Draw {
    fn default() -> Self {
        Self {
            active_layer: LAYER_FOREGROUND,
            points: Vec::new(),
            layers: [Layer::default(), Layer::default(), Layer::default()],
            hover: None,
            control_drag: None,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }
}

impl Draw {
    /// create new instance of draw
    pub fn new() -> Self {
        Self::default()
    }

    /// clear vertices
    pub fn clear_points(&mut self) {
        self.points.clear();
    }

    /// hover action ends
    pub fn hover_end(&mut self) {
        self.hover = None;
    }

    /// hover action begins
    pub fn hover_start(&mut self, action: ActionBinding) {
        self.hover = Some(action);
    }

    /// toggle the fill mode of active layer
    pub fn fill(&mut self) {
        self.layers[self.active_layer].fill = !self.layers[self.active_layer].fill;
    }

    /// is layer mode fill?
    pub fn is_fill(&self, layer: usize) -> bool {
        if layer < NUMBER_LAYERS {
            self.layers[layer].fill
        }
        else {
            false
        }
    }

    /// merge layers into the current layer, adopting active layer colour
    pub fn merge_layers(&mut self) {
        
        let mut tmp: Vec<SVGEntity> = Vec::new();
        for e in self.layers[LAYER_FOREGROUND].entities.iter() {
            tmp.push(e.clone());
        }
        self.layers[LAYER_FOREGROUND].entities.clear();

        for e in self.layers[LAYER_MIDDLEGROUND].entities.iter() {
            tmp.push(e.clone());
        }
        self.layers[LAYER_MIDDLEGROUND].entities.clear();

        for e in self.layers[LAYER_BACKGROUND].entities.iter() {
            tmp.push(e.clone());
        }
        self.layers[LAYER_BACKGROUND].entities.clear();

        // finally push them all on to the active layer
        for e in tmp.iter() {
            self.layers[self.active_layer].entities.push(e.clone());
        }
    }

    /// set active layer to middleground
    pub fn layer_middleground(&mut self) {
        self.active_layer = LAYER_MIDDLEGROUND;
        // clear any existing control points
        self.points.clear();
    }

    /// set active layer to foreground
    pub fn layer_foreground(&mut self) {
        self.active_layer = LAYER_FOREGROUND;
        // clear any existing control points
        self.points.clear();
    }

    /// set active layer to background
    pub fn layer_background(&mut self) {
        self.active_layer = LAYER_BACKGROUND;
        // clear any existing control points
        self.points.clear();
    }

    /// select next linecap
    pub fn linecap(&mut self) {
        if self.layers[self.active_layer].linecap == LineCap::Butt {
            self.layers[self.active_layer].linecap = LineCap::Round;
        }
        else if self.layers[self.active_layer].linecap == LineCap::Round {
            self.layers[self.active_layer].linecap = LineCap::Square;
        }
        else {
            self.layers[self.active_layer].linecap = LineCap::Butt;
        }
    }

    /// select next linejoin
    pub fn linejoin(&mut self) {
        if self.layers[self.active_layer].linejoin == LineJoin::Miter {
            self.layers[self.active_layer].linejoin = LineJoin::MiterClip;
        }
        else if self.layers[self.active_layer].linejoin == LineJoin::MiterClip {
            self.layers[self.active_layer].linejoin = LineJoin::Round;
        }
        else if self.layers[self.active_layer].linejoin == LineJoin::Round {
            self.layers[self.active_layer].linejoin = LineJoin::Bevel;
        }
        else {
            self.layers[self.active_layer].linejoin = LineJoin::Miter;
        }
    }

    /// set draw colour for current layer
    pub fn set_colour(&mut self, colour: &[f32;4]) {
        self.layers[self.active_layer].colour = *colour;
    }

    /// get active layer's colour
    pub fn get_colour(&self) -> [f32;4] {
        self.layers[self.active_layer].colour
    }

    /// increment thinkness for current layer
    pub fn inc_thickness(&mut self, inc: f32) {
        if self.layers[self.active_layer].thickness + inc >= 0.0 {
            self.layers[self.active_layer].thickness  += inc;
        }
    }

    /// begin control point drag, if point hits control point
    /// returns true if point is position of control point on current layer, otherwise false
    pub fn start_drag<'a>(
        &mut self, 
        point: &Point,
        quad_beizer: &WriteStorage<'a, QuadraticBeizer>) -> bool {
        for e in &self.layers[self.active_layer].entities {
            if let Some(entity) = e.entity {
                if let Some(qb) = quad_beizer.get(entity) {
                    if qb.point_c == *point {
                        self.control_drag = Some(entity);
                        return true;
                    }
                }
            }
        }
        false
    }

    /// finish draging control point
    pub fn stop_drag(&mut self) {
        self.control_drag = None;
    }

    /// try and drag control point, only if we are already draging a control point
    pub fn drag<'a>(
        &mut self, 
        diff: &Vector,
        quad_beizer: &mut WriteStorage<'a, QuadraticBeizer>,
        menu: &Write<'a, MainUI>) {
        if let Some(entity) = self.control_drag {
            if let Some(qb) = quad_beizer.get_mut(entity) {
                // normalize point to grid
                let p = qb.point_c + *diff;
                let (x,y) = menu.normalize_mouse_position((p.x, p.y));
                qb.point_c = point(x,y);
            }
        }
    }

    /// add point to the set of points that (might) contrabute to a draw command
    pub fn add_point(&mut self, point: &Point) {
        self.points.push(*point);
    }

    /// use current points to draw line(s)
    pub fn line<'a>(
        &mut self, 
        entities: &Entities<'a>, 
        move_to: &mut WriteStorage<'a, MoveTo>, 
        line_to: &mut WriteStorage<'a, LineTo>) {
        if self.points.len() >= 2 {
            // move_to
            self.layers[self.active_layer].entities.push(
                SVGEntity::new(
                    entities,
                    MoveTo{ point: self.points[0] }, move_to));
            
            for i in 1..self.points.len() {
                // line_to
                self.layers[self.active_layer].entities.push(
                    SVGEntity::new(
                        entities,
                        LineTo{ point: self.points[i] }, line_to));
            }

            // track undo/redo
            println!("p {}", self.points.len());
            self.undo.push((self.active_layer, self.points.len()));
            println!("pp {:?}", self.undo[0]);
            self.redo.clear();
            // clear all the consumed points
            self.points.clear();
        }
    }

    // generate a temorary line path for yet to be added points 
    pub fn hover_line<'a>(&self,) -> Vec<Box<dyn SVGPathPart>> {
        let mut parts: Vec<Box<dyn SVGPathPart>> = Vec::new();
        if self.points.len() >= 2 {
            // move_to
            parts.push(Box::new(MoveTo{ point: self.points[0] }));
            
            for i in 1..self.points.len() {
                // line_to
                parts.push(Box::new(LineTo{ point: self.points[i] }));
            }
        }
        parts
    }

    // generate a temorary line path for yet to be added points 
    pub fn hover_arc<'a>(&self, sweep: bool) -> Vec<Box<dyn SVGPathPart>> {
        let mut parts: Vec<Box<dyn SVGPathPart>> = Vec::new();
        if self.points.len() >= 2 {
            for i in 0..self.points.len()-1 {
                parts.push(Box::new(MoveTo{ point: self.points[i] }));

                let p1 = self.points[i];
                let p2 = self.points[i+1];

                let offset: Point = point((p2.x - p1.x).abs(), (p2.y - p1.y).abs());

                if offset.x == 0. || offset.y == 0. {
                    parts.push(Box::new(LineTo{ point: self.points[i+1] }));
                }
                else {
                    parts.push(
                        Box::new(
                            EllipticalArc { 
                                p1: p1,
                                p2: p2,
                                x_radius: offset.x,
                                y_radius: offset.y,
                                x_axis_rotation: 0.,
                                large_arc: false,
                                sweep: sweep, }));
                }
            }
        }
        parts
    }

    /// generate arc or arc rev
    pub fn arc<'a>(
        &mut self, 
        sweep: bool,
        entities: &Entities<'a>, 
        move_to: &mut WriteStorage<'a, MoveTo>, 
        line_to: &mut WriteStorage<'a, LineTo>,
        arc: &mut WriteStorage<'a, EllipticalArc>) {
       
        if self.points.len() >= 2 {
            for i in 0..self.points.len()-1 {
                self.layers[self.active_layer].entities.push(
                    SVGEntity::new(
                        entities,
                        MoveTo{ point: self.points[i] }, move_to));

                let p1 = self.points[i];
                let p2 = self.points[i+1];

                let offset: Point = point((p2.x - p1.x).abs(), (p2.y - p1.y).abs());

                if offset.x == 0. || offset.y == 0. {
                    self.layers[self.active_layer].entities.push(
                        SVGEntity::new(
                            entities,
                            LineTo{ point: self.points[i+1] }, line_to));
                }
                else {
                    self.layers[self.active_layer].entities.push(
                        SVGEntity::new(
                            entities,
                            EllipticalArc { 
                                p1: p1,
                                p2: p2,
                                x_radius: offset.x,
                                y_radius: offset.y,
                                x_axis_rotation: 0.,
                                large_arc: false,
                                sweep: sweep, }, arc));
                }              
                
            }
            // track undo/redo
            self.undo.push((self.active_layer, (self.points.len()-1) * 2));
            self.redo.clear();

            // clear all the consumed points
            self.points.clear();
        }
    }

    /// try and generate a quadric beizer from current points
    pub fn quad_beizer<'a>(
        &mut self, 
        entities: &Entities<'a>, 
        move_to: &mut WriteStorage<'a, MoveTo>, 
        quad_beizer: &mut WriteStorage<'a, QuadraticBeizer>) {
        let num_points = self.points.len();
        if num_points >= 3 && (num_points + 1) % 2 == 0 {
            // move_to
            self.layers[self.active_layer].entities.push(
                SVGEntity::new(
                    entities,
                    MoveTo{ point: self.points[0] }, move_to));
            // now beizer curves
            for i in (1..self.points.len()).step_by(2) {
                // beizer
                self.layers[self.active_layer].entities.push(
                    SVGEntity::new(
                        entities,
                        QuadraticBeizer {
                            point_c: self.points[i],
                            point_n: self.points[i+1],
                        },
                        quad_beizer));
            }
            // track undo and redo
            self.undo.push((self.active_layer, 1 + (self.points.len()-1) / 2));
            self.redo.clear();
            // clear all the consumed points
            self.points.clear();
        }
    }

    // generate a temorary beizer for yet to be added points 
    pub fn hover_cubic_beizer<'a> (&self) -> Vec<Box<dyn SVGPathPart>> {
        let mut parts: Vec<Box<dyn SVGPathPart>> = Vec::new();
        let num_points = self.points.len();
        if num_points >= 3 && (num_points + 1) % 2 == 0 {
            // move_to
            parts.push(Box::new(MoveTo{ point: self.points[0] }));
            // now beizer curves
            for i in (1..num_points).step_by(2) {
                parts.push(Box::new(QuadraticBeizer { point_c: self.points[i], point_n: self.points[i+1], }));
            }
        }
        parts
    }

    /// add close to active layer path
    pub fn close<'a> ( 
        &mut self, 
        entities: &Entities<'a>, 
        close: &mut WriteStorage<'a, Close>) {
            self.layers[self.active_layer].entities.push(
                SVGEntity::new(
                    entities,
                    Close { }, close));
             // track undo/redo
             self.undo.push((self.active_layer, 1));
             self.redo.clear();
    }

    /// create new drawing
    pub fn clear<'a> (
        &mut self,
        entities: &Entities<'a>) {
            // first clear layers
            for layer in 0..NUMBER_LAYERS {
                // delete any entities
                for e in &self.layers[layer].entities {
                    if let Some(entity) = e.entity {
                        match entities.delete(entity) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                self.layers[layer].entities.clear();
                self.layers[layer].colour = [0.0, 0.0, 0.0, 1.0];
                self.layers[layer].thickness = 1.0;
                self.layers[layer].linecap = LineCap::Butt;
                self.layers[layer].linejoin = LineJoin::Miter;
                self.layers[layer].fill = false;
            }

            // now clear draw itself
            self.points.clear();
            self.active_layer = LAYER_FOREGROUND;
            self.hover = None;
            self.control_drag = None;
            self.undo.clear();
            self.redo.clear();
    }

    /// save to JSON
    pub fn save<'a> (
        &self,
        to_json: bool,
        dimensions: (u32,u32),
        move_to: &WriteStorage<'a, MoveTo>,
        line_to: &WriteStorage<'a, LineTo>,
        quad_beizer: &WriteStorage<'a, QuadraticBeizer>,
        arc: &WriteStorage<'a, EllipticalArc>,
        close: &WriteStorage<'a, Close>) {
        
        // pop up save dialog 
        let default_name = if to_json { DEFAULT_FILE_NAME_JSON } else { DEFAULT_FILE_NAME_SVG };
        match tinyfiledialogs::save_file_dialog("Save", default_name) {
            Some(file) => {
                if to_json {
                    let rebound_file = self.to_file(dimensions, move_to, line_to, quad_beizer, arc, close);
                    if let Ok(json) = rebound_file.to_json() {
                        match fs::write(file, json) {
                            Ok(_) => {}
                            Err(_) => {
                                // TODO: add error dialog box
                            }
                        }
                    }
                    else {
                        // TODO: add error dialog box
                    }
                }
                else {
                    let mut body = "".to_string();
                    for layer in 0..NUMBER_LAYERS {
                        body += &self.to_path(layer, move_to, line_to, quad_beizer, arc, close)[..];
                    }
                    let svg = svg_file(&body[..]);
                    match fs::write(file, svg) {
                        Ok(_) => {}
                        Err(_) => {
                            // TODO: add error dialog box
                        }
                    }
                }
            },
            None => {},
        }
    }

    /// generate arrtibutes for layer
    fn layer_attribute(&self, layer: usize) -> String {
        let pixel: [u8; 3] = Srgb::from_linear(LinSrgb::new(
            self.layers[layer].colour[0], 
            self.layers[layer].colour[1], 
            self.layers[layer].colour[2]))
                .into_format()
                .into_raw();
        let colour = hex::encode(pixel);

        if self.layers[layer].fill { 
            "".to_string() + " fill=\"#" + &colour[..] + "\" " 
        } 
        else { 
            "fill=\"none\"".to_string() +  
            " stroke=\"#" + &colour[..] + "\"" +
            " stroke-linecap=\"" + &format!("{:?}", self.layers[layer].linecap)[..] + "\"" +
            " stroke-linejoin=\"" + &format!("{:?}", self.layers[layer].linejoin)[..] + "\"" +
            " stroke-width=\"" + &(self.layers[layer].thickness as u32).to_string()[..] + "\" " 
        }
    }

    /// convert to svg path 
    fn to_path<'a>(
        &self,
        layer: usize,
        move_to: &WriteStorage<'a, MoveTo>,
        line_to: &WriteStorage<'a, LineTo>,
        quad_beizer: &WriteStorage<'a, QuadraticBeizer>,
        arc: &WriteStorage<'a, EllipticalArc>,
        close: &WriteStorage<'a, Close>) -> String {
            let attr = self.layer_attribute(layer);
            let mut path = "    <path ".to_string() + &attr[..] + "d=\"";
            for e in &self.layers[layer].entities {
                if let Some(entity) = e.entity {
                    if let Some(m_to) = move_to.get(entity) {
                        path += &m_to.gen_output()[..];
                    }
                    else if let Some(l_to) = line_to.get(entity) {
                        path += &l_to.gen_output()[..];
                    }
                    else if let Some(q_beizer) = quad_beizer.get(entity) {
                        path += &q_beizer.gen_output()[..];
                    }
                    else if let Some(close) = close.get(entity) {
                        path += &close.gen_output()[..];
                    }
                    else if let Some(arc) = arc.get(entity) {
                        path += &arc.gen_output()[..];
                    }
                }
            }
            path + "\"/>\n"
    }


    /// convert to internal file format
    fn to_file<'a>(
        &self,
        dimensions: (u32,u32),
        move_to: &WriteStorage<'a, MoveTo>,
        line_to: &WriteStorage<'a, LineTo>,
        quad_beizer: &WriteStorage<'a, QuadraticBeizer>,
        arc: &WriteStorage<'a, EllipticalArc>,
        close: &WriteStorage<'a, Close>) -> ReboundFile {
            let mut rebound_file = ReboundFile {
                settings: Size { width: dimensions.0, height: dimensions.1},
                layers: Vec::new(),
                styles: Vec::new(),
            };

            for layer in 0..NUMBER_LAYERS {
                // create style
                rebound_file.styles.push(Style {
                    thickness: self.layers[layer].thickness,
                    linecap: format!("{:?}", self.layers[layer].linecap),
                    linejoin: format!("{:?}", self.layers[layer].linejoin),
                    colour: self.layers[layer].colour,
                    fill: if self.layers[layer].fill { "fill".to_string() } else { "nofill".to_string() },
                });

                let mut layer_value: Vec<SVGType> = Vec::new();
                for e in &self.layers[layer].entities {
                    if let Some(entity) = e.entity {
                        if let Some(m_to) = move_to.get(entity) {
                            layer_value.push(SVGType {
                                command: MOVETO_NAME.to_string(),
                                vertices: m_to.vertices(),
                            });
                        }
                        else if let Some(l_to) = line_to.get(entity) {
                            layer_value.push(SVGType {
                                command: LINETO_NAME.to_string(),
                                vertices: l_to.vertices(),
                            });
                        }
                        else if let Some(q_beizer) = quad_beizer.get(entity) {
                            layer_value.push(SVGType {
                                command: QUADRATIC_BEIZER_NAME.to_string(),
                                vertices: q_beizer.vertices(),
                            });
                        }
                        else if let Some(close) = close.get(entity) {
                            layer_value.push(SVGType {
                                command: CLOSE_NAME.to_string(),
                                vertices: close.vertices(),
                            });
                        }
                        else if let Some(arc) = arc.get(entity) {
                            layer_value.push(SVGType {
                                command: ELLIPTICAL_ARC_NAME.to_string(),
                                vertices: arc.vertices(),
                            });
                        }
                    }
                }
                rebound_file.layers.push(layer_value);
            }
            rebound_file
    }

    // load from JSON
    pub fn load<'a> (
        &mut self,
        entities: &Entities<'a>,
        move_to: &mut WriteStorage<'a, MoveTo>,
        line_to: &mut WriteStorage<'a, LineTo>,
        quad_beizer: &mut WriteStorage<'a, QuadraticBeizer>,
        arc: &mut WriteStorage<'a, EllipticalArc>,
        close: &mut WriteStorage<'a, Close>) {
        
        // pop up save dialog 
        match tinyfiledialogs::open_file_dialog("Open", DEFAULT_FILE_NAME_JSON, None) {
            Some(file) => {
                if let Ok(data) = fs::read_to_string(file) {
                    if let Ok(rebound) = ReboundFile::from_json(data) {
                        for layer in 0..NUMBER_LAYERS {
                            self.active_layer = layer;
                            // handle types
                            for svgt in &rebound.layers[layer] {
                                match &svgt.command[..] {
                                    MOVETO_NAME => {
                                        self.add_point(&point(svgt.vertices[0].x, svgt.vertices[0].y));
                                    },
                                    LINETO_NAME => {
                                        self.add_point(&point(svgt.vertices[0].x, svgt.vertices[0].y));
                                        self.line(entities, move_to, line_to);
                                    },
                                    QUADRATIC_BEIZER_NAME => {
                                        self.add_point(&point(svgt.vertices[0].x, svgt.vertices[0].y));
                                        self.add_point(&point(svgt.vertices[1].x, svgt.vertices[1].y));
                                        self.quad_beizer(entities, move_to, quad_beizer);
                                    },
                                    CUBIC_BEIZER_NAME => {
                                        // not generated at the moment
                                    },
                                    ELLIPTICAL_ARC_NAME => {
                                        self.add_point(&point(svgt.vertices[0].x, svgt.vertices[0].y));
                                        self.add_point(&point(svgt.vertices[1].x, svgt.vertices[1].y));
                                        self.arc(svgt.vertices[4].y == 1.0, entities, move_to, line_to, arc);
                                    },
                                    CLOSE_NAME => { 
                                        self.close(entities, close);
                                    },
                                    _ => {},
                                }
                            }

                            // handle style
                            self.layers[layer].thickness = rebound.styles[layer].thickness;
                            self.layers[layer].linecap = 
                                    if rebound.styles[layer].linecap == "Butt" { LineCap::Butt } 
                                    else if rebound.styles[layer].linecap == "Round" { LineCap::Round } 
                                    else { LineCap::Square };
                            self.layers[layer].linejoin = match &rebound.styles[layer].linejoin[..] {
                                "Miter" => LineJoin::Miter,
                                "Bevel" => LineJoin::Bevel,
                                "MiterClip" => LineJoin::MiterClip,
                                _ => LineJoin::Round,
                            };
                            self.layers[layer].colour = rebound.styles[layer].colour;
                            self.layers[layer].fill = rebound.styles[layer].fill == "fill";
                        }
                        self.active_layer = LAYER_FOREGROUND;
                    }
                    else {
                        // TODO: add error dialog box
                    }
                }
                else {
                    // TODO: add error dialog box
                }
            },
            None => {},
        }
    }

    /// handle undo command
    pub fn undo(&mut self) {
        if let Some((layer, number)) = self.undo.pop() {
            let mut cmds = Vec::new();
            for _ in 0..number {
                if let Some(entity) = self.layers[layer].entities.pop() {
                    cmds.push(entity); 
                }
            }
            self.redo.push((layer, cmds));
        }
    }

    /// handle redo command
    pub fn redo(&mut self) {
        if let Some((layer, mut cmds)) = self.redo.pop() {
            let len = cmds.len();
            for i in 0..len {
                if let Some(entity) = cmds.pop() {
                    self.layers[layer].entities.push(entity);
                }
            }
            // setup cmds for undo
            self.undo.push((layer, len));
        }
    }
}

impl Component for Draw {
    type Storage = DenseVecStorage<Self>;
}