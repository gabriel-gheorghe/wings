use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

type SizeQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Style,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct SizeQuery<'w, 's, T: Component>(SizeQueryType<'w, 's, T>);

impl <'w, 's, T: Component> SizeQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &SizeQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut SizeQueryType<'w, 's, T> { &mut self.0 }

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
    pub fn set_size<F>(&mut self, f: F)
    where
        F: FnOnce((Val, Val)) -> (Val, Val) + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_style)| {
            let (width, height) = &f((c_style.width, c_style.height));
            c_style.width = *width;
            c_style.height = *height;
        });
    }

    #[inline]
    pub fn set_width<F>(&mut self, f: F)
    where
        F: FnOnce(Val) -> Val + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_style)| {
            let width = &f(c_style.width);
            c_style.width = *width;
        });
    }

    #[inline]
    pub fn set_height<F>(&mut self, f: F)
    where
        F: FnOnce(Val) -> Val + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_style)| {
            let height = &f(c_style.height);
            c_style.height = *height;
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
