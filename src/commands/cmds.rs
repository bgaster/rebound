//! Description: 
//! 
//! External commands processed by vector drawing system
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//! 

use amethyst::{
    ecs::prelude::{Entities},
    core::ecs::{Component, DenseVecStorage, WriteStorage},
};

use lyon::{ 
    math::{point, Point},
    tessellation::{LineCap, LineJoin},
};

use crate::bindings::{ActionBinding}; 
use crate::commands::svg::{SVGEntity};
use crate::commands::svg_path::*;

#[derive(Clone, Debug, PartialEq)]
pub enum HoverMode {
    Start,
    End,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Input(ActionBinding),
    Hover(HoverMode, ActionBinding),
    //Hover(HoverMode, ActionBinding),
    DrawColour([f32;4]),
    AddControlPoint(Point),
}   

pub const LAYER_FOREGROUND: usize = 0;
pub const LAYER_MIDDLEGROUND: usize = 1;
pub const LAYER_BACKGROUND: usize = 2;
pub const NUMBER_LAYERS: usize = 3;

#[derive(Clone, Debug)]
pub struct Layer {
    pub entities: Vec<SVGEntity>,
    pub colour: [f32;4],
    pub thickness: f32,
    pub linecap: LineCap,
    pub linejoin: LineJoin,
    pub fill: bool,
}

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

#[derive(Clone, Debug)]
pub struct Draw {
    /// layers
    pub active_layer: usize,
    pub points: Vec<Point>,
    pub layers: [Layer; NUMBER_LAYERS],
    pub hover: Option<ActionBinding>,
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            active_layer: LAYER_FOREGROUND,
            points: Vec::new(),
            layers: [Layer::default(), Layer::default(), Layer::default()],
            hover: None,
        }
    }
}

impl Draw {
    pub fn new() -> Self {
        Self::default()
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
            // clear all the consumed points
            self.points.clear();
        }
    }

    /// try and generate a quadric beizer from current points
    pub fn cubic_beizer<'a>(
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
            // clear all the consumed points
            self.points.clear();
        }
    }

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
    }
}

impl Component for Draw {
    type Storage = DenseVecStorage<Self>;
}