use bevy::prelude::*;
use crate::widgets::{UiCenter, UiVisibility};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Bundle, Clone, Debug)]
pub struct UiCenterBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiCenter,
}

impl Default for UiCenterBundle {
    fn default() -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility::default(),
            internal_tag: UiCenter::default(),
        }
    }
}

impl UiCenterBundle {
    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&visibility),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                visibility: get_computed_visibility(&visibility),
                ..default()
            },
            visibility,
            internal_tag: UiCenter::default(),
        }
    }
}
