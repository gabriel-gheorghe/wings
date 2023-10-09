use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

type UiSizeQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct UiSizeQuery<'w, 's, T: Component>(UiSizeQueryType<'w, 's, T>);

impl <'w, 's, T: Component> UiSizeQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &UiSizeQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut UiSizeQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn get_size(&mut self, target: Entity) -> (Val, Val) {
        let mut res = (Val::default(), Val::default());
        self.0.for_each_mut(|(entity, c_style)| {
            if entity == target {
                res = (c_style.width, c_style.height);
            }
        });
        res
    }

    #[inline]
    pub fn get_width(&mut self, target: Entity) -> Val {
        let mut res = Val::default();
        self.0.for_each_mut(|(entity, c_style)| {
            if entity == target {
                res = c_style.width;
            }
        });
        res
    }

    #[inline]
    pub fn get_height(&mut self, target: Entity) -> Val {
        let mut res = Val::default();
        self.0.for_each_mut(|(entity, c_style)| {
            if entity == target {
                res = c_style.height;
            }
        });
        res
    }

    #[inline]
    pub fn set_size(&mut self, width: Val, height: Val) {
        self.0.for_each_mut(|(_, mut c_style)| {
            c_style.width = width;
            c_style.height = height;
        });
    }

    #[inline]
    pub fn set_width(&mut self, width: Val) {
        self.0.for_each_mut(|(_, mut c_style)| {
            c_style.width = width;
        });
    }

    #[inline]
    pub fn set_height(&mut self, height: Val) {
        self.0.for_each_mut(|(_, mut c_style)| {
            c_style.height = height;
        });
    }

    #[inline]
    pub fn set_size_single(&mut self, target: Entity, width: Val, height: Val) {
        self.0.for_each_mut(|(entity, mut c_style)| {
            if entity == target {
                c_style.width = width;
                c_style.height = height;
            }
        });
    }

    #[inline]
    pub fn set_width_single(&mut self, target: Entity, width: Val) {
        self.0.for_each_mut(|(entity, mut c_style)| {
            if entity == target {
                c_style.width = width;
            }
        });
    }

    #[inline]
    pub fn set_height_single(&mut self, target: Entity, height: Val) {
        self.0.for_each_mut(|(entity, mut c_style)| {
            if entity == target {
                c_style.height = height;
            }
        });
    }
}
