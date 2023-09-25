use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Bundle)]
pub struct UiCenter {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl UiCenter {
    pub fn new() -> Self {
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
