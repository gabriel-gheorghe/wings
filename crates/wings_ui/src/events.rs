use bevy::prelude::*;

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintHeight(pub(crate) Entity, pub(crate) f32);

#[derive(Event, Debug)]
pub(crate) struct ApplyConstraintWidth(pub(crate) Entity, pub(crate) f32);

pub mod gesture {
    use bevy_mod_picking::prelude as bmp;

    pub type Tap = bmp::Pointer<bmp::Click>;
    pub type TapListener<'w> = bmp::Listener<'w, Tap>;
    pub type OnTap = bmp::On<Tap>;

    pub type TapDown = bmp::Pointer<bmp::Down>;
    pub type TapDownListener<'w> = bmp::Listener<'w, TapDown>;
    pub type OnTapDown = bmp::On<TapDown>;

    pub type TapUp = bmp::Pointer<bmp::Up>;
    pub type TapUpListener<'w> = bmp::Listener<'w, TapUp>;
    pub type OnTapUp = bmp::On<TapUp>;

    pub type Move = bmp::Pointer<bmp::Move>;
    pub type MoveListener<'w> = bmp::Listener<'w, Move>;
    pub type OnMove = bmp::On<Move>;

    pub type Contact = bmp::Pointer<bmp::Over>;
    pub type ContactListener<'w> = bmp::Listener<'w, Contact>;
    pub type OnContact = bmp::On<Contact>;

    pub type Leave = bmp::Pointer<bmp::Out>;
    pub type LeaveListener<'w> = bmp::Listener<'w, Leave>;
    pub type OnLeave = bmp::On<Leave>;
}