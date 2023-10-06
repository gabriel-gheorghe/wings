use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::widgets::visibility::{get_computed_display, get_computed_visibility, UiLayoutVisibility, UiVisibility};

type UiVisibilityQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
        &'static mut UiVisibility,
        With<T>,
    ),
>;

#[derive(SystemParam)]
pub struct UiVisibilityQuery<'w, 's, T: Component>(UiVisibilityQueryType<'w, 's, T>);

impl <'w, 's, T: Component> UiVisibilityQuery<'w, 's, T> {
    pub fn get(&self) -> &UiVisibilityQueryType<'w, 's, T> { &self.0 }
    pub fn get_mut(&mut self) -> &mut UiVisibilityQueryType<'w, 's, T> { &mut self.0 }

    pub fn set(&mut self, visible: bool) {
        self.0.for_each_mut(|(_, mut c_style, mut ui_visibility, _)| {
            c_style.display = if visible { Display::Flex } else { Display::None };
            *ui_visibility = UiVisibility(visible);
        });
    }

    pub fn set_single(&mut self, target: Entity, visible: bool) {
        self.0.for_each_mut(|(entity, mut c_style, mut ui_visibility, _)| {
            if entity == target {
                c_style.display = if visible { Display::Flex } else { Display::None };
                *ui_visibility = UiVisibility(visible);
            }
        });
    }
}

type UiLayoutVisibilityQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
        &'static mut Visibility,
        &'static mut UiLayoutVisibility,
        With<T>,
    ),
>;

#[derive(SystemParam)]
pub struct UiLayoutVisibilityQuery<'w, 's, T: Component>(UiLayoutVisibilityQueryType<'w, 's, T>);

impl <'w, 's, T: Component> UiLayoutVisibilityQuery<'w, 's, T> {
    pub fn get(&self) -> &UiLayoutVisibilityQueryType<'w, 's, T> { &self.0 }
    pub fn get_mut(&mut self) -> &mut UiLayoutVisibilityQueryType<'w, 's, T> { &mut self.0 }

    pub fn set(&mut self, visibility: UiLayoutVisibility) {
        self.0.for_each_mut(|(_, mut c_style, mut c_visibility, mut ui_visibility, _)| {
            c_style.display = get_computed_display(&visibility);
            *c_visibility = get_computed_visibility(&visibility);
            *ui_visibility = visibility;
        });
    }

    pub fn set_single(&mut self, target: Entity, visibility: UiLayoutVisibility) {
        self.0.for_each_mut(|(entity, mut c_style, mut c_visibility, mut ui_visibility, _)| {
            if entity == target {
                c_style.display = get_computed_display(&visibility);
                *c_visibility = get_computed_visibility(&visibility);
                *ui_visibility = visibility;
            }
        });
    }
}
