use bevy::prelude::*;
use wings_utils::color::get_transparent_color;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum BorderStyle {
    None,
    Solid,
}

#[derive(Copy, Clone, Debug)]
pub struct BorderSide {
    pub color: Color,
    pub width: Val,
    pub style: BorderStyle,
}

impl Default for BorderSide {
    #[inline]
    fn default() -> Self {
        Self {
            color: get_transparent_color(),
            width: Val::Px(1.),
            style: BorderStyle::Solid,
        }
    }
}

impl BorderSide {
    pub const NONE: BorderSide = BorderSide {
        color: get_transparent_color(),
        width: Val::Px(0.),
        style: BorderStyle::None,
    };

    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            ..default()
        }
    }

    #[inline]
    pub fn from_width_color(width: Val, color: Color) -> Self {
        Self {
            width,
            color,
            ..default()
        }
    }

    #[inline]
    pub fn merge(a: BorderSide, b: BorderSide) -> Self {
        assert!(Self::can_merge(a, b));

        let a_is_none = a.style == BorderStyle::None && a.width == Val::Px(0.);
        let b_is_none = b.style == BorderStyle::None && b.width == Val::Px(0.);

        if a_is_none && b_is_none {
            return BorderSide::NONE;
        }
        if a_is_none {
            return b;
        }
        if b_is_none {
            return a;
        }

        assert_eq!(a.color, b.color);
        assert_eq!(a.style, b.style);

        Self {
            color: a.color,
            width: match a.width {
                Val::Px(a_width) => match b.width {
                    Val::Px(b_width) => Val::Px(a_width + b_width),
                    _ => b.width
                },
                _ => a.width
            },
            style: a.style,
        }
    }

    #[inline]
    pub fn can_merge(a: BorderSide, b: BorderSide) -> bool {
        return if (a.style == BorderStyle::None && a.width == Val::Px(0.)) ||
            (b.style == BorderStyle::None && b.width == Val::Px(0.)) {
            true
        } else {
            a.style == b.style && a.color == b.color
        }
    }

    #[inline]
    pub fn scale(&self, s: f32) -> Self {
        let width = match self.width {
            Val::Px(self_width) => self_width,
            _ => { assert!(false, "Expected Val::Px"); 0. },
        };

        Self {
            color: self.color,
            width: Val::Px(f32::max(0., width * s)),
            style: if s <= 0. { BorderStyle::None } else { self.style },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub left: BorderSide,
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
}

impl Default for Border {
    #[inline]
    fn default() -> Self {
        Self {
            left: BorderSide::NONE,
            top: BorderSide::NONE,
            right: BorderSide::NONE,
            bottom: BorderSide::NONE,
        }
    }
}

impl Border {
    #[inline]
    pub fn only_left(side: BorderSide) -> Self {
        Self {
            left: side,
            ..default()
        }
    }

    #[inline]
    pub fn only_top(side: BorderSide) -> Self {
        Self {
            top: side,
            ..default()
        }
    }

    #[inline]
    pub fn only_right(side: BorderSide) -> Self {
        Self {
            right: side,
            ..default()
        }
    }

    #[inline]
    pub fn only_bottom(side: BorderSide) -> Self {
        Self {
            bottom: side,
            ..default()
        }
    }

    #[inline]
    pub fn all(side: BorderSide) -> Self {
        Self {
            top: side,
            right: side,
            bottom: side,
            left: side,
        }
    }

    #[inline]
    pub fn from_ltrb(left: BorderSide, top: BorderSide,
                     right: BorderSide, bottom: BorderSide) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn symmetric_vh(vertical: BorderSide, horizontal: BorderSide) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }

    #[inline]
    pub fn symmetric_vertical(value: BorderSide) -> Self {
        Self {
            top: value,
            bottom: value,
            ..default()
        }
    }

    #[inline]
    pub fn symmetric_horizontal(value: BorderSide) -> Self {
        Self {
            left: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tl(top: BorderSide, left: BorderSide) -> Self {
        Self {
            top,
            left,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tr(top: BorderSide, right: BorderSide) -> Self {
        Self {
            top,
            right,
            ..default()
        }
    }

    #[inline]
    pub fn corner_bl(bottom: BorderSide, left: BorderSide) -> Self {
        Self {
            bottom,
            left,
            ..default()
        }
    }

    #[inline]
    pub fn corner_br(bottom: BorderSide, right: BorderSide) -> Self {
        Self {
            bottom,
            right,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tl_all(value: BorderSide) -> Self {
        Self {
            top: value,
            left: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_tr_all(value: BorderSide) -> Self {
        Self {
            top: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_bl_all(value: BorderSide) -> Self {
        Self {
            bottom: value,
            left: value,
            ..default()
        }
    }

    #[inline]
    pub fn corner_br_all(value: BorderSide) -> Self {
        Self {
            bottom: value,
            right: value,
            ..default()
        }
    }

    #[inline]
    pub fn to_ui_rect(&self) -> UiRect {
        UiRect {
            left: self.left.width,
            top: self.top.width,
            right: self.right.width,
            bottom: self.bottom.width,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxDecoration {
    pub color: Color,
    pub border: Border,
    pub image: Option<Image>,
}

impl Default for BoxDecoration {
    #[inline]
    fn default() -> Self {
        Self {
            color: Color::BISQUE,
            border: Border::default(),
            image: None,
        }
    }
}
