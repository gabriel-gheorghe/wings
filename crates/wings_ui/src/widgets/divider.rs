use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct HorizontalDividerWidget;

#[derive(Copy, Clone, Debug)]
pub struct HorizontalDividerProps {
    pub width: Val,
}

impl Default for HorizontalDividerProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct HorizontalDividerBundle {
    child: WidgetBundle,
    background_color: BackgroundColor,
    widget: HorizontalDividerWidget,
}

impl Default for HorizontalDividerBundle {
    #[inline]
    fn default() -> Self {
        HorizontalDividerBundle::from(HorizontalDividerProps {
            width: Val::Px(3.),
        })
    }
}

impl HorizontalDividerBundle {
    #[inline]
    pub fn from(props: HorizontalDividerProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: props.width,
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            widget: HorizontalDividerWidget::default(),
        }
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width,
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            ..default()
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct VerticalDividerWidget;

#[derive(Copy, Clone, Debug)]
pub struct VerticalDividerProps {
    pub height: Val,
}

impl Default for VerticalDividerProps {
    #[inline]
    fn default() -> Self {
        Self {
            height: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct VerticalDividerBundle {
    child: WidgetBundle,
    background_color: BackgroundColor,
    widget: VerticalDividerWidget,
}

impl Default for VerticalDividerBundle {
    #[inline]
    fn default() -> Self {
        VerticalDividerBundle::from(VerticalDividerProps {
            height: Val::Px(3.),
        })
    }
}

impl VerticalDividerBundle {
    #[inline]
    pub fn from(props: VerticalDividerProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            widget: VerticalDividerWidget::default(),
        }
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            ..default()
        }
    }
}
