use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct SizedBoxWidget;

#[derive(Copy, Clone, Debug)]
pub struct SizedBoxProps {
    pub width: Val,
    pub height: Val,
}

impl Default for SizedBoxProps {
    #[inline]
    fn default() -> Self {
        Self {
            width: Val::Px(100.0),
            height: Val::Px(100.0),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SizedBoxBundle {
    child: WidgetBundle,
    widget: SizedBoxWidget,
}

impl Default for SizedBoxBundle {
    #[inline]
    fn default() -> Self {
        SizedBoxBundle::from(SizedBoxProps::default())
    }
}

impl SizedBoxBundle {
    #[inline]
    pub fn from(props: SizedBoxProps) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    width: props.width,
                    height: props.height,
                    ..default()
                },
                ..default()
            },
            widget: SizedBoxWidget::default(),
        }
    }

    #[inline]
    pub fn from_size(width: Val, height: Val) -> Self {
        Self::from(SizedBoxProps {
            width,
            height,
            ..default()
        })
    }

    #[inline]
    pub fn from_size_splat(val: Val) -> Self {
        Self::from(SizedBoxProps {
            width: val,
            height: val,
            ..default()
        })
    }

    #[inline]
    pub fn from_width(width: Val) -> Self {
        Self::from(SizedBoxProps {
            width,
            ..default()
        })
    }

    #[inline]
    pub fn from_height(height: Val) -> Self {
        Self::from(SizedBoxProps {
            height,
            ..default()
        })
    }
}
