use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Style},
};
use tiny_skia::{Pixmap, Transform};
use std::path::Path;
use image::io::Reader as ImageReader; // Используем crate `image` для загрузки изображений
use crate::edges::padding::Padding;

pub const IMAGE_PADDING: f32 = 10.;

pub struct ImageComponent {
    path: String,
    width: f32,
}

impl Component for ImageComponent {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        // У ImageComponent нет дочерних элементов, поэтому возвращаем пустой вектор
        &Vec::new()
    }

    fn style(&self) -> RawComponentStyle {
        Style::default()
            .min_width(self.width)
            .align(ComponentAlign::Row)
            .padding(Padding::from_value(IMAGE_PADDING))
    }

    fn draw_self(
        &self,
        pixmap: &mut Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let x = render_params.x;
        let y = render_params.y;
        let w = style.width;

        // Загружаем изображение с помощью библиотеки image
        let img = ImageReader::open(&self.path)?.decode()?;
        let img = img.to_rgba8(); // Преобразуем изображение в формат RGBA

        // Преобразуем изображение в Pixmap и отрисовываем
        let img_width = img.width() as f32;
        let img_height = img.height() as f32;
        let scale_factor = w / img_width; // Масштабирование по ширине
        let scaled_height = img_height * scale_factor;

        let img_pixmap = Pixmap::from_vec(
            img.into_vec(),
            img_width as u32,
            img_height as u32
        ).ok_or_else(|| render_error::RenderError::InvalidImage)?;

        // Отрисовываем изображение с учётом масштаба и padding
        pixmap.draw_pixmap(
            x as i32,
            y as i32,
            &img_pixmap,
            &Transform::from_scale(scale_factor, scale_factor),
            None,
        );

        Ok(())
    }
}

impl ImageComponent {
    pub fn new(path: String, width: f32) -> ImageComponent {
        ImageComponent {
            path,
            width,
        }
    }
}
