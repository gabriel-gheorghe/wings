use bevy::prelude::*;
use crate::classes::edge_insets::UiEdgeInsets;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiPadding(pub UiEdgeInsets);

#[derive(Copy, Clone, Debug, Default)]
pub struct UiPaddingProps {
    pub padding: UiEdgeInsets,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiPaddingBundle {
    pub child: UiWidgetBundle,
    pub padding: UiPadding,
}

impl Default for UiPaddingBundle {
    #[inline]
    fn default() -> Self {
        UiPaddingBundle::from(UiPaddingProps::default())
    }
}

impl UiPaddingBundle {
    #[inline]
    pub fn from(props: UiPaddingProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::new(
                        props.padding.left,
                        props.padding.right,
                        props.padding.top,
                        props.padding.bottom,
                    ),
                    ..default()
                },
                ..default()
            },
            padding: UiPadding(props.padding),
        }
    }

    #[inline]
    pub fn from_edge(padding: UiEdgeInsets) -> Self {
        Self::from(UiPaddingProps { padding })
    }
}
