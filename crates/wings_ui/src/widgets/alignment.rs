use bevy::prelude::*;
use crate::classes::alignment::UiAlignment;
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiAlign(UiAlignment);

#[derive(Copy, Clone, Debug, Default)]
pub struct UiAlignProps {
    pub alignment: UiAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiAlignBundle {
    pub child: UiWidgetBundle,
    pub align: UiAlign,
}

impl Default for UiAlignBundle {
    #[inline]
    fn default() -> Self {
        UiAlignBundle::from(UiAlignProps::default())
    }
}

impl UiAlignBundle {
    #[inline]
    pub fn from(props: UiAlignProps) -> Self {
        let justify_content = if props.alignment.x < 0. {
            JustifyContent::Start
        } else if props.alignment.x == 0. {
            JustifyContent::Center
        } else {
            JustifyContent::End
        };

        let align_items = if props.alignment.y < 0. {
            AlignItems::Start
        } else if props.alignment.y == 0. {
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
            align: UiAlign(props.alignment),
        }
    }

    #[inline]
    pub fn from_alignment(alignment: UiAlignment) -> Self {
        Self::from(UiAlignProps { alignment })
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiCenter;

#[derive(Bundle, Clone, Debug)]
pub struct UiCenterBundle {
    pub child: UiWidgetBundle,
    internal_tag_0: UiAlign,
    internal_tag_1: UiCenter,
}

impl Default for UiCenterBundle {
    #[inline]
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
            internal_tag_0: UiAlign::default(),
            internal_tag_1: UiCenter::default(),
        }
    }
}
