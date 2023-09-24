use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Copy, Clone, Debug)]
pub struct UiContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub is_collapsed: bool,
}

impl Default for UiContainerProps {
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
pub struct UiContainer {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl Default for UiContainer {
    fn default() -> Self {
        UiContainer::from(UiContainerProps::default())
    }
}

impl UiContainer {
    pub fn from(props: UiContainerProps) -> Self {
        Self {
            child: NodeBundle {
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
