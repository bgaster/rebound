use amethyst::{
    ecs::prelude::{Entity, WorldExt},
    core::ecs::{Component, DenseVecStorage},
    prelude::*,
};

use amethyst_lyon::{
    utils::{Mesh}
};

#[derive(Default)]
pub struct Meshes {
    pub menu: Option<Entity>,
    pub grid: Option<Entity>,
    pub cmds: Option<Entity>,
}

impl Meshes {
    pub fn new<'a>(world: &'a mut World) -> Self {
        Self {
            menu: Some(world
                .create_entity()
                .with(Mesh::default())
                .build()),
            grid: Some(world
                .create_entity()
                .with(Mesh::default())
                .build()),
            cmds: Some(world
                .create_entity()
                .with(Mesh::default())
                .build()),
        }
    }
}

impl Component for Meshes {
    type Storage = DenseVecStorage<Self>;
}