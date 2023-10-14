use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::RaycastPickTarget};
use crate::events::gesture::*;
use crate::widgets::WidgetBundle;

/// A widget that detects gestures.
#[derive(Component, Clone, Debug, Default)]
pub struct GestureDetectorWidget;

/// Properties for [`GestureDetectorBundle`] which is a bundle for a widget that detects gestures.
pub struct GestureDetectorProps {
    /// Fires when a tap with a pointer button has occurred.
    pub on_tap: OnTap,
    /// Fires when a pointer that might cause a tap with a button has contacted the
    /// screen at a particular location.
    pub on_tap_down: OnTapDown,
    /// Fires when a pointer that will trigger a tap with a button has stopped
    /// contacting the screen at a particular location.
    pub on_tap_up: OnTapUp,
    /// Fires when a pointer is moving over the widget.
    pub on_move: OnMove,
    /// Fires when a pointer crosses into the bounds of the target entity.
    pub on_contact: OnContact,
    /// Fires when a pointer crosses out of the bounds of the target entity.
    pub on_leave: OnLeave,
}

impl Default for GestureDetectorProps {
    #[inline]
    fn default() -> Self {
        Self {
            on_tap: OnTap::run(|| {}),
            on_tap_down: OnTapDown::run(|| {}),
            on_tap_up: OnTapUp::run(|| {}),
            on_move: OnMove::run(|| {}),
            on_contact: OnContact::run(|| {}),
            on_leave: OnLeave::run(|| {}),
        }
    }
}

/// A bundle for [`GestureDetectorWidget`] which is a widget that detects gestures.
#[derive(Bundle)]
pub struct GestureDetectorBundle {
    /// The widget below this widget in the tree.
    child: WidgetBundle,
    /// Makes an entity pickable.
    pickable: PickableBundle,
    /// Marks an entity that should be pickable with ray casts.
    raycast: RaycastPickTarget,
    /// Fires when a tap with a pointer button has occurred.
    on_tap: OnTap,
    /// Fires when a pointer that might cause a tap with a button has contacted the
    /// screen at a particular location.
    on_tap_down: OnTapDown,
    /// Fires when a pointer that will trigger a tap with a button has stopped
    /// contacting the screen at a particular location.
    on_tap_up: OnTapUp,
    /// Fires when a pointer is moving over the widget.
    on_move: OnMove,
    /// Fires when a pointer crosses into the bounds of the target entity.
    on_contact: OnContact,
    /// Fires when a pointer crosses out of the bounds of the target entity.
    on_leave: OnLeave,
}

impl Default for GestureDetectorBundle {
    /// Create a [`GestureDetectorBundle`] using [`GestureDetectorProps`] defaults.
    ///
    /// This is called in `widget_tree!` when no properties or only `child`, `children` or `tags`
    /// are defined.
    #[inline]
    fn default() -> Self {
        GestureDetectorBundle::from(GestureDetectorProps::default())
    }
}

impl GestureDetectorBundle {
    /// Create a [`GestureDetectorBundle`] using [`GestureDetectorProps`].
    #[inline]
    pub fn from(props: GestureDetectorProps) -> Self {
        Self {
            child: WidgetBundle::default(),
            pickable: PickableBundle::default(),
            raycast: RaycastPickTarget::default(),
            on_tap: props.on_tap,
            on_tap_down: props.on_tap_down,
            on_tap_up: props.on_tap_up,
            on_move: props.on_move,
            on_contact: props.on_contact,
            on_leave: props.on_leave,
        }
    }
}