//! Description: 
//! 
//! System for Commands
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{
    //derive::SystemDesc,
    core::SystemDesc,
    ecs::{Read, Write, WriteStorage, System, SystemData, Entities, ReadStorage, World, Join},
    shrev::{EventChannel, ReaderId},
};
  
use amethyst_lyon::{
    utils::{Mesh, VertexType}
 };

extern crate lyon;
use lyon::math::{point, Point, Vector, vector, Scale};
use lyon::path::{Path, Builder};
use lyon::tessellation::*;
 

use crate::{
    ui::mainui::{MainUI},  
    commands::{Command, Draw, NUMBER_LAYERS},
    bindings::{ActionBinding},
    commands::svg_path::*,
    vector_meshes::*,
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
        WriteStorage<'a, Mesh>,
        Read<'a, Meshes>,
    );

    fn run(&mut self, (draw, move_to, line_to, mut mesh_storage, meshes): Self::SystemData) { 

       
        let fill_options = FillOptions::default();
        let mut geometry: VertexBuffers<VertexType, u16> = VertexBuffers::new();
        let mut tessellator_stroke = StrokeTessellator::new();
        let mut tessellator_fill = FillTessellator::new();

        // tesselate layers, in reverese order
        for layer in (0..NUMBER_LAYERS).rev() {
            let stroke_options = StrokeOptions::tolerance(0.02)
                .with_line_width(draw.layers[layer].thickness)
                .with_line_join(LineJoin::Round)
                .with_line_cap(LineCap::Round);

            let mut builder = Path::builder();

            for e in &draw.layers[layer].entities {
                if let Some(entity) = e.entity {
                    if let Some(m_to) = move_to.get(entity) {
                        m_to.tessellate(&mut builder);
                        //println!("{}", m_to.gen_output());
                    }
                    else if let Some(l_to) = line_to.get(entity) {
                        l_to.tessellate(&mut builder);
                        //println!("{}", l_to.gen_output());
                    }
                }
            }
    
            tessellate_stroke(
                &mut geometry, 
                &mut tessellator_stroke, &stroke_options, 
                builder, 
                draw.layers[layer].colour);
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