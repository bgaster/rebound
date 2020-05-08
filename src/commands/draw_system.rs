//! Description: 
//! 
//! System for Commands
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
    //derive::SystemDesc,
    core::SystemDesc,
    ecs::{Read, WriteStorage, System, SystemData, ReadStorage, World},
};
  
use amethyst_lyon::{
    utils::{Mesh, VertexType}
 };

extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale};
use lyon::path::{Path, Builder};
use lyon::tessellation::*;
 

use crate::{
    commands::{Draw, NUMBER_LAYERS},
    commands::svg_path::*,
    commands::draw_utils::*,
    vector_meshes::*,
    bindings::{ActionBinding},
};
  
use log::info;
  
pub struct DrawSystem {

}

#[derive(Default, Debug)]
pub struct DrawSystemDesc;


impl<'a, 'b> SystemDesc<'a, 'b, DrawSystem> for DrawSystemDesc {
    fn build(self, world: &mut World) -> DrawSystem {
        <DrawSystem as System<'_>>::SystemData::setup(world);

        //world.insert(self.output.clone());

        DrawSystem{}
    }
}

impl<'a> System<'a> for DrawSystem {
    type SystemData = (
        Read<'a, Draw>,
        ReadStorage<'a, MoveTo>,
        ReadStorage<'a, LineTo>,
        ReadStorage<'a, QuadraticBeizer>,
        WriteStorage<'a, EllipticalArc>,
        ReadStorage<'a, Close>,
        WriteStorage<'a, Mesh>,
        Read<'a, Meshes>,
    );

    fn run(
        &mut self, 
        (draw, move_to, line_to, quad_beizer, arc, close, mut mesh_storage, meshes): Self::SystemData) { 
        let fill_options = FillOptions::default();
        let mut geometry: VertexBuffers<VertexType, u16> = VertexBuffers::new();
        let mut tessellator_stroke = StrokeTessellator::new();
        let mut tessellator_fill = FillTessellator::new();

        // tesselate layers, in reverese order
        for layer in (0..NUMBER_LAYERS).rev() {
            let stroke_options = StrokeOptions::tolerance(0.02)
                .with_line_width(draw.layers[layer].thickness)
                .with_line_join(LineJoin::Round)
                .with_line_cap(draw.layers[layer].linecap);

            let mut builder = Path::builder();

            for e in &draw.layers[layer].entities {
                if let Some(entity) = e.entity {
                    if let Some(m_to) = move_to.get(entity) {
                        m_to.tessellate(
                            &mut builder,
                            &mut geometry);
                        //println!("{}", m_to.gen_output());
                    }
                    else if let Some(l_to) = line_to.get(entity) {
                        l_to.tessellate(&mut builder, &mut geometry);
                        //println!("{}", l_to.gen_output());
                    }
                    else if let Some(q_beizer) = quad_beizer.get(entity) {
                        q_beizer.tessellate(&mut builder, &mut geometry);
                    }
                    else if let Some(close) = close.get(entity) {
                        close.tessellate(&mut builder, &mut geometry);
                    }
                    else if let Some(arc) = arc.get(entity) {
                        arc.tessellate(&mut builder, &mut geometry);
                    }
                }
            }
            
            if draw.is_fill(layer) {
                tessellate_fill(
                    &mut geometry, 
                    &mut tessellator_fill, &fill_options, 
                    builder, 
                    draw.layers[layer].colour);
            }
            else {
                tessellate_stroke(
                    &mut geometry, 
                    &mut tessellator_stroke, &stroke_options, 
                    builder, 
                    draw.layers[layer].colour);
            }
        }

        // renderer any control points
        for cp in &draw.points {
            let mut builder = Path::builder();
            circle(&mut builder);
            let scale = Scale::new(3.0);
            let v = vector(cp.x, cp.y);
            let p = builder.build();
            let stroke_options = StrokeOptions::tolerance(0.02)
                .with_line_width(1.0)
                .with_line_join(LineJoin::Round)
                .with_line_cap(LineCap::Round);
            tessellator_stroke.tessellate_path(
                &p,
                &stroke_options,
                &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                    // scale and then translate
                    let pos = scale.transform_point(pos) + v;
                    VertexType {
                        position: pos.to_array(),
                        colour: [0.,0.,0.,1.0],
                    }
                }),
            ).unwrap();
        }

        // is mouse currently over an icon for hover  
        if let Some(action) = &draw.hover {
            // use a local builder here as we need to use a different colour
            let mut builder = Path::builder();
            let parts = match action {
                ActionBinding::StrokeLine => draw.hover_line(),
                ActionBinding::StrokeArc =>  draw.hover_arc(false),
                ActionBinding::StrokeArcRev => draw.hover_arc(true),
                ActionBinding::StrokeBezier => draw.hover_cubic_beizer(),
                _ => Vec::new()
            };
            for p in parts {
                p.tessellate(&mut builder, &mut geometry); 
            }

            let path = builder.build();
            let stroke_options = StrokeOptions::tolerance(0.02)
                .with_line_width(1.0)
                .with_line_join(LineJoin::Round)
                .with_line_cap(LineCap::Round);
            tessellator_stroke.tessellate_path(
                    &path,
                    &stroke_options,
                    &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
                        // scale and then translate
                        VertexType {
                            position: pos.to_array(),
                            colour: [1.,1.,1.,1.],
                        }
                    }),
                ).unwrap();
        }

        // write generated geometry to mesh
        if let Some(entity) = meshes.cmds {
            if let Some(mesh) = mesh_storage.get_mut(entity) {
               *mesh =  Mesh {
                    vertices: geometry.vertices,
                    indices: geometry.indices,
                    ..Mesh::default()
               };
            }
        }
    }
}

fn tessellate_stroke(
    mut geometry: &mut VertexBuffers<VertexType, u16>, 
    tessellator_stroke: &mut StrokeTessellator,
    stroke_options: &StrokeOptions,
    builder: Builder,
    colour: [f32;4]) {
    let path = builder.build();
    tessellator_stroke.tessellate_path(
        &path,
        stroke_options,
        &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: StrokeAttributes| {
            VertexType {
                position: pos.to_array(),
                colour: colour,
            }
        }),
    ).unwrap();
}

fn tessellate_fill(
    mut geometry: &mut VertexBuffers<VertexType, u16>, 
    tessellator_fill: &mut FillTessellator,
    fill_options: &FillOptions,
    builder: Builder,
    colour: [f32;4]) {
    let path = builder.build();
    tessellator_fill.tessellate_path(
        &path,
        fill_options,
        &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
            VertexType {
                position: pos.to_array(),
                colour: colour,
            }
        }),
    ).unwrap();
}