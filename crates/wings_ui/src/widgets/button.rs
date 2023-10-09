use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct FlatButtonWidget;

#[derive(Copy, Clone, Debug)]
pub struct FlatButtonProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub border: UiRect,
    pub border_color: Color,
}

impl Default for FlatButtonProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(270.0),
            height: Val::Px(90.0),
            color: Color::TEAL,
            border: UiRect::all(Val::Px(5.0)),
            border_color: Color::BLACK,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct FlatButtonBundle {
    child: WidgetBundle,
    button: Button,
    interaction: Interaction,
    background_color: BackgroundColor,
    border_color: BorderColor,
    image: UiImage,
    widget: FlatButtonWidget,
}

impl Default for FlatButtonBundle {
    #[inline]
    fn default() -> Self {
        FlatButtonBundle::from(FlatButtonProps::default())
    }
}

impl FlatButtonBundle {
    #[inline]
    pub fn from(props: FlatButtonProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: props.border,
                    ..default()
                },
                ..default()
            },
            button: Default::default(),
            interaction: Default::default(),
            background_color: BackgroundColor::from(props.color),
            border_color: BorderColor(props.border_color),
            image: Default::default(),
            widget: FlatButtonWidget::default(),
        }
    }

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(FlatButtonProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(val: Val) -> Self {
        Self::from(FlatButtonProps {
            width: val,
            height: val,
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(FlatButtonProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(FlatButtonProps {
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self::from(FlatButtonProps {
            color,
            ..default()
        })
    }

    #[inline]
    pub fn from_border(border: UiRect, border_color: Color) -> Self {
        Self::from(FlatButtonProps {
            border,
            border_color,
            ..default()
        })
    }
}
