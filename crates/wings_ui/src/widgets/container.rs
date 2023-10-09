use bevy::prelude::*;
use wings_utils::color::get_transparent_color;
use crate::classes::decoration::BoxDecoration;
use crate::prelude::UiEdgeInsets;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiContainer;

#[derive(Clone, Debug)]
pub struct UiContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Option<Color>,
    pub decoration: Option<BoxDecoration>,
    pub margin: UiEdgeInsets,
    pub padding: UiEdgeInsets,
}

impl Default for UiContainerProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: None,
            decoration: None,
            margin: UiEdgeInsets::default(),
            padding: UiEdgeInsets::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiContainerBundle {
    pub child: UiWidgetBundle,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub image: UiImage,
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
        assert!(props.color.is_none() || props.decoration.is_none(),
            "Cannot provide both a color and a decoration.
            To provide both, use the color from the decoration."
        );

        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    margin: props.margin.to_ui_rect(),
                    padding: props.padding.to_ui_rect(),
                    border: if props.decoration.is_some() {
                        props.decoration.clone().unwrap().border.to_ui_rect()
                    } else {
                        UiRect::default()
                    },
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(if props.decoration.is_some() {
                props.decoration.clone().unwrap().color
            } else {
                match props.color {
                    Some(color) => color,
                    None => Color::BISQUE,
                }
            }),
            border_color: if props.decoration.is_some() {
                BorderColor::from(props.decoration.unwrap().border.bottom.color) // todo. all border colors
            } else {
                BorderColor::from(get_transparent_color())
            },
            image: Default::default(),
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
            color: Some(color),
            ..default()
        })
    }

    #[inline]
    pub fn from_color_sized(color: Color, width: Val, height: Val) -> Self {
        Self::from(UiContainerProps {
            color: Some(color),
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_squared(color: Color, size: Val) -> Self {
        Self::from(UiContainerProps {
            color: Some(color),
            width: size,
            height: size,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_relative(color: Color) -> Self {
        Self::from(UiContainerProps {
            color: Some(color),
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

    #[inline]
    pub fn from_decoration(decoration: BoxDecoration) -> Self {
        Self::from(UiContainerProps {
            decoration: Some(decoration),
            ..default()
        })
    }
}
