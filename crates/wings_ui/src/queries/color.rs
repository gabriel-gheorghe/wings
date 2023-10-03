use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use wings_utils::color::get_random_color;

#[derive(SystemParam)]
pub struct UiColorQuery<'w, 's, T: Component> (
    Query<'w, 's,
        (
            Entity,
            &'static mut BackgroundColor,
            With<T>,
        ),
    >,
);

impl <'w, 's, T: Component> UiColorQuery<'w, 's, T> {
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

    pub fn set_random_equally(&mut self) {
        self.set(get_random_color());
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
}
