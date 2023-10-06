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
    #[inline]
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
    #[inline]
    fn default() -> Self {
        UiContainerBundle::from(UiContainerProps::default())
    }
}

impl UiContainerBundle {
    #[inline]
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

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiContainerProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(size: Val) -> Self {
        Self::from(UiContainerProps {
            width: size,
            height: size,
            ..default()
        })
    }

    #[inline]
    pub fn from_relative_size() -> Self {
        Self::from(UiContainerProps {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(UiContainerProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(UiContainerProps {
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self::from(UiContainerProps {
            color,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_sized(color: Color, width: Val, height: Val) -> Self {
        Self::from(UiContainerProps {
            color,
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_squared(color: Color, size: Val) -> Self {
        Self::from(UiContainerProps {
            color,
            width: size,
            height: size,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_relative(color: Color) -> Self {
        Self::from(UiContainerProps {
            color,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
    }
}
