use bevy::prelude::*;
use crate::classes::edge_insets::UiEdgeInsets;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiPadding(pub UiEdgeInsets);

#[derive(Bundle, Clone, Debug)]
pub struct UiPaddingBundle {
    pub child: UiWidgetBundle,
    pub padding: UiPadding,
}

impl Default for UiPaddingBundle {
    fn default() -> Self {
        UiPaddingBundle::from(UiEdgeInsets::default())
    }
}

impl UiPaddingBundle {
    pub fn from(props: UiEdgeInsets) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::new(
                        props.left,
                        props.right,
                        props.top,
                        props.bottom,
                    ),
                    ..default()
                },
                ..default()
            },
            padding: UiPadding(props),
        }
    }
}
