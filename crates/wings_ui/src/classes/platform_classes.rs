#[derive(Copy, Clone, Debug, Default)]
pub struct ViewPadding {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl ViewPadding {
    #[inline]
    pub fn zero() -> Self {
        Self::default()
    }
}
