use bevy::prelude::*;
use crate::enums::LayoutVisibility;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct VisibleWidget(pub bool);

#[derive(Clone, Debug)]
pub struct VisibleProps {
    pub visible: bool,
}

impl Default for VisibleProps {
    #[inline]
    fn default() -> Self {
        Self {
            visible: true,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct VisibleBundle {
    child: WidgetBundle,
    widget: VisibleWidget,
}

impl Default for VisibleBundle {
    #[inline]
    fn default() -> Self {
        VisibleBundle::from(VisibleProps::default())
    }
}

impl VisibleBundle {
    #[inline]
    pub fn from(props: VisibleProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    display: if props.visible { Display::Flex } else { Display::None },
                    ..default()
                },
                ..default()
            },
            widget: VisibleWidget(props.visible),
        }
    }

    #[inline]
    pub fn from_visible(visible: bool) -> Self {
        Self::from(VisibleProps {
            visible
        })
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct LayoutVisibilityWidget(pub LayoutVisibility);

#[derive(Clone, Debug)]
pub struct LayoutVisibilityProps {
    pub layout_visibility: LayoutVisibility,
}

impl Default for LayoutVisibilityProps {
    #[inline]
    fn default() -> Self {
        Self {
            layout_visibility: LayoutVisibility::default(),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct LayoutVisibilityBundle {
    child: WidgetBundle,
    widget: LayoutVisibilityWidget,
}

impl Default for LayoutVisibilityBundle {
    #[inline]
    fn default() -> Self {
        LayoutVisibilityBundle::from(LayoutVisibilityProps::default())
    }
}

impl LayoutVisibilityBundle {
    #[inline]
    pub fn from(props: LayoutVisibilityProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    display: get_computed_display(&props.layout_visibility),
                    ..default()
                },
                visibility: get_computed_visibility(&props.layout_visibility),
                ..default()
            },
            widget: LayoutVisibilityWidget(props.layout_visibility),
        }
    }

    #[inline]
    pub fn from_layout_visibility(layout_visibility: LayoutVisibility) -> Self {
        Self::from(LayoutVisibilityProps {
            layout_visibility
        })
    }
}

#[inline]
pub(crate) fn get_computed_display(visibility: &LayoutVisibility) -> Display {
    match visibility {
        LayoutVisibility::Collapsed => Display::None,
        _ => Display::Flex,
    }
}

#[inline]
pub(crate) fn get_computed_visibility(visibility: &LayoutVisibility) -> Visibility {
    match visibility {
        LayoutVisibility::Visible => Visibility::Visible,
        LayoutVisibility::Hidden => Visibility::Hidden,
        _ => Visibility::Inherited,
    }
}
