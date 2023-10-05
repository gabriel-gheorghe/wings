#[derive(Copy, Clone, Debug, Default)]
pub struct UiAlignment {
    pub x: f32,
    pub y: f32,
}

impl UiAlignment {
    pub const TOP_LEFT: UiAlignment = UiAlignment { x: -1. , y: -1. };
    pub const TOP_CENTER: UiAlignment = UiAlignment { x: 0. , y: -1. };
    pub const TOP_RIGHT: UiAlignment = UiAlignment { x: 1. , y: -1. };

    pub const CENTER_LEFT: UiAlignment = UiAlignment { x: -1. , y: 0. };
    pub const CENTER: UiAlignment = UiAlignment { x: 0. , y: 0. };
    pub const CENTER_RIGHT: UiAlignment = UiAlignment { x: 1. , y: 0. };

    pub const BOTTOM_LEFT: UiAlignment = UiAlignment { x: -1. , y: 1. };
    pub const BOTTOM_CENTER: UiAlignment = UiAlignment { x: 0. , y: 1. };
    pub const BOTTOM_RIGHT: UiAlignment = UiAlignment { x: 1. , y: 1. };
}
