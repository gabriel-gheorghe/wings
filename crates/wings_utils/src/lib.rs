pub mod color;

pub mod prelude {
    pub use crate::color::{
        get_random_color, get_random_color_with_alpha, get_transparent_color,
    };
}
