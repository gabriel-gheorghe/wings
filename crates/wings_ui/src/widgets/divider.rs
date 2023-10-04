use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiHorizontalDivider;

#[derive(Copy, Clone, Debug)]
pub struct UiHorizontalDividerProps {
    pub width: Val,
}

impl Default for UiHorizontalDividerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiHorizontalDividerBundle {
    pub child: UiWidgetBundle,
    pub background_color: BackgroundColor,
    internal_tag: UiHorizontalDivider,
}

impl Default for UiHorizontalDividerBundle {
    fn default() -> Self {
        UiHorizontalDividerBundle::from(UiHorizontalDividerProps {
            width: Val::Px(3.),
        })
    }
}

impl UiHorizontalDividerBundle {
    pub fn from(props: UiHorizontalDividerProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            internal_tag: UiHorizontalDivider::default(),
        }
    }

    pub fn from_width(width: Val) -> Self {
        Self {
            child: UiWidgetBundle {
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
pub struct UiVerticalDivider;

#[derive(Copy, Clone, Debug)]
pub struct UiVerticalDividerProps {
    pub height: Val,
}

impl Default for UiVerticalDividerProps {
    fn default() -> Self {
        Self {
            height: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiVerticalDividerBundle {
    pub child: UiWidgetBundle,
    pub background_color: BackgroundColor,
    internal_tag: UiVerticalDivider,
}

impl Default for UiVerticalDividerBundle {
    fn default() -> Self {
        UiVerticalDividerBundle::from(UiVerticalDividerProps {
            height: Val::Px(3.),
        })
    }
}

impl UiVerticalDividerBundle {
    pub fn from(props: UiVerticalDividerProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor::from(Color::BLACK),
            internal_tag: UiVerticalDivider::default(),
        }
    }

    pub fn from_height(height: Val) -> Self {
        Self {
            child: UiWidgetBundle {
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
