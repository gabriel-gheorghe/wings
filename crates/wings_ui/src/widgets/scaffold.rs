use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ScaffoldWidget;

#[derive(Copy, Clone, Debug)]
pub struct ScaffoldProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
}

impl Default for ScaffoldProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Auto,
            color: get_transparent_color(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct ScaffoldBundle {
    child: WidgetBundle,
    background_color: BackgroundColor,
    widget: ScaffoldWidget,
}

impl Default for ScaffoldBundle {
    #[inline]
    fn default() -> Self {
        Self::from(ScaffoldProps::default())
    }
}

impl ScaffoldBundle {
    #[inline]
    pub fn from(props: ScaffoldProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(props.color),
            widget: ScaffoldWidget::default(),
        }
    }

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(ScaffoldProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(val: Val) -> Self {
        Self::from(ScaffoldProps {
            width: val,
            height: val,
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(ScaffoldProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(ScaffoldProps {
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self::from(ScaffoldProps {
            color,
            ..default()
        })
    }
}
