use bevy::prelude::*;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiAlign(pub f32, pub f32);

#[derive(Copy, Clone, Debug)]
pub struct UiAlignProps {
    pub x: f32,
    pub y: f32,
}

impl UiAlignProps {
    pub const TOP_LEFT: UiAlignProps = UiAlignProps { x: -1. , y: -1. };
    pub const TOP_CENTER: UiAlignProps = UiAlignProps { x: 0. , y: -1. };
    pub const TOP_RIGHT: UiAlignProps = UiAlignProps { x: 1. , y: -1. };

    pub const CENTER_LEFT: UiAlignProps = UiAlignProps { x: -1. , y: 0. };
    pub const CENTER: UiAlignProps = UiAlignProps { x: 0. , y: 0. };
    pub const CENTER_RIGHT: UiAlignProps = UiAlignProps { x: 1. , y: 0. };

    pub const BOTTOM_LEFT: UiAlignProps = UiAlignProps { x: -1. , y: 1. };
    pub const BOTTOM_CENTER: UiAlignProps = UiAlignProps { x: 0. , y: 1. };
    pub const BOTTOM_RIGHT: UiAlignProps = UiAlignProps { x: 1. , y: 1. };
}

#[derive(Bundle, Clone, Debug)]
pub struct UiAlignBundle {
    pub child: UiWidgetBundle,
    pub align: UiAlign,
}

impl Default for UiAlignBundle {
    fn default() -> Self {
        UiAlignBundle::from(UiAlignProps::TOP_LEFT)
    }
}

impl UiAlignBundle {
    pub fn from(props: UiAlignProps) -> Self {
        let justify_content = if props.x < 0. {
            JustifyContent::Start
        } else if props.x == 0. {
            JustifyContent::Center
        } else {
            JustifyContent::End
        };

        let align_items = if props.y < 0. {
            AlignItems::Start
        } else if props.y == 0. {
            AlignItems::Center
        } else {
            AlignItems::End
        };

        // todo. issue. maybe compute on a separate system by reading child size
        /*let padding = match (justify_content, align_items) {
            (JustifyContent::Start, AlignItems::Start) => UiRect::percent((props.x + 1.) * 50., 0., (props.y + 1.) * 50., 0.),
            (JustifyContent::Start, AlignItems::Center) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::Start, AlignItems::End) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::Center, AlignItems::Start) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::Center, AlignItems::Center) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::Center, AlignItems::End) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::End, AlignItems::Start) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::End, AlignItems::Center) => UiRect::percent(0., 0., 0., 0.),
            (JustifyContent::End, AlignItems::End) => UiRect::percent(0., 0., 0., 0.),
            (_, _) => UiRect::percent(0., 0., 0., 0.),
        };*/

        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content,
                    align_items,
                    //padding,
                    ..default()
                },
                ..default()
            },
            align: UiAlign(props.x, props.y),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiCenterBundle {
    pub child: UiWidgetBundle,
    internal_tag: UiAlign,
}

impl Default for UiCenterBundle {
    fn default() -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            internal_tag: UiAlign::default(),
        }
    }
}
