use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

type UiSizeQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
        With<T>,
    ),
>;

#[derive(SystemParam)]
pub struct UiSizeQuery<'w, 's, T: Component>(UiSizeQueryType<'w, 's, T>);

impl <'w, 's, T: Component> UiSizeQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &UiSizeQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut UiSizeQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn set(&mut self, width: Val, height: Val) {
        self.0.for_each_mut(|(_, mut c_style, _)| {
            c_style.width = width;
            c_style.height = height;
        });
    }

    #[inline]
    pub fn set_width(&mut self, width: Val) {
        self.0.for_each_mut(|(_, mut c_style, _)| {
            c_style.width = width;
        });
    }

    #[inline]
    pub fn set_height(&mut self, height: Val) {
        self.0.for_each_mut(|(_, mut c_style, _)| {
            c_style.height = height;
        });
    }

    #[inline]
    pub fn set_single(&mut self, target: Entity, width: Val, height: Val) {
        self.0.for_each_mut(|(entity, mut c_style, _)| {
            if entity == target {
                c_style.width = width;
                c_style.height = height;
            }
        });
    }

    #[inline]
    pub fn set_width_single(&mut self, target: Entity, width: Val) {
        self.0.for_each_mut(|(entity, mut c_style, _)| {
            if entity == target {
                c_style.width = width;
            }
        });
    }

    #[inline]
    pub fn set_height_single(&mut self, target: Entity, height: Val) {
        self.0.for_each_mut(|(entity, mut c_style, _)| {
            if entity == target {
                c_style.height = height;
            }
        });
    }
}
