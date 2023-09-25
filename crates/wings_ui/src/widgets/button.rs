use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Copy, Clone, Debug)]
pub struct UiButtonProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub is_collapsed: bool,
}

impl Default for UiButtonProps {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: Color::BEIGE,
            is_collapsed: false,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiButton {
    pub child: ButtonBundle,
    pub visibility: UiVisibility,
}

impl Default for UiButton {
    fn default() -> Self {
        UiButton::from(UiButtonProps::default())
    }
}

impl UiButton {
    pub fn from(props: UiButtonProps) -> Self {
        Self {
            child: ButtonBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                background_color: BackgroundColor::from(props.color),
                ..default()
            },
            visibility: UiVisibility {
                cached_width: props.width,
                cached_height: props.height,
                is_collapsed: props.is_collapsed,
            },
        }
    }
}
