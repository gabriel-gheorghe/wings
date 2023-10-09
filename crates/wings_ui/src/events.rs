use bevy::prelude::*;
use bevy_eventlistener::{
    event_listener::{EntityEvent},
};
use bevy_eventlistener_derive::EntityEvent;

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerClick {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerDoubleClick {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerRelease {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerPress {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerMove {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerOver {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerEnter {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct PointerExit {
    #[target]
    pub target: Entity,
}

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintHeight(pub(crate) Entity, pub(crate) f32);

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintWidth(pub(crate) Entity, pub(crate) f32);
