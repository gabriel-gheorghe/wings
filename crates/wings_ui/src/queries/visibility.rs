use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::widgets::UiVisibility;
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(SystemParam)]
pub struct UiVisibilityQuery<'w, 's, T: Component>(
    Query<'w, 's,
        (
            &'static mut Style,
            &'static mut Visibility,
            &'static mut UiVisibility,
            With<T>,
        )
    >,
);

impl <'w, 's, T: Component> UiVisibilityQuery<'w, 's, T> {
    pub fn set(&mut self, visibility: UiVisibility) {
        self.0.for_each_mut(|(mut c_style, mut c_visibility, mut ui_visibility, _)| {
            c_style.display = get_computed_display(&visibility);
            *c_visibility = get_computed_visibility(&visibility);
            *ui_visibility = visibility;
        });
    }
}
