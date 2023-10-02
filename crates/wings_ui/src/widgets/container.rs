use bevy::prelude::*;
use crate::components::UiVisibility;
use crate::prelude::UiContainer;
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug)]
pub struct UiContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub visibility: UiVisibility,
}

impl Default for UiContainerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: Color::BEIGE,
            ..default()
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiContainerBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiContainer,
}

impl Default for UiContainerBundle {
    fn default() -> Self {
        UiContainerBundle::from(UiContainerProps::default())
    }
}

impl UiContainerBundle {
    pub fn from(props: UiContainerProps) -> Self {
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
            ..default()
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiContainerProps { visibility, ..default() })
    }

    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiContainerProps {
            width,
            height,
            ..default()
        })
    }

    pub fn from_size_splat(val: Val) -> Self {
        Self::from(UiContainerProps {
            width: val,
            height: val,
            ..default()
        })
    }

    pub fn from_width(width: Val) -> Self {
        Self::from(UiContainerProps {
            width,
            ..default()
        })
    }

    pub fn from_height(height: Val) -> Self {
        Self::from(UiContainerProps {
            height,
            ..default()
        })
    }

    pub fn from_color(color: Color) -> Self {
        Self::from(UiContainerProps {
            color,
            ..default()
        })
    }
}
