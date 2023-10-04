use bevy::prelude::*;

pub mod alignment;
pub mod button;
pub mod constrained;
pub mod container;
pub mod direction;
pub mod divider;
pub mod scaffold;
pub mod sized_box;
pub mod visibility;

#[derive(Bundle, Clone, Debug)]
pub struct UiWidgetBundle {
    pub node: Node,
    pub style: Style,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for UiWidgetBundle {
    fn default() -> Self {
        UiWidgetBundle {
            node: Default::default(),
            style: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
