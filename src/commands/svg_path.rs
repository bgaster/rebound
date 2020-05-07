//! Description: 
//! 
//! Simple SVG path interface.
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
    core::ecs::{Component, DenseVecStorage},
    prelude::*,
};

extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale};
use lyon::path::{Path, Builder};
use lyon::tessellation::*;

pub trait SVGPathPart : std::fmt::Debug {
    fn gen_output(&self) -> String;
    fn tessellate(&self, builder: &mut Builder);
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MoveTo {
    pub point: Point,
}

impl SVGPathPart for MoveTo {
     fn gen_output(&self) -> String {
        let mut o = String::new();
        o.push_str(&format!("M{:?},{:?}", self.point.x, self.point.y)[..]);
        o
     }

     fn tessellate(&self, builder: &mut Builder) {
        builder.move_to(self.point);
     }
}

impl Component for MoveTo {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct LineTo {
    pub point: Point,
}

impl SVGPathPart for LineTo {
     fn gen_output(&self) -> String {
        let mut o = String::new();
        o.push_str(&format!("L{:?},{:?}", self.point.x, self.point.y)[..]);
        o
     }

    fn tessellate(&self, builder: &mut Builder) {
         builder.line_to(self.point);
    }
}

impl Component for LineTo {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct CubicBeizer {
    pub point_n: Point,
    pub point_cs: Point,
    pub point_es: Point,
}

impl SVGPathPart for CubicBeizer {
    fn gen_output(&self) -> String {
        let mut o = String::new();
        o.push_str(&format!("C{:?},{:?} {:?},{:?} {:?},{:?}", 
            self.point_cs.x, self.point_cs.y,
            self.point_es.x, self.point_es.y,
            self.point_n.x, self.point_n.y,)[..]);
        o
     }

    fn tessellate(&self, builder: &mut Builder) {
        builder.cubic_bezier_to(self.point_cs, self.point_es, self.point_n);
    }
}

impl Component for CubicBeizer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct QuadraticBeizer {
    pub point_c: Point,
    pub point_n: Point,
}

impl SVGPathPart for QuadraticBeizer {
    fn gen_output(&self) -> String {
        let mut o = String::new();
        o.push_str(&format!("Q{:?},{:?} {:?},{:?}", 
            self.point_c.x, self.point_c.y,
            self.point_n.x, self.point_n.y,)[..]);
        o
    }

    fn tessellate(&self, builder: &mut Builder) {
        builder.quadratic_bezier_to(self.point_c, self.point_n);
    }
}

impl Component for QuadraticBeizer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct EllipticalArc {
    pub point: Point,
    pub x_radius: f32,
    pub y_radius: f32,
    pub x_axis_rotation: f32,
    pub large_arc: bool,
    pub sweep: bool,
}

impl SVGPathPart for EllipticalArc {
    fn gen_output(&self) -> String {
        let la = if self.large_arc { 1 } else { 0 };
        let sweep = if self.sweep { 1 } else { 0 };

        let mut o = String::new();
        o.push_str(&format!("A{} {} {} {} {} {},{} ", 
            self.x_radius, self.y_radius, self.x_axis_rotation, la, sweep, self.point.x, self.point.y)[..]);
        o
    }

    fn tessellate(&self, builder: &mut Builder) {
         //builder.arc(self.point, radii: Vector, sweep_angle: Angle, x_rotation: Angle)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Close {
}

impl SVGPathPart for Close {
     fn gen_output(&self) -> String {
        let mut o = String::new();
        o.push_str(&format!("Z ",)[..]);
        o
     }

     fn tessellate(&self, builder: &mut Builder) {
        builder.close();
     }
}

impl Component for Close {
    type Storage = DenseVecStorage<Self>;
}