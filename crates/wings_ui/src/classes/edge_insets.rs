use bevy::prelude::*;
use crate::classes::platform_classes::UiViewPadding;

#[derive(Copy, Clone, Debug, Default)]
pub struct UiEdgeInsets {
    pub left: Val,
    pub top: Val,
    pub right: Val,
    pub bottom: Val,
}

impl UiEdgeInsets {
    #[inline]
    pub fn zero() -> Self {
        Self::default()
    }

    #[inline]
    pub fn only_left(value: Val) -> Self {
        Self {
            left: value,
            ..default()
        }
    }

    #[inline]
    pub fn only_top(value: Val) -> Self {
        Self {
            top: value,
            ..default()
        }
    }

    #[inline]
    pub fn only_right(value: Val) -> Self {
        Self {
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn only_bottom(value: Val) -> Self {
        Self {
            bottom: value,
            ..default()
        }
    }

    #[inline]
    pub fn all(value: Val) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    #[inline]
    pub fn from_ltrb(left: Val, top: Val, right: Val, bottom: Val) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn symmetric_vh(vertical: Val, horizontal: Val) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }

    #[inline]
    pub fn symmetric_vertical(value: Val) -> Self {
        Self {
            top: value,
            bottom: value,
            ..default()
        }
    }

    #[inline]
    pub fn symmetric_horizontal(value: Val) -> Self {
        Self {
            left: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tl(top: Val, left: Val) -> Self {
        Self {
            top,
            left,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tr(top: Val, right: Val) -> Self {
        Self {
            top,
            right,
            ..default()
        }
    }

    #[inline]
    pub fn corner_bl(bottom: Val, left: Val) -> Self {
        Self {
            bottom,
            left,
            ..default()
        }
    }

    #[inline]
    pub fn corner_br(bottom: Val, right: Val) -> Self {
        Self {
            bottom,
            right,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tl_all(value: Val) -> Self {
        Self {
            top: value,
            left: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tr_all(value: Val) -> Self {
        Self {
            top: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_bl_all(value: Val) -> Self {
        Self {
            bottom: value,
            left: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_br_all(value: Val) -> Self {
        Self {
            bottom: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn from_view_padding(padding: UiViewPadding, pixel_ratio: f32) -> Self {
        Self {
            left: Val::Px(padding.left / pixel_ratio),
            top: Val::Px(padding.top / pixel_ratio),
            right: Val::Px(padding.right / pixel_ratio),
            bottom: Val::Px(padding.bottom / pixel_ratio),
        }
    }
}
