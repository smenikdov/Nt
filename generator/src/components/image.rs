use super::interface::{
    component::{Component, ComponentContext, RenderParams},
    render_error,
    style::{ComponentAlign, ComponentStyle, RawComponentStyle, Style},
};
use tiny_skia::{Pixmap, Transform, IntSize, PixmapPaint};
use image::io::Reader as ImageReader; // Используем crate `image` для загрузки изображений
use crate::edges::padding::Padding;
use std::env;
use std::path::{Path, PathBuf};

pub const IMAGE_PADDING: f32 = 10.;

pub struct Image {
    path: String,
    width: f32,
    children: Vec<Box<dyn Component>>,
}


fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home_dir) = env::home_dir() {
            // Заменяем `~` на домашнюю директорию
            let without_tilde = path.strip_prefix("~/").unwrap();
            return PathBuf::from(home_dir).join(without_tilde);
        }
    }
    PathBuf::from(path)
}

impl Component for Image {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
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
        parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let x = render_params.x;
        let y = render_params.y;
        let w = style.width;

        // Загружаем изображение с помощью библиотеки image
        let img = ImageReader::open(expand_tilde(&self.path))
            .map_err(|e| render_error::RenderError::Other(format!("Failed to open image: {}", e)))?
            .decode()
            .map_err(|e| render_error::RenderError::Other(format!("Failed to decode image: {}", e)))?;
        let img = img.to_rgba8(); // Преобразуем изображение в формат RGBA

        // Преобразуем изображение в Pixmap и отрисовываем
        let img_width = img.width() as f32;
        let img_height = img.height() as f32;
        let scale_factor = w / img_width; // Масштабирование по ширине
        let scaled_height = img_height * scale_factor;

        let img_pixmap = Pixmap::from_vec(
            img.into_vec(),
            IntSize::from_wh(img_width as u32, img_height as u32).ok_or_else(|| render_error::RenderError::Other("Invalid image size".to_string()))?
        ).ok_or_else(|| render_error::RenderError::Other("Invalid image data".to_string()))?;

        // Отрисовываем изображение с учётом масштаба и padding
        pixmap.draw_pixmap(
            (x + parent_style.width) as i32,
            (y + img_height * 2) as i32,
            img_pixmap.as_ref(),            // Используем as_ref(), чтобы передать ссылку на Pixmap
            &PixmapPaint::default(),         // Используем PixmapPaint по умолчанию
            Transform::from_scale(scale_factor, scale_factor),
            None,
        );

        Ok(())
    }
}

impl Image {
    pub fn new(path: String, width: f32) -> Image {
        Image {
            path,
            width,
            children: vec![],
        }
    }
}
