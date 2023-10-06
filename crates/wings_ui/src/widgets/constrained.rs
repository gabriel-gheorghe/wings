use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiConstrainedHeight;

#[derive(Bundle, Clone, Debug)]
pub struct UiConstrainedHeightBundle {
    pub child: UiWidgetBundle,
    pub debug_color: BackgroundColor,
    internal_tag: UiConstrainedHeight,
}

impl Default for UiConstrainedHeightBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            debug_color: BackgroundColor::from(get_transparent_color()),
            internal_tag: UiConstrainedHeight::default(),
        }
    }
}

impl UiConstrainedHeightBundle {
    #[inline]
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.debug_color = BackgroundColor::from(color);
        res
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiConstrainedWidth;

#[derive(Bundle, Clone, Debug)]
pub struct UiConstrainedWidthBundle {
    pub child: UiWidgetBundle,
    pub debug_color: BackgroundColor,
    internal_tag: UiConstrainedWidth,
}

impl Default for UiConstrainedWidthBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            debug_color: BackgroundColor::from(get_transparent_color()),
            internal_tag: UiConstrainedWidth::default(),
        }
    }
}

impl UiConstrainedWidthBundle {
    #[inline]
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.debug_color = BackgroundColor::from(color);
        res
    }
}
