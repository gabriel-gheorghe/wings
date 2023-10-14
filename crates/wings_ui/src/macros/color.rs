#[allow(unused_macros)]

/// Macro used to define a color inside a `widget_tree`.
///
/// Use cases when you need to retrieve Some(Color) or Color:
/// 1. As an identifier:
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// let some_color = color![TEAL];
/// assert_eq!(some_color, Some(Color::TEAL));
///
/// let color = color![^TEAL];
/// assert_eq!(color, Color::TEAL);
/// ```
/// 2. As an expression:
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// fn main() {
///     let some_color = color![Color::default()];
///     assert_eq!(some_color, Some(Color::default()));
///
///     let color = color![^Color::default()];
///     assert_eq!(color, Color::default());
/// }
/// ```
/// 3. As an array of expressions (rgb):
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// fn main() {
///     let some_color = color![r: 0.75, g: 0.5, b: 0.25];
///     assert_eq!(some_color, Some(Color::rgb(0.75, 0.5, 0.25)));
///
///     let color = color![^r: 0.75, g: 0.5, b: 0.25];
///     assert_eq!(color, Color::rgb(0.75, 0.5, 0.25));
///
///     let some_color_u8 = color![r8: 117, g8: 63, b8: 223];
///     assert_eq!(some_color_u8, Some(Color::rgb_u8(117, 63, 223)));
///
///     let color_u8 = color![^r8: 117, g8: 63, b8: 223];
///     assert_eq!(color_u8, Color::rgb_u8(117, 63, 223));
/// }
/// ```
/// 4. As an array of expressions (rgba):
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// fn main() {
///     let some_color = color![r: 0.75, g: 0.5, b: 0.25, a: 1.];
///     assert_eq!(some_color, Some(Color::rgba(0.75, 0.5, 0.25, 1.)));
///
///     let color = color![^r: 0.75, g: 0.5, b: 0.25, a: 1.];
///     assert_eq!(color, Color::rgba(0.75, 0.5, 0.25, 1.));
///
///     let some_color_u8 = color![r8: 117, g8: 63, b8: 223, a8: 255];
///     assert_eq!(some_color_u8, Some(Color::rgba_u8(117, 63, 223, 255)));
///
///     let color_u8 = color![^r8: 117, g8: 63, b8: 223, a8: 255];
///     assert_eq!(color_u8, Color::rgba_u8(117, 63, 223, 255));
/// }
/// ```
/// 5. As a literal (hex):
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// fn main() {
///     let some_color = color!["#00FF00FF"];
///     assert_eq!(some_color, Some(Color::hex("#00FF00FF").unwrap_or(Color::NONE)));
///
///     let color = color![^"#00FF00FF"];
///     assert_eq!(color, Color::hex("#00FF00FF").unwrap_or(Color::NONE));
/// }
/// ```
/// 6. As an expression (hex):
/// ```
/// use bevy::prelude::*;
/// use wings_ui::prelude::*;
///
/// fn main() {
///     let mut hex_color = "#00FF00FF";
///
///     let some_color = color![hex hex_color];
///     assert_eq!(some_color, Some(Color::hex("#00FF00FF").unwrap_or(Color::NONE)));
///
///     let color = color![^hex hex_color];
///     assert_eq!(color, Color::hex("#00FF00FF").unwrap_or(Color::NONE));
/// }
/// ```
#[macro_export]
macro_rules! color {
    ($x:literal) => {{
        Some(Color::hex($x).unwrap_or(Color::NONE))
    }};
    (hex $x:expr) => {{
        Some(Color::hex($x).unwrap_or(Color::NONE))
    }};
    (r: $r:expr, g: $g:expr, b: $b:expr) => {
        Some(Color::rgb($r, $g, $b))
    };
    (r8: $r:expr, g8: $g:expr, b8: $b:expr) => {
        Some(Color::rgb_u8($r, $g, $b))
    };
    (r: $r:expr, g: $g:expr, b: $b:expr, a: $a:expr) => {
        Some(Color::rgba($r, $g, $b, $a))
    };
    (r8: $r:expr, g8: $g:expr, b8: $b:expr, a8: $a:expr) => {
        Some(Color::rgba_u8($r, $g, $b, $a))
    };
    ($x:ident) => {
        Some(Color::$x)
    };
    ($x:expr) => {
        Some($x)
    };

    (^$x:literal) => {{
        Color::hex($x).unwrap_or(Color::NONE)
    }};
    (^hex $x:expr) => {{
        Color::hex($x).unwrap_or(Color::NONE)
    }};
    (^r: $r:expr, g: $g:expr, b: $b:expr) => {
        Color::rgb($r, $g, $b)
    };
    (^r8: $r:expr, g8: $g:expr, b8: $b:expr) => {
        Color::rgb_u8($r, $g, $b)
    };
    (^r: $r:expr, g: $g:expr, b: $b:expr, a: $a:expr) => {
        Color::rgba($r, $g, $b, $a)
    };
    (^r8: $r:expr, g8: $g:expr, b8: $b:expr, a8: $a:expr) => {
        Color::rgba_u8($r, $g, $b, $a)
    };
    (^$x:ident) => {
        Color::$x
    };
    (^$x:expr) => {
        $x
    };
}