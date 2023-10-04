use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiCenter;

#[derive(Bundle, Clone, Debug)]
pub struct UiCenterBundle {
    pub child: UiWidgetBundle,
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
            internal_tag: UiCenter::default(),
        }
    }
}
