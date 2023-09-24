use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Copy, Clone, Debug)]
pub struct UiScreenProps {
    pub width: Val,
    pub height: Val,
    pub centered: bool,
    pub is_collapsed: bool,
}

impl Default for UiScreenProps {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            centered: false,
            is_collapsed: false,
        }
    }
}

#[derive(Bundle)]
pub struct UiScreen {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
}

impl Default for UiScreen {
    fn default() -> Self {
        UiScreen::from(UiScreenProps::default())
    }
}

impl UiScreen {
    pub fn from(props: UiScreenProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    justify_content: if props.centered {
                        JustifyContent::Center
                    } else {
                        JustifyContent::FlexStart
                    },
                    align_items: if props.centered {
                        AlignItems::Center
                    } else {
                        AlignItems::FlexStart
                    },
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility {
                cached_width: props.width,
                cached_height: props.height,
                is_collapsed: props.is_collapsed,
            },
        }
    }
}
