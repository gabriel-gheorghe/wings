use bevy::prelude::*;
use bevy_eventlistener::{
    event_listener::{EntityEvent},
};
use bevy_eventlistener_derive::EntityEvent;

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerClick {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerDoubleClick {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerRelease {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerPress {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerMove {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerOver {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerEnter {
    #[target]
    pub target: Entity,
}

#[derive(Event, EntityEvent, Clone, Debug)]
pub struct UiPointerExit {
    #[target]
    pub target: Entity,
}

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintHeight(pub(crate) Entity, pub(crate) f32);

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintWidth(pub(crate) Entity, pub(crate) f32);
