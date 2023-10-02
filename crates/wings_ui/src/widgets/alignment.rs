use bevy::prelude::*;
use crate::components::UiVisibility;
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Bundle, Clone, Debug)]
pub struct UiCenter {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl Default for UiCenter {
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
        }
    }
}

impl UiCenter {
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
        }
    }
}
