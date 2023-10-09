use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiHorizontalDivider;

#[derive(Copy, Clone, Debug)]
pub struct UiHorizontalDividerProps {
    pub width: Val,
}

impl Default for UiHorizontalDividerProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiHorizontalDividerBundle {
    pub child: WidgetBundle,
    pub background_color: BackgroundColor,
    internal_tag: UiHorizontalDivider,
}

impl Default for UiHorizontalDividerBundle {
    #[inline]
    fn default() -> Self {
        UiHorizontalDividerBundle::from(UiHorizontalDividerProps {
            width: Val::Px(3.),
        })
    }
}

impl UiHorizontalDividerBundle {
    #[inline]
    pub fn from(props: UiHorizontalDividerProps) -> Self {
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
            internal_tag: UiHorizontalDivider::default(),
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
pub struct UiVerticalDivider;

#[derive(Copy, Clone, Debug)]
pub struct UiVerticalDividerProps {
    pub height: Val,
}

impl Default for UiVerticalDividerProps {
    #[inline]
    fn default() -> Self {
        Self {
            height: Val::Px(3.),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiVerticalDividerBundle {
    pub child: WidgetBundle,
    pub background_color: BackgroundColor,
    internal_tag: UiVerticalDivider,
}

impl Default for UiVerticalDividerBundle {
    #[inline]
    fn default() -> Self {
        UiVerticalDividerBundle::from(UiVerticalDividerProps {
            height: Val::Px(3.),
        })
    }
}

impl UiVerticalDividerBundle {
    #[inline]
    pub fn from(props: UiVerticalDividerProps) -> Self {
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
            internal_tag: UiVerticalDivider::default(),
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
