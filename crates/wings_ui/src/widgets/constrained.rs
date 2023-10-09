use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ConstrainedHeightWidget;

#[derive(Bundle, Clone, Debug)]
pub struct ConstrainedHeightBundle {
    child: WidgetBundle,
    widget: ConstrainedHeightWidget,
}

impl Default for ConstrainedHeightBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            widget: ConstrainedHeightWidget::default(),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct ConstrainedWidthWidget;

#[derive(Bundle, Clone, Debug)]
pub struct ConstrainedWidthBundle {
    child: WidgetBundle,
    widget: ConstrainedWidthWidget,
}

impl Default for ConstrainedWidthBundle {
    #[inline]
    fn default() -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            widget: ConstrainedWidthWidget::default(),
        }
    }
}