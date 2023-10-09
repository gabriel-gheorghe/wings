use bevy::prelude::*;
use crate::classes::edge_insets::EdgeInsets;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiPadding(pub EdgeInsets);

#[derive(Copy, Clone, Debug, Default)]
pub struct UiPaddingProps {
    pub padding: EdgeInsets,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiPaddingBundle {
    child: WidgetBundle,
    widget: UiPadding,
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
            child: WidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: props.padding.to_ui_rect(),
                    ..default()
                },
                ..default()
            },
            widget: UiPadding(props.padding),
        }
    }

    #[inline]
    pub fn from_edge(padding: EdgeInsets) -> Self {
        Self::from(UiPaddingProps { padding })
    }
}
