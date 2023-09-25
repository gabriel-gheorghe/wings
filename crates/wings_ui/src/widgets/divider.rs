use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Copy, Clone, Debug)]
pub struct UiHorizontalDividerProps {
    pub width: Val,
    pub is_collapsed: bool,
}

impl Default for UiHorizontalDividerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            is_collapsed: false,
        }
    }
}

#[derive(Bundle)]
pub struct UiHorizontalDivider {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl UiHorizontalDivider {
    pub fn from(props: UiHorizontalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    width: props.width,
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility::from_width_and_collapsed(props.width, props.is_collapsed),
        }
    }

    pub fn from_width(width: Val) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    width,
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility::from_width(width),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UiVerticalDividerProps {
    pub height: Val,
    pub is_collapsed: bool,
}

impl Default for UiVerticalDividerProps {
    fn default() -> Self {
        Self {
            height: Val::Px(100.0),
            is_collapsed: false,
        }
    }
}

#[derive(Bundle)]
pub struct UiVerticalDivider {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl UiVerticalDivider {
    pub fn from(props: UiVerticalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility::from_height_and_collapsed(props.height, props.is_collapsed),
        }
    }

    pub fn from_height(height: Val) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    height,
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility::from_height(height),
        }
    }
}
