use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiVisibility(pub bool);

#[derive(Bundle, Clone, Debug)]
pub struct UiVisibilityBundle {
    pub child: UiWidgetBundle,
    pub visibility: UiVisibility,
}

impl Default for UiVisibilityBundle {
    fn default() -> Self {
        UiVisibilityBundle::from(true)
    }
}

impl UiVisibilityBundle {
    pub fn from(visible: bool) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    display: if visible { Display::Flex } else { Display::None },
                    ..default()
                },
                ..default()
            },
            visibility: UiVisibility(visible),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiLayoutVisibility {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiLayoutVisibilityBundle {
    pub child: UiWidgetBundle,
    pub visibility: UiLayoutVisibility,
}

impl Default for UiLayoutVisibilityBundle {
    fn default() -> Self {
        UiLayoutVisibilityBundle::from(UiLayoutVisibility::Inherited)
    }
}

impl UiLayoutVisibilityBundle {
    pub fn from(visibility: UiLayoutVisibility) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    display: get_computed_display(&visibility),
                    ..default()
                },
                visibility: get_computed_visibility(&visibility),
                ..default()
            },
            visibility,
        }
    }
}

pub(crate) fn get_computed_display(visibility: &UiLayoutVisibility) -> Display {
    match visibility {
        UiLayoutVisibility::Collapsed => Display::None,
        _ => Display::Flex,
    }
}

pub(crate) fn get_computed_visibility(visibility: &UiLayoutVisibility) -> Visibility {
    match visibility {
        UiLayoutVisibility::Visible => Visibility::Visible,
        UiLayoutVisibility::Hidden => Visibility::Hidden,
        _ => Visibility::Inherited,
    }
}
