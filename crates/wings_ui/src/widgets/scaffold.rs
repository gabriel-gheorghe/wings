use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::widgets::{UiScaffold, UiVisibility};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug)]
pub struct UiScaffoldProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub visibility: UiVisibility,
}

impl Default for UiScaffoldProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            color: get_transparent_color(),
            visibility: UiVisibility::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiScaffoldBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
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
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                background_color: BackgroundColor::from(props.color),
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiScaffold::default(),
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiScaffoldProps { visibility, ..default() })
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
