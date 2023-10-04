use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::prelude::UiWidgetBundle;
use crate::widgets::UiScaffold;

#[derive(Copy, Clone, Debug)]
pub struct UiScaffoldProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
}

impl Default for UiScaffoldProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            color: get_transparent_color(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiScaffoldBundle {
    pub child: UiWidgetBundle,
    pub background_color: BackgroundColor,
    internal_tag: UiScaffold,
}

impl Default for UiScaffoldBundle {
    fn default() -> Self {
        Self::from(UiScaffoldProps::default())
    }
}

impl UiScaffoldBundle {
    pub fn from(props: UiScaffoldProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(props.color),
            internal_tag: UiScaffold::default(),
        }
    }

    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiScaffoldProps {
            width,
            height,
            ..default()
        })
    }

    pub fn from_size_splat(val: Val) -> Self {
        Self::from(UiScaffoldProps {
            width: val,
            height: val,
            ..default()
        })
    }

    pub fn from_width(width: Val) -> Self {
        Self::from(UiScaffoldProps {
            width,
            ..default()
        })
    }

    pub fn from_height(height: Val) -> Self {
        Self::from(UiScaffoldProps {
            height,
            ..default()
        })
    }

    pub fn from_color(color: Color) -> Self {
        Self::from(UiScaffoldProps {
            color,
            ..default()
        })
    }
}
