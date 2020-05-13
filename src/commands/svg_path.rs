//! Description: 
//! 
//! Simple SVG path interface.
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
    core::ecs::{Component, DenseVecStorage},
    prelude::*,
};

use amethyst_lyon::{
    utils::{VertexType}
};

use crate::{
    commands::draw_utils::*,
};

extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale, Angle};
use lyon::path::{Path, Builder};
use lyon::geom::{Arc, SvgArc, arc::ArcFlags};
use lyon::tessellation::*;

pub trait SVGPathPart : std::fmt::Debug {
    fn gen_output(&self) -> String;
    fn tessellate(&self, 
        builder: &mut Builder,
        geometry: &mut VertexBuffers<VertexType, u16>);
    fn vertices(&self) -> Vec<Vertex>;
    fn name(&self) -> String;
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

    fn tessellate(&self, builder: &mut Builder, _geometry: &mut VertexBuffers<VertexType, u16>) {
        builder.move_to(self.point);
    }

    fn vertices(&self) -> Vec<Vertex> {
        vec![Vertex::from(self.point)]
    }

    fn name(&self) -> String {
        "move_to".to_string()
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

    fn tessellate(&self, builder: &mut Builder, _geometry: &mut VertexBuffers<VertexType, u16>) {
         builder.line_to(self.point);
    }

    fn vertices(&self) -> Vec<Vertex> {
        vec![Vertex::from(self.point)]
    }

    fn name(&self) -> String {
        "line_to".to_string()
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

    fn tessellate(&self, builder: &mut Builder, mut geometry: &mut VertexBuffers<VertexType, u16>) {
        builder.cubic_bezier_to(self.point_cs, self.point_es, self.point_n);
        // add command point direclty
        let stroke_options = StrokeOptions::tolerance(0.02)
            .with_line_width(1.0)
            .with_line_join(LineJoin::Round)
            .with_line_cap(LineCap::Round);
        let mut tessellator_stroke = StrokeTessellator::new();

        let mut builder = Builder::new();
        circle(&mut builder);
        let scale = Scale::new(3.0);
        let v = vector(self.point_cs.x, self.point_cs.y);
        let p = builder.build();
        tessellator_stroke.tessellate_path(
            &p,
            &stroke_options,
            &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                // scale and then translate
                let pos = scale.transform_point(pos) + v;
                VertexType {
                    position: pos.to_array(),
                    colour: [1.,1.,1.,1.],
                }
            }),
        ).unwrap();

        let mut builder = Builder::new();
        circle(&mut builder);
        let scale = Scale::new(3.0);
        let v = vector(self.point_es.x, self.point_es.y);
        let p = builder.build();
        tessellator_stroke.tessellate_path(
            &p,
            &stroke_options,
            &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                // scale and then translate
                let pos = scale.transform_point(pos) + v;
                VertexType {
                    position: pos.to_array(),
                    colour: [1.,1.,1.,1.],
                }
            }),
        ).unwrap();
    }

    fn vertices(&self) -> Vec<Vertex> {
        vec![Vertex::from(self.point_n), Vertex::from(self.point_cs), Vertex::from(self.point_es)]
    }

    fn name(&self) -> String {
        "cubic_beizer".to_string()
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

    fn tessellate(&self, builder: &mut Builder, mut geometry: &mut VertexBuffers<VertexType, u16>) {
        builder.quadratic_bezier_to(self.point_c, self.point_n);
        // add command point direclty
        let stroke_options = StrokeOptions::tolerance(0.02)
            .with_line_width(1.0)
            .with_line_join(LineJoin::Round)
            .with_line_cap(LineCap::Round);
        let mut tessellator_stroke = StrokeTessellator::new();
        let mut builder = Builder::new();
        circle(&mut builder);
        let scale = Scale::new(3.0);
        let v = vector(self.point_c.x, self.point_c.y);
        let p = builder.build();
        tessellator_stroke.tessellate_path(
            &p,
            &stroke_options,
            &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                // scale and then translate
                let pos = scale.transform_point(pos) + v;
                VertexType {
                    position: pos.to_array(),
                    colour: [1.,1.,1.,1.],
                }
            }),
        ).unwrap();
    }

    fn vertices(&self) -> Vec<Vertex> {
        vec![Vertex::from(self.point_c), Vertex::from(self.point_n)]
    }

    fn name(&self) -> String {
        "quadratic_beizer".to_string()
    }
}

impl Component for QuadraticBeizer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct EllipticalArc {
    pub p1: Point,
    pub p2: Point,
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
            self.x_radius, self.y_radius, self.x_axis_rotation, la, sweep, self.p2.x, self.p2.y)[..]);
        o
    }

    fn tessellate(&self, builder: &mut Builder, _geometry: &mut VertexBuffers<VertexType, u16>) {
        // convert to an Lyon SVG ARC and then to a arc with centre notation, and then finally we can
        // use Lyon to tessellate
        let svg = SvgArc { 
                from: self.p1, 
                to: self.p2, 
                radii: vector(self.x_radius, self.y_radius),
                x_rotation: Angle { radians: self.x_axis_rotation},
                flags: ArcFlags { large_arc: self.large_arc, sweep: self.sweep },
             };

        let mut f = |cb:&lyon::geom::CubicBezierSegment<f32>| {
            builder.move_to(cb.from);
            builder.cubic_bezier_to(cb.ctrl1, cb.ctrl2, cb.to);
        }; 
        svg.for_each_cubic_bezier(&mut f);
    }

    fn vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex::from(self.p1), 
            Vertex::from(self.p2), 
            Vertex { x: self.x_radius, y: self.y_radius }, 
            Vertex { x: self.x_axis_rotation, y: 0.0 },
            Vertex { x: if self.large_arc { 1.0 } else { 0.0 }, y: if self.sweep { 1.0 } else { 0.0 } }]
    }

    fn name(&self) -> String {
        "arc".to_string()
    }
}

impl Component for EllipticalArc {
    type Storage = DenseVecStorage<Self>;
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

    fn tessellate(&self, builder: &mut Builder, _geometry: &mut VertexBuffers<VertexType, u16>) {
        builder.close();
    }

    fn vertices(&self) -> Vec<Vertex> {
        Vec::new()
    }

    fn name(&self) -> String {
        "close".to_string()
    }
}

impl Component for Close {
    type Storage = DenseVecStorage<Self>;
}