//! Description: 
//! 
//! Basic bundle to get some things initialized
//! 
//! Copyright © 2020 Benedict Gaster. All rights reserved.

use amethyst::{ 
    core::{
        bundle::SystemBundle,
        ecs::prelude::{DispatcherBuilder, World},
        shrev::{EventChannel},
    },
    input::BindingTypes,
};

use amethyst_error::Error;
use std::marker::PhantomData;
use derive_new::new;

use crate::bindings::{ActionBinding};
use crate::commands::{Command, Draw};

/// Rebound bundle
///
/// Will register all necessary components and systems needed for app, along with any resources.
/// The generic type T represent the T generic parameter of the InputHandler<T>.
///
/// Will fail with error 'No resource with the given id' if the InputBundle is not added.
#[derive(new, Debug)]
pub struct ReboundBundle<T: BindingTypes, G=()> {
    #[new(default)]
    _marker: PhantomData<(T, G)>,
}

impl<'a, 'b, T, G> SystemBundle<'a, 'b> for ReboundBundle<T, G>
where
    T: BindingTypes,
    G: Send + Sync + PartialEq + 'static,
{
    fn build(
        self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {

        world.insert(EventChannel::<ActionBinding>::new());
        world.insert(EventChannel::<Command>::new());
        world.insert(Draw::new());

        Ok(())
    }
}
