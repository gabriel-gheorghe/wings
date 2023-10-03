use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::components::{UiScreen, UiVisibility};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug)]
pub struct UiScreenProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub centered: bool,
    pub visibility: UiVisibility,
}

impl Default for UiScreenProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            color: get_transparent_color(),
            centered: false,
            visibility: UiVisibility::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiScreenBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiScreen,
}

impl Default for UiScreenBundle {
    fn default() -> Self {
        UiScreenBundle::from(UiScreenProps::default())
    }
}

impl UiScreenBundle {
    pub fn from(props: UiScreenProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    width: props.width,
                    height: props.height,
                    justify_content: if props.centered {
                        JustifyContent::Center
                    } else {
                        JustifyContent::FlexStart
                    },
                    align_items: if props.centered {
                        AlignItems::Center
                    } else {
                        AlignItems::FlexStart
                    },
                    ..default()
                },
                background_color: BackgroundColor::from(props.color),
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiScreen::default(),
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiScreenProps { visibility, ..default() })
    }

    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiScreenProps {
            width,
            height,
            ..default()
        })
    }

    pub fn from_size_splat(val: Val) -> Self {
        Self::from(UiScreenProps {
            width: val,
            height: val,
            ..default()
        })
    }

    pub fn from_width(width: Val) -> Self {
        Self::from(UiScreenProps {
            width,
            ..default()
        })
    }

    pub fn from_height(height: Val) -> Self {
        Self::from(UiScreenProps {
            height,
            ..default()
        })
    }

    pub fn from_color(color: Color) -> Self {
        Self::from(UiScreenProps {
            color,
            ..default()
        })
    }
}
