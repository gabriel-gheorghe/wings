use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use wings_utils::color::get_random_color;

#[derive(SystemParam)]
pub struct UiColorQuery<'w, 's, T: Component> (
    Query<'w, 's, (&'static mut BackgroundColor, With<T>)>,
);

impl <'w, 's, T: Component> UiColorQuery<'w, 's, T> {
    pub fn set(&mut self, color: Color) {
        self.0.for_each_mut(|(mut c_color, _)| {
            c_color.0 = color;
        });
    }

    pub fn set_random(&mut self) {
        self.0.for_each_mut(|(mut c_color, _)| {
            c_color.0 = get_random_color();
        });
    }

    pub fn set_random_equally(&mut self) {
        self.set(get_random_color());
    }
}
