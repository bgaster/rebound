//! Description: 
//! 
//! External commands processed by vector drawing system
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//! 
use serde::{Serialize, Deserialize};

use amethyst::{
    ecs::prelude::{Entity, Entities},
    core::ecs::{Component, DenseVecStorage, WriteStorage},
    prelude::*,
};

use lyon::math::{point, Point, Vector, vector, Scale};

use crate::bindings::{ActionBinding}; 
use crate::commands::svg::{SVGEntity, ForegroundLayer, BackgroundLayer, MiddlegroundLayer, LayerTag};
use crate::commands::svg_path::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Input(ActionBinding),
    DrawColour([f32;4]),
    AddControlPoint(Point),
}   

pub const LAYER_FOREGROUND: usize = 0;
pub const LAYER_MIDDLEGROUND: usize = 0;
pub const LAYER_BACKGROUND: usize = 0;
pub const NUMBER_LAYERS: usize = 3;

#[derive(Clone, Debug)]
pub struct Layer {
    pub entities: Vec<SVGEntity>,
    pub colour: [f32;4],
    pub thinkness: f32,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            colour: [0.0, 0.0, 0.0, 1.0],
            thinkness: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Draw {
    /// layers
    pub active_layer: usize,
    pub points: Vec<Point>,
    pub layers: [Layer; NUMBER_LAYERS],
}

impl Default for Draw {
    fn default() -> Self {
        Self {
            active_layer: LAYER_FOREGROUND,
            points: Vec::new(),
            layers: [Layer::default(), Layer::default(), Layer::default()],
        }
    }
}

impl Draw {
    pub fn new() -> Self {
        Self::default()
    }

    /// set active layer to middleground
    pub fn layer_middleground(&mut self) {
        self.active_layer = LAYER_MIDDLEGROUND;
    }

    /// set active layer to foreground
    pub fn layer_foreground(&mut self) {
        self.active_layer = LAYER_FOREGROUND;
    }

    /// set active layer to background
    pub fn layer_background(&mut self) {
        self.active_layer = LAYER_BACKGROUND;
    }

    /// set draw colour for current layer
    pub fn set_colour(&mut self, colour: &[f32;4]) {
        self.layers[self.active_layer].colour = *colour;
    }

    /// increment thinkness for current layer
    pub fn inc_thinkness(&mut self, inc: f32) {
        if self.layers[self.active_layer].thinkness + inc >= 0.0 {
            self.layers[self.active_layer].thinkness  += inc;
        }
    }

    pub fn add_point(&mut self, point: &Point) {
        self.points.push(*point);
    }

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
        }
        // clear all the consumed points
        self.points.clear();
    }
}

impl Component for Draw {
    type Storage = DenseVecStorage<Self>;
}