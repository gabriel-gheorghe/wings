use bevy::prelude::*;
use crate::classes::edge_insets::EdgeInsets;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct PaddingWidget(pub EdgeInsets);

#[derive(Copy, Clone, Debug, Default)]
pub struct PaddingProps {
    pub padding: EdgeInsets,
}

#[derive(Bundle, Clone, Debug)]
pub struct PaddingBundle {
    child: WidgetBundle,
    widget: PaddingWidget,
}

impl Default for PaddingBundle {
    #[inline]
    fn default() -> Self {
        PaddingBundle::from(PaddingProps::default())
    }
}

impl PaddingBundle {
    #[inline]
    pub fn from(props: PaddingProps) -> Self {
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
            widget: PaddingWidget(props.padding),
        }
    }

    #[inline]
    pub fn from_edge(padding: EdgeInsets) -> Self {
        Self::from(PaddingProps { padding })
    }
}
