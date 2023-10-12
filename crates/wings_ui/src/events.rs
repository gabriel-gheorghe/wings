use bevy::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintHeight(pub(crate) Entity, pub(crate) f32);

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintWidth(pub(crate) Entity, pub(crate) f32);
