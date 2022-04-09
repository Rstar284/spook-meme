use image::imageops::overlay;
use image::{DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

const OUTLINE_THICKNESS: i32 = 3;
const FONT_SCALE: Scale = Scale { x: 50.0, y: 50.0 };
const FONT_OUTLINE_SCALE: Scale = Scale {
    x: FONT_SCALE.x + (OUTLINE_THICKNESS as f32),
    y: FONT_SCALE.y + (OUTLINE_THICKNESS as f32),
};
const TEXT_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);
const TEXT_OUTLINE_COLOR: Rgba<u8> = Rgba([0, 0, 0, 255]);

pub struct BBox {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn draw_image_mut(img: &mut DynamicImage, img2: &DynamicImage, bbox: BBox) {
    let insert_img = img2.resize(bbox.w, bbox.h, image::imageops::FilterType::Gaussian);

    let (insert_width, insert_height) = (insert_img.width(), insert_img.height());
    let (x, y) = (
        bbox.x + (bbox.w - insert_width) / 2,
        bbox.y + (bbox.h - insert_height) / 2,
    );

    overlay(img, &insert_img, x as i64, y as i64);
}

pub fn create_meme_text(font: &Font, text: &str) -> DynamicImage {
    let (width, height) = text_size(FONT_OUTLINE_SCALE, font, text);
    let mut out_img = DynamicImage::ImageRgba8(ImageBuffer::new(width as u32, height as u32));

    draw_text_centered_mut(
        &mut out_img,
        TEXT_COLOR,
        FONT_SCALE,
        font,
        text,
        TEXT_OUTLINE_COLOR,
        OUTLINE_THICKNESS,
    );
    out_img
}

pub fn draw_text_centered_mut(
    img: &mut DynamicImage,
    color: Rgba<u8>,
    scale: Scale,
    font: &Font,
    text: &str,
    stroke_color: Rgba<u8>,
    stroke_thickness: i32,
) {
    let image_width = img.width() as i32;
    let image_height = img.height() as i32;
    let (text_width, text_height) = text_size(scale, font, text);

    let x = (image_width - text_width) / 2;
    let y = (image_height - text_height) / 2;

    draw_text_mut(
        img,
        stroke_color,
        x + stroke_thickness,
        y + stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x + stroke_thickness,
        y - stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x - stroke_thickness,
        y + stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x - stroke_thickness,
        y - stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x,
        y + stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x + stroke_thickness,
        y,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x,
        y - stroke_thickness,
        scale,
        font,
        text,
    );
    draw_text_mut(
        img,
        stroke_color,
        x - stroke_thickness,
        y,
        scale,
        font,
        text,
    );

    draw_text_mut(img, color, x, y, scale, font, text);
}
