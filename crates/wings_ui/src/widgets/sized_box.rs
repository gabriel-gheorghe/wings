use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiSizedBox;

#[derive(Copy, Clone, Debug)]
pub struct UiSizedBoxProps {
    pub width: Val,
    pub height: Val,
}

impl Default for UiSizedBoxProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiSizedBoxBundle {
    pub child: UiWidgetBundle,
    internal_tag: UiSizedBox,
}

impl Default for UiSizedBoxBundle {
    #[inline]
    fn default() -> Self {
        UiSizedBoxBundle::from(UiSizedBoxProps::default())
    }
}

impl UiSizedBoxBundle {
    #[inline]
    pub fn from(props: UiSizedBoxProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            internal_tag: UiSizedBox::default(),
        }
    }

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(UiSizedBoxProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(val: Val) -> Self {
        Self::from(UiSizedBoxProps {
            width: val,
            height: val,
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(UiSizedBoxProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(UiSizedBoxProps {
            height,
            ..default()
        })
    }
}
