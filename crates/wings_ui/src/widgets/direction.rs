use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Bundle)]
pub struct UiColumn {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl Default for UiColumn {
    fn default() -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
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

#[derive(Bundle)]
pub struct UiRow {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl Default for UiRow {
    fn default() -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
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
