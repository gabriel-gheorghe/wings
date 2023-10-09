use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ConstrainedHeightWidget;

#[derive(Bundle, Clone, Debug)]
pub struct ConstrainedHeightBundle {
    child: WidgetBundle,
    debug_color: BackgroundColor,
    widget: ConstrainedHeightWidget,
}

impl Default for ConstrainedHeightBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            debug_color: BackgroundColor::from(get_transparent_color()),
            widget: ConstrainedHeightWidget::default(),
        }
    }
}

impl ConstrainedHeightBundle {
    #[inline]
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.debug_color = BackgroundColor::from(color);
        res
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct ConstrainedWidthWidget;

#[derive(Bundle, Clone, Debug)]
pub struct ConstrainedWidthBundle {
    child: WidgetBundle,
    debug_color: BackgroundColor,
    widget: ConstrainedWidthWidget,
}

impl Default for ConstrainedWidthBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            debug_color: BackgroundColor::from(get_transparent_color()),
            widget: ConstrainedWidthWidget::default(),
        }
    }
}

impl ConstrainedWidthBundle {
    #[inline]
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.debug_color = BackgroundColor::from(color);
        res
    }
}
