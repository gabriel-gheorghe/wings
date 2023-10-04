use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiContainer;

#[derive(Copy, Clone, Debug)]
pub struct UiContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
}

impl Default for UiContainerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: Color::BEIGE,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiContainerBundle {
    pub child: UiWidgetBundle,
    pub background_color: BackgroundColor,
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
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(props.color),
            internal_tag: UiContainer::default(),
        }
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
