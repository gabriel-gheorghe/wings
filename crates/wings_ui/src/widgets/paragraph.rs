use bevy::prelude::*;
use bevy::text::{DEFAULT_FONT_HANDLE, TextLayoutInfo};
use bevy::ui::ContentSize;
use bevy::ui::widget::TextFlags;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ParagraphWidget;

#[derive(Clone, Debug)]
pub struct ParagraphProps {
    pub text: String,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub color: Color,
}

impl Default for ParagraphProps {
    #[inline]
    fn default() -> Self {
        Self {
            text: "Empty".to_string(),
            font: DEFAULT_FONT_HANDLE.typed(),
            font_size: 24.,
            color: Color::WHITE,
        }
    }
}

#[derive(Bundle, Debug)]
pub struct ParagraphBundle {
    child: WidgetBundle,
    text: Text,
    text_layout_info: TextLayoutInfo,
    text_flags: TextFlags,
    calculated_size: ContentSize,
    widget: ParagraphWidget,
}

impl Default for ParagraphBundle {
    fn default() -> Self {
        ParagraphBundle::from(ParagraphProps::default())
    }
}

impl ParagraphBundle {
    #[inline]
    pub fn from(props: ParagraphProps) -> Self {
        Self {
            child: WidgetBundle::default(),
            text: Text {
                sections: vec![TextSection {
                    value: props.text.to_string(),
                    style: TextStyle {
                        font: props.font,
                        font_size: props.font_size,
                        color: props.color,
                    },
                }],
                ..default()
            },
            text_layout_info: TextLayoutInfo::default(),
            text_flags: TextFlags::default(),
            calculated_size: ContentSize::default(),
            widget: ParagraphWidget::default(),
        }
    }

    #[inline]
    pub fn from_text(text: String) -> Self {
        Self::from(ParagraphProps {
            text,
            ..default()
        })
    }

    #[inline]
    pub fn from_text_color(text: String, color: Color) -> Self {
        Self::from(ParagraphProps {
            text,
            color,
            ..default()
        })
    }
}
