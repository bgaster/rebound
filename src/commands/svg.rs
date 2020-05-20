//! Description: 
//! 
//! Simple SVG interface. Mostly this is a simplified path type, so each layer 
//! can be rerpresented. Functionality, to genenerate a mesh for rendering, 
//! using Lyon, is provided, and outputing an SVG file representing the 3 layers.
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use std::collections::HashMap;

use amethyst::{
    ecs::prelude::{Entity, Entities},
    core::ecs::{Component, DenseVecStorage, WriteStorage},
    prelude::*,
};

use amethyst_lyon::{
    utils::{VertexType}
};

extern crate lyon;
use lyon::path::{Builder};
use lyon::tessellation::*;

use crate::commands::svg_path::*;

const DOC_TYPE: &'static str = "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
const  XMLNS: &'static str = "xmlns=\"http://www.w3.org/2000/svg\"\n     height=\"720mm\" \
viewBox=\"0 0 720 720\" width=\"720mm\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" version=\"1.11.1\" ";
const VERSION: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";

pub fn svg_file(body: &str) -> String {
    VERSION.to_string() + DOC_TYPE + " <svg " + XMLNS + ">\n" + body + "</svg>"
}

pub trait LayerTag {
}

#[derive(Clone, Debug, Default)]
pub struct ForegroundLayer;

impl LayerTag for ForegroundLayer {
}

impl Component for ForegroundLayer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct MiddlegroundLayer;

impl LayerTag for MiddlegroundLayer {
}

impl Component for MiddlegroundLayer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct BackgroundLayer;

impl LayerTag for BackgroundLayer {   
}

impl Component for BackgroundLayer {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Clone, Debug, Default)]
pub struct SVGEntity {
    pub entity: Option<Entity>,
}

impl SVGEntity {
    pub fn new<'a, T>(
        entities: &Entities<'a>,
        svg_part: T, 
        mut svg_parts: &mut WriteStorage<T>) -> Self 
        where T : SVGPathPart + Component {
            let entity = entities.build_entity()
            .with(svg_part, &mut svg_parts)
            .build();
        Self {
            entity: Some(entity),
        }        
    }
}

impl Component for SVGEntity {
    type Storage = DenseVecStorage<Self>;
}

//-----------------------------------------------------------------------------

pub trait SVGElement {
    fn gen_output(&self) -> String;
    fn tessellate(&self, builder: &mut Builder, geometry: &mut VertexBuffers<VertexType, u16>, active_layer: bool);
}

#[derive(Default, Debug)]
pub struct Path {
    pub path: Vec<Box<dyn SVGPathPart>>,
    pub attribs: HashMap<String, String>,
}

impl SVGElement for Path {
    fn gen_output(&self) -> String {
        let mut o = "<path ".to_string();
        for e in &self.path {
            o.push_str(&e.gen_output()[..]);
        }
        o = insert_attribs(o, &self.attribs);
        finalize(o)
    }

    fn tessellate(&self, builder: &mut Builder, geometry: &mut VertexBuffers<VertexType, u16>, active_layer: bool) {
        for e in &self.path {
            e.tessellate(builder, geometry, active_layer);
        }
    }
}

// #[derive(Debug, PartialEq, Clone)]
// pub struct Line {
//     pub p1: (f32, f32),
//     pub p2: (f32, f32),
//     pub attribs: HashMap<String, String>,
//     //pub transform: Option<Transform>
// }

// impl SVGElement for Line {
//     fn gen_output(&self) -> String {
//         let mut o = String::new();
//         o.push_str(&format!("<line x1=\"{:?}\" y1=\"{:?}\" x2=\"{:?}\" y2=\"{:?}\"",
//                            self.p1.0, self.p1.1, self.p2.0, self.p2.1)[..]);
//         o = insert_attribs(o, &self.attribs);
//         finalize(o)
//     }
// }

// #[derive(Debug, PartialEq, Clone)]
// pub struct PolyLine {
//     pub points: Vec<(f32, f32)>,
//     pub attribs: HashMap<String, String>,
//     //pub transform: Option<Transform>
// }

// impl PolyLine {
//     pub fn add_point(&mut self, p: (f32, f32)) {
//         self.points.push(p)
//     }
// }

// impl SVGElement for PolyLine {
//     fn gen_output(&self) -> String {
//         let mut o = String::new();
//         o.push_str(&format!("<polyline {:?}", get_points(&self.points))[..]);
//         o = insert_attribs(o, &self.attribs);
//         finalize(o)
//     }
// }

//-----------------------------------------------------------------------------
// Helper functions
//-----------------------------------------------------------------------------

fn get_points(points: &Vec<(f32, f32)>) -> String {
    let mut p: String = "points=\"".to_string();
    for &(ref x, ref y) in points.iter() {
        p.push_str(&format!("{:?},{:?} ", x, y)[..])
    }
    p.push_str("\"");
    p
}

fn insert_attribs(mut o: String, attribs: &HashMap<String, String>) -> String {
    for (at, value) in attribs.iter() {
        o.push_str(&format!(" {:?}=\"{:?}\"", *at, *value)[..])
    }
    o
}

fn finalize(mut o: String) -> String { 
    o.push_str(" />\n"); 
    o 
}