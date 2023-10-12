use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::widgets::WidgetBundle;

pub mod events {
    use bevy_mod_picking::prelude::*;

    pub type Tap = Pointer<Click>;
    pub type TapListener<'w> = Listener<'w, Tap>;
    pub type OnTap = On<Tap>;
}

#[derive(Component, Clone, Debug, Default)]
pub struct GestureDetectorWidget;

pub struct GestureDetectorProps {
    pub on_tap: On<Pointer<Click>>,
}

impl Default for GestureDetectorProps {
    #[inline]
    fn default() -> Self {
        Self {
            on_tap: On::<Pointer<Click>>::run(|| {}),
        }
    }
}

#[derive(Bundle)]
pub struct GestureDetectorBundle {
    child: WidgetBundle,
    pickable: PickableBundle,
    raycast: RaycastPickTarget,
    on_tap: On<Pointer<Click>>,
}

impl Default for GestureDetectorBundle {
    fn default() -> Self {
        GestureDetectorBundle::from(GestureDetectorProps::default())
    }
}

impl GestureDetectorBundle {
    pub fn from(props: GestureDetectorProps) -> Self {
        Self {
            child: WidgetBundle::default(),
            pickable: PickableBundle::default(),
            raycast: RaycastPickTarget::default(),
            on_tap: props.on_tap,
        }
    }
}