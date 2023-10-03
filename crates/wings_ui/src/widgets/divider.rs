use bevy::prelude::*;
use crate::widgets::UiVisibility;
use crate::prelude::{UiHorizontalDivider, UiVerticalDivider};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug)]
pub struct UiHorizontalDividerProps {
    pub width: Val,
    pub visibility: UiVisibility,
}

impl Default for UiHorizontalDividerProps {
    fn default() -> Self {
        Self {
            width: Val::Px(3.),
            visibility: UiVisibility::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiHorizontalDividerBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiHorizontalDivider,
}

impl Default for UiHorizontalDividerBundle {
    fn default() -> Self {
        UiHorizontalDividerBundle::from(UiHorizontalDividerProps {
            width: Val::Px(3.),
            visibility: UiVisibility::default(),
        })
    }
}

impl UiHorizontalDividerBundle {
    pub fn from(props: UiHorizontalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    width: props.width,
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiHorizontalDivider::default(),
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
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
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
            height: Val::Px(3.),
            visibility: UiVisibility::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiVerticalDividerBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiVerticalDivider,
}

impl Default for UiVerticalDividerBundle {
    fn default() -> Self {
        UiVerticalDividerBundle::from(UiVerticalDividerProps {
            height: Val::Px(3.),
            visibility: UiVisibility::default(),
        })
    }
}

impl UiVerticalDividerBundle {
    pub fn from(props: UiVerticalDividerProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    width: Val::Percent(100.),
                    height: props.height,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiVerticalDivider::default(),
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiVerticalDividerProps { visibility, ..default() })
    }

    pub fn from_height(height: Val) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            },
            ..default()
        }
    }
}
