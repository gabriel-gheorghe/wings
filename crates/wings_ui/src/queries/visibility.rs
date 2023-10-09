use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::widgets::visibility::{get_computed_display, get_computed_visibility, UiLayoutVisibility, UiVisibility};

type VisibilityQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
        &'static mut UiVisibility,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct VisibilityQuery<'w, 's, T: Component>(VisibilityQueryType<'w, 's, T>);

impl <'w, 's, T: Component> VisibilityQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &VisibilityQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut VisibilityQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn get_visible(&mut self, target: Entity) -> bool {
        let mut res = false;
        self.0.for_each_mut(|(entity, _, ui_visibility)| {
            if entity == target {
                res = ui_visibility.0;
            }
        });
        res
    }

    #[inline]
    pub fn set_visible(&mut self, visible: bool) {
        self.0.for_each_mut(|(_, mut c_style, mut ui_visibility)| {
            c_style.display = if visible { Display::Flex } else { Display::None };
            *ui_visibility = UiVisibility(visible);
        });
    }

    #[inline]
    pub fn set_visible_single(&mut self, target: Entity, visible: bool) {
        self.0.for_each_mut(|(entity, mut c_style, mut ui_visibility)| {
            if entity == target {
                c_style.display = if visible { Display::Flex } else { Display::None };
                *ui_visibility = UiVisibility(visible);
            }
        });
    }
}

type LayoutVisibilityQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
        &'static mut Visibility,
        &'static mut UiLayoutVisibility,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct LayoutVisibilityQuery<'w, 's, T: Component>(LayoutVisibilityQueryType<'w, 's, T>);

impl <'w, 's, T: Component> LayoutVisibilityQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &LayoutVisibilityQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut LayoutVisibilityQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn get_visibility(&mut self, target: Entity) -> UiLayoutVisibility {
        let mut res = UiLayoutVisibility::default();
        self.0.for_each_mut(|(entity, _, _, ui_visibility)| {
            if entity == target {
                res = ui_visibility.to_owned();
            }
        });
        res
    }

    #[inline]
    pub fn set_visibility(&mut self, visibility: UiLayoutVisibility) {
        self.0.for_each_mut(|(_, mut c_style, mut c_visibility, mut ui_visibility)| {
            c_style.display = get_computed_display(&visibility);
            *c_visibility = get_computed_visibility(&visibility);
            *ui_visibility = visibility;
        });
    }

    #[inline]
    pub fn set_visibility_single(&mut self, target: Entity, visibility: UiLayoutVisibility) {
        self.0.for_each_mut(|(entity, mut c_style, mut c_visibility, mut ui_visibility)| {
            if entity == target {
                c_style.display = get_computed_display(&visibility);
                *c_visibility = get_computed_visibility(&visibility);
                *ui_visibility = visibility;
            }
        });
    }
}
