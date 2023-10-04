use bevy::prelude::*;
use crate::prelude::UiWidgetBundle;
use crate::widgets::{UiCenter, UiVisibility};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Bundle, Clone, Debug)]
pub struct UiCenterBundle {
    pub child: UiWidgetBundle,
    pub visibility: UiVisibility,
    internal_tag: UiCenter,
}

impl Default for UiCenterBundle {
    fn default() -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
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
            child: UiWidgetBundle {
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
