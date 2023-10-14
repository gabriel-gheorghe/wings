use bevy::prelude::*;
use crate::classes::decoration::BoxDecoration;
use crate::prelude::EdgeInsets;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ContainerWidget;

#[derive(Clone, Debug)]
pub struct ContainerProps {
    pub width: Val,
    pub height: Val,
    pub color: Option<Color>,
    pub decoration: Option<BoxDecoration>,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
}

impl Default for ContainerProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            color: None,
            decoration: None,
            margin: EdgeInsets::default(),
            padding: EdgeInsets::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct ContainerBundle {
    child: WidgetBundle,
    background_color: BackgroundColor,
    border_color: BorderColor,
    image: UiImage,
    widget: ContainerWidget,
}

impl Default for ContainerBundle {
    #[inline]
    fn default() -> Self {
        ContainerBundle::from(ContainerProps::default())
    }
}

impl ContainerBundle {
    #[inline]
    pub fn from(props: ContainerProps) -> Self {
        assert!(props.color.is_none() || props.decoration.is_none(),
            "Cannot provide both a color and a decoration.
            To provide both, use the color from the decoration."
        );

        Self {
            child: WidgetBundle {
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
                BorderColor::from(Color::NONE)
            },
            image: Default::default(),
            widget: ContainerWidget::default(),
        }
    }

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(ContainerProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(size: Val) -> Self {
        Self::from(ContainerProps {
            width: size,
            height: size,
            ..default()
        })
    }

    #[inline]
    pub fn from_relative_size() -> Self {
        Self::from(ContainerProps {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(ContainerProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(ContainerProps {
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self::from(ContainerProps {
            color: Some(color),
            ..default()
        })
    }

    #[inline]
    pub fn from_color_sized(color: Color, width: Val, height: Val) -> Self {
        Self::from(ContainerProps {
            color: Some(color),
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_squared(color: Color, size: Val) -> Self {
        Self::from(ContainerProps {
            color: Some(color),
            width: size,
            height: size,
            ..default()
        })
    }

    #[inline]
    pub fn from_color_relative(color: Color) -> Self {
        Self::from(ContainerProps {
            color: Some(color),
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
    }

    #[inline]
    pub fn from_margin(margin: EdgeInsets) -> Self {
        Self::from(ContainerProps {
            margin,
            ..default()
        })
    }

    #[inline]
    pub fn from_padding(padding: EdgeInsets) -> Self {
        Self::from(ContainerProps {
            padding,
            ..default()
        })
    }

    #[inline]
    pub fn from_margin_padding(margin: EdgeInsets, padding: EdgeInsets) -> Self {
        Self::from(ContainerProps {
            margin,
            padding,
            ..default()
        })
    }

    #[inline]
    pub fn from_decoration(decoration: BoxDecoration) -> Self {
        Self::from(ContainerProps {
            decoration: Some(decoration),
            ..default()
        })
    }
}
