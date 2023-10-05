#[derive(Copy, Clone, Debug, Default)]
pub struct UiViewPadding {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl UiViewPadding {
    pub fn zero() -> Self {
        Self::default()
    }
}
