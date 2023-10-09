use bevy::prelude::*;

#[inline]
pub fn get_random_color() -> Color {
    Color::rgb(rand::random(), rand::random(), rand::random())
}

#[inline]
pub fn get_random_color_with_alpha() -> Color {
    Color::rgba(rand::random(), rand::random(), rand::random(), rand::random())
}

#[inline]
pub const fn get_transparent_color() -> Color {
    Color::rgba(0.0, 0.0, 0.0, 0.0)
}
