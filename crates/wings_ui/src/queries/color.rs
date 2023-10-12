use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use wings_utils::color::{get_random_color, get_random_color_with_alpha};

type ColorQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut BackgroundColor,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct ColorQuery<'w, 's, T: Component>(ColorQueryType<'w, 's, T>);

impl <'w, 's, T: Component> ColorQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &ColorQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut ColorQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn get_color(&mut self, target: Entity) -> Color {
        let mut res = Color::WHITE;
        self.0.for_each_mut(|(entity, c_color)| {
            if entity == target {
                res = c_color.0;
            }
        });
        res
    }

    #[inline]
    pub fn set_color<F>(&mut self, f: F)
    where
        F: FnOnce(Color) -> Color + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_color)| {
            let color = &f(c_color.0);
            c_color.0 = *color;
        });
    }

    #[inline]
    pub fn set_random_color(&mut self) {
        self.0.for_each_mut(|(_, mut c_color)| {
            c_color.0 = get_random_color();
        });
    }

    #[inline]
    pub fn set_random_color_with_alpha(&mut self) {
        self.0.for_each_mut(|(_, mut c_color)| {
            c_color.0 = get_random_color_with_alpha(None);
        });
    }

    #[inline]
    pub fn set_random_color_equally(&mut self) {
        self.set_color(|_| get_random_color());
    }

    #[inline]
    pub fn set_random_color_with_alpha_equally(&mut self) {
        self.set_color(|_| get_random_color_with_alpha(None));
    }

    #[inline]
    pub fn set_color_single(&mut self, target: Entity, color: Color) {
        self.0.for_each_mut(|(entity, mut c_color)| {
            if entity == target {
                c_color.0 = color;
            }
        });
    }

    #[inline]
    pub fn set_random_color_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_color)| {
            if entity == target {
                c_color.0 = get_random_color();
            }
        });
    }

    #[inline]
    pub fn set_random_color_with_alpha_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_color)| {
            if entity == target {
                c_color.0 = get_random_color_with_alpha(None);
            }
        });
    }
}
