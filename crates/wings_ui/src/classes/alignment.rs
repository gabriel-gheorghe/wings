#[derive(Copy, Clone, Debug)]
pub struct Alignment {
    pub x: f32,
    pub y: f32,
}

impl Default for Alignment {
    #[inline]
    fn default() -> Self {
        Alignment::TOP_LEFT
    }
}

impl Alignment {
    pub const TOP_LEFT: Alignment = Alignment { x: -1. , y: -1. };
    pub const TOP_CENTER: Alignment = Alignment { x: 0. , y: -1. };
    pub const TOP_RIGHT: Alignment = Alignment { x: 1. , y: -1. };

    pub const CENTER_LEFT: Alignment = Alignment { x: -1. , y: 0. };
    pub const CENTER: Alignment = Alignment { x: 0. , y: 0. };
    pub const CENTER_RIGHT: Alignment = Alignment { x: 1. , y: 0. };

    pub const BOTTOM_LEFT: Alignment = Alignment { x: -1. , y: 1. };
    pub const BOTTOM_CENTER: Alignment = Alignment { x: 0. , y: 1. };
    pub const BOTTOM_RIGHT: Alignment = Alignment { x: 1. , y: 1. };
}
