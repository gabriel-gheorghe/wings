use bevy::prelude::*;
use crate::widgets::{UiConstrainedHeight, UiConstrainedWidth};

#[derive(Bundle, Clone, Debug)]
pub struct UiConstrainedHeightBundle {
    pub child: NodeBundle,
    internal_tag: UiConstrainedHeight,
}

impl Default for UiConstrainedHeightBundle {
    fn default() -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            internal_tag: UiConstrainedHeight::default(),
        }
    }
}

impl UiConstrainedHeightBundle {
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.child.background_color = BackgroundColor::from(color);
        res
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiConstrainedWidthBundle {
    pub child: NodeBundle,
    internal_tag: UiConstrainedWidth,
}

impl Default for UiConstrainedWidthBundle {
    fn default() -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            internal_tag: UiConstrainedWidth::default(),
        }
    }
}

impl UiConstrainedWidthBundle {
    pub fn with_debug_color(color: Color) -> Self {
        let mut res = Self::default();
        res.child.background_color = BackgroundColor::from(color);
        res
    }
}
