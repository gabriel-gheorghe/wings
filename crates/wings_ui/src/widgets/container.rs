use bevy::prelude::*;
use crate::prelude::UiEdgeInsets;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiContainer;

#[derive(Copy, Clone, Debug)]
pub struct UiContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub margin: UiEdgeInsets,
    pub padding: UiEdgeInsets,
}

impl Default for UiContainerProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: Color::BEIGE,
            margin: UiEdgeInsets::default(),
            padding: UiEdgeInsets::default(),
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
                    margin: UiRect::new(
                        props.margin.left,
                        props.margin.right,
                        props.margin.top,
                        props.margin.bottom,
                    ),
                    padding: UiRect::new(
                        props.padding.left,
                        props.padding.right,
                        props.padding.top,
                        props.padding.bottom,
                    ),
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

    #[inline]
    pub fn from_margin(margin: UiEdgeInsets) -> Self {
        Self::from(UiContainerProps {
            margin,
            ..default()
        })
    }

    #[inline]
    pub fn from_padding(padding: UiEdgeInsets) -> Self {
        Self::from(UiContainerProps {
            padding,
            ..default()
        })
    }

    #[inline]
    pub fn from_margin_padding(margin: UiEdgeInsets, padding: UiEdgeInsets) -> Self {
        Self::from(UiContainerProps {
            margin,
            padding,
            ..default()
        })
    }
}
