#[allow(unused_macros)]

/// Macro used to define a color inside a `widget_tree`.
///
/// Use cases when you need to get Some(Color):
/// ```ignore
/// color: color![TEAL] // as an identifier
/// color: color![Color::default()] // as an expression
/// color: color![r: 1., g: 0.5, b: 0.5] // as an array of expressions [rgb]
/// color: color![r: 1., g: 0.5, b: 0.5., a: 1.] // as an array of expressions [rgba]
/// color: color!["#00FF00FF"] // as a literal (hex)
///
/// let mut hex_color = "#00FF00FF";
/// color: color![hex hex_color] // as an expression (hex)
/// ```
///
/// Use cases when you need to get simply Color (just add ^):
/// ```ignore
/// color: color![^TEAL] // as an identifier
/// color: color![^Color::default()] // as an expression
/// color: color![^r: 1., g: 0.5, b: 0.5] // as an array of expressions [rgb]
/// color: color![^r: 1., g: 0.5, b: 0.5., a: 1.] // as an array of expressions [rgba]
/// color: color![^"#00FF00FF"] // as a literal (hex)
///
/// let mut hex_color = "#00FF00FF";
/// color: color![^hex hex_color] // as an expression (hex)
/// ```
#[macro_export]
macro_rules! color {
    ($x:literal) => {{
        use wings::prelude::get_transparent_color;
        Some(Color::hex($x).unwrap_or(get_transparent_color()))
    }};
    (hex $x:expr) => {{
        use wings::prelude::get_transparent_color;
        Some(Color::hex($x).unwrap_or(get_transparent_color()))
    }};
    (r: $r:expr, g: $g:expr, b: $b:expr) => {
        Some(Color::rgb($r, $g, $b))
    };
    (r: $r:expr, g: $g:expr, b: $b:expr, a: $a:expr) => {
        Some(Color::rgba($r, $g, $b, $a))
    };
    ($x:ident) => {
        Some(Color::$x)
    };
    ($x:expr) => {
        Some($x)
    };

    (^$x:literal) => {{
        use wings::prelude::get_transparent_color;
        Color::hex($x).unwrap_or(get_transparent_color())
    }};
    (^hex $x:expr) => {{
        use wings::prelude::get_transparent_color;
        Color::hex($x).unwrap_or(get_transparent_color())
    }};
    (^r: $r:expr, g: $g:expr, b: $b:expr) => {
        Color::rgb($r, $g, $b)
    };
    (^r: $r:expr, g: $g:expr, b: $b:expr, a: $a:expr) => {
        Color::rgba($r, $g, $b, $a)
    };
    (^$x:ident) => {
        Color::$x
    };
    (^$x:expr) => {
        $x
    };
}