use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use wings_utils::color::{get_random_color, get_random_color_with_alpha};

type UiColorQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut BackgroundColor,
        With<T>,
    ),
>;

#[derive(SystemParam)]
pub struct UiColorQuery<'w, 's, T: Component>(UiColorQueryType<'w, 's, T>);

impl <'w, 's, T: Component> UiColorQuery<'w, 's, T> {
    pub fn get(&self) -> &UiColorQueryType<'w, 's, T> { &self.0 }
    pub fn get_mut(&mut self) -> &mut UiColorQueryType<'w, 's, T> { &mut self.0 }


    pub fn set(&mut self, color: Color) {
        self.0.for_each_mut(|(_, mut c_color, _)| {
            c_color.0 = color;
        });
    }

    pub fn set_random(&mut self) {
        self.0.for_each_mut(|(_, mut c_color, _)| {
            c_color.0 = get_random_color();
        });
    }

    pub fn set_random_with_alpha(&mut self) {
        self.0.for_each_mut(|(_, mut c_color, _)| {
            c_color.0 = get_random_color_with_alpha();
        });
    }

    pub fn set_random_equally(&mut self) {
        self.set(get_random_color());
    }

    pub fn set_random_with_alpha_equally(&mut self) {
        self.set(get_random_color_with_alpha());
    }

    pub fn set_single(&mut self, target: Entity, color: Color) {
        self.0.for_each_mut(|(entity, mut c_color, _)| {
            if entity == target {
                c_color.0 = color;
            }
        });
    }

    pub fn set_random_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_color, _)| {
            if entity == target {
                c_color.0 = get_random_color();
            }
        });
    }

    pub fn set_random_with_alpha_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_color, _)| {
            if entity == target {
                c_color.0 = get_random_color_with_alpha();
            }
        });
    }
}
