use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use wings_utils::color::{get_random_color, get_random_color_with_alpha};

type TextQueryType<'w, 's, T> = Query<'w, 's,
    (
        Entity,
        &'static mut Text,
    ),
    With<T>,
>;

#[derive(SystemParam)]
pub struct TextQuery<'w, 's, T: Component>(TextQueryType<'w, 's, T>);

impl <'w, 's, T: Component> TextQuery<'w, 's, T> {
    #[inline]
    pub fn get(&self) -> &TextQueryType<'w, 's, T> { &self.0 }

    #[inline]
    pub fn get_mut(&mut self) -> &mut TextQueryType<'w, 's, T> { &mut self.0 }

    #[inline]
    pub fn get_text(&mut self, target: Entity) -> String {
        let mut res = "".to_string();
        self.0.for_each_mut(|(entity, c_text)| {
            if entity == target {
                res = c_text.sections.first().unwrap().value.clone();
            }
        });
        res
    }

    #[inline]
    pub fn set_text<F>(&mut self, f: F)
    where
        F: FnOnce(String) -> String + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_text)| {
            let text = f(c_text.sections.first().cloned().unwrap().value);
            c_text.sections.first_mut().unwrap().value = text;
        });
    }

    #[inline]
    pub fn set_text_single(&mut self, text: String, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_text)| {
            if entity == target {
                c_text.sections.first_mut().unwrap().value = text.clone();
            }
        });
    }

    #[inline]
    pub fn get_font_size(&mut self, target: Entity) -> f32 {
        let mut res = 0.;
        self.0.for_each_mut(|(entity, c_text)| {
            if entity == target {
                res = c_text.sections.first().unwrap().style.font_size;
            }
        });
        res
    }

    #[inline]
    pub fn set_font_size<F>(&mut self, f: F)
    where
        F: FnOnce(f32) -> f32 + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_text)| {
            let font_size
                = &f(c_text.sections.first().cloned().unwrap().style.font_size);
            c_text.sections.first_mut().unwrap().style.font_size = *font_size;
        });
    }

    #[inline]
    pub fn set_font_size_single(&mut self, font_size: f32, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_text)| {
            if entity == target {
                c_text.sections.first_mut().unwrap().style.font_size = font_size;
            }
        });
    }

    #[inline]
    pub fn get_color(&mut self, target: Entity) -> Color {
        let mut res = Color::WHITE;
        self.0.for_each_mut(|(entity, c_text)| {
            if entity == target {
                res = c_text.sections.first().unwrap().style.color;
            }
        });
        res
    }

    #[inline]
    pub fn set_color<F>(&mut self, f: F)
    where
        F: FnOnce(Color) -> Color + Copy + Clone
    {
        self.0.for_each_mut(|(_, mut c_text)| {
            let color = &f(c_text.sections.first().unwrap().style.color);
            c_text.sections.first_mut().unwrap().style.color = *color;
        });
    }

    #[inline]
    pub fn set_random_color(&mut self) {
        self.0.for_each_mut(|(_, mut c_text)| {
            c_text.sections.first_mut().unwrap().style.color = get_random_color();
        });
    }

    #[inline]
    pub fn set_random_color_with_alpha(&mut self) {
        self.0.for_each_mut(|(_, mut c_text)| {
            c_text.sections.first_mut().unwrap().style.color = get_random_color_with_alpha(None);
        });
    }

    #[inline]
    pub fn set_random_color_equally(&mut self) {
        self.set_color(|_| get_random_color());
    }

    #[inline]
    pub fn set_random_color_with_alpha_equally(&mut self) {
        self.set_color(|_| get_random_color_with_alpha(None));
    }

    #[inline]
    pub fn set_color_single(&mut self, target: Entity, color: Color) {
        self.0.for_each_mut(|(entity, mut c_text)| {
            if entity == target {
                c_text.sections.first_mut().unwrap().style.color = color;
            }
        });
    }

    #[inline]
    pub fn set_random_color_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_text)| {
            if entity == target {
                c_text.sections.first_mut().unwrap().style.color = get_random_color();
            }
        });
    }

    #[inline]
    pub fn set_random_color_with_alpha_single(&mut self, target: Entity) {
        self.0.for_each_mut(|(entity, mut c_text)| {
            if entity == target {
                c_text.sections.first_mut().unwrap().style.color
                    = get_random_color_with_alpha(None);
            }
        });
    }
}