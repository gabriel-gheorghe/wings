use bevy::prelude::*;

/// Returns a new randomly chosen [`Color`](Color) with red, green and blue channels.
#[inline]
pub fn get_random_color() -> Color {
    Color::rgb(rand::random(), rand::random(), rand::random())
}

/// Returns a new randomly chosen [`Color`](Color) with red, green, blue and alpha channels.
///
/// The `a` parameter represents the alpha channel value.
/// In order to be valid it must fall within the range of 0 to 1.
/// If `None` is passed then the alpha channel will be randomly chosen.
///
/// ```
/// use wings_utils::prelude::*;
///
/// let color = get_random_color_with_alpha(Some(0.35));
/// assert_eq!(color.a(), 0.35);
/// ```
#[inline]
pub fn get_random_color_with_alpha(a: Option<f32>) -> Color {
    if let Some(a) = a {
        assert!(a >= 0. && a <= 1., "Alpha must be between 0 and 1.");
    }

    match a {
        Some(a) => Color::rgba(rand::random(), rand::random(), rand::random(), a),
        None => Color::rgba(rand::random(), rand::random(), rand::random(), rand::random()),
    }
}

/// Returns a transparent [`Color`](Color). All color channels ar set to `0.`;
#[inline]
pub const fn get_transparent_color() -> Color {
    Color::rgba(0., 0., 0., 0.)
}
