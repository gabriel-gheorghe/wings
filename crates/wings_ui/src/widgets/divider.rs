use bevy::prelude::*;
use crate::components::UiVisibility;
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug)]
pub struct UiHorizontalDividerProps {
    pub width: Val,
    pub visibility: UiVisibility,
}

impl Default for UiHorizontalDividerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            ..default()
        }
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct UiHorizontalDivider {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl UiHorizontalDivider {
    pub fn from(props: UiHorizontalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    width: props.width,
                    ..default()
                },
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiHorizontalDividerProps { visibility, ..default() })
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
            ..default()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UiVerticalDividerProps {
    pub height: Val,
    pub visibility: UiVisibility,
}

impl Default for UiVerticalDividerProps {
    fn default() -> Self {
        Self {
            height: Val::Px(100.0),
            ..default()
        }
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct UiVerticalDivider {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl UiVerticalDivider {
    pub fn from(props: UiVerticalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    height: props.height,
                    ..default()
                },
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiVerticalDividerProps { visibility, ..default() })
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
            ..default()
        }
    }
}
