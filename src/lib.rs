pub mod image_editing;
pub mod meme_args;
pub mod meme_data;

pub use crate::meme_args::{MemeArg, MemeArgs, MemeArgsBuilder};

use crate::error::{ImageDownloadError, MemeMakeError};
use crate::image_editing::create_meme_text;
use image::DynamicImage;
use image_editing::BBox;
use meme_data::Meme;
use reqwest::{Client, Error as ReqwestError};
use rusttype::Font;

pub mod error {
    use image::ImageError;
    use reqwest::Error as ReqwestError;

    #[derive(Debug)]
    pub enum ImageDownloadError {
        ReqwestError(ReqwestError),
        ImageError(ImageError),
    }

    impl From<ImageError> for ImageDownloadError {
        fn from(err: ImageError) -> Self {
            ImageDownloadError::ImageError(err)
        }
    }

    impl From<ReqwestError> for ImageDownloadError {
        fn from(err: ReqwestError) -> Self {
            ImageDownloadError::ReqwestError(err)
        }
    }

    #[derive(Debug)]
    pub enum MemeMakeError {
        ImageDownloadError(ReqwestError),
        ImageLoadError(ImageError),
        MemeDataError(ReqwestError),
    }

    impl From<ImageDownloadError> for MemeMakeError {
        fn from(err: ImageDownloadError) -> Self {
            match err {
                ImageDownloadError::ReqwestError(err) => MemeMakeError::ImageDownloadError(err),
                ImageDownloadError::ImageError(err) => MemeMakeError::ImageLoadError(err),
            }
        }
    }

    impl From<ReqwestError> for MemeMakeError {
        fn from(err: ReqwestError) -> Self {
            MemeMakeError::MemeDataError(err)
        }
    }

    impl From<ImageError> for MemeMakeError {
        fn from(err: ImageError) -> Self {
            MemeMakeError::ImageLoadError(err)
        }
    }
}

pub struct MemeGen<'a> {
    client: Client,
    font: Font<'a>,
}

impl<'a> MemeGen<'_> {
    pub fn new(font: Font) -> MemeGen {
        MemeGen {
            client: Client::new(),
            font: font,
        }
    }

    pub async fn download_image(&self, url: &str) -> Result<DynamicImage, ImageDownloadError> {
        let data = self.client.get(url).send().await?.bytes().await?;
        let img = image::io::Reader::new(std::io::Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()?;
        return Ok(img);
    }

    pub async fn make_meme(
        &self,
        meme_name: &str,
        args: &MemeArgs,
    ) -> Result<DynamicImage, MemeMakeError> {
        let meme_data = self.fetch_meme_data(meme_name).await?;
        let mut primary_image = self.download_image(meme_data.image_url.as_str()).await?;

        for (arg, parameter) in std::iter::zip(&args.args, &meme_data.parameter) {
            let secondary_image = match arg {
                MemeArg::Text(text) => create_meme_text(&self.font, text.as_str()),
                MemeArg::ImageUrl(url) => self.download_image(url.as_str()).await?,
                MemeArg::ImagePath(path) => image::open(path)?,
            };
            for pos in &parameter.position {
                let bbox = BBox {
                    x: pos.box_left,
                    y: pos.box_top,
                    w: pos.box_right - pos.box_left,
                    h: pos.box_bottom - pos.box_top,
                };
                image_editing::draw_image_mut(&mut primary_image, &secondary_image, bbox);
            }
        }
        Ok(primary_image)
    }

    async fn fetch_meme_data(&self, name: &str) -> Result<Meme, ReqwestError> {
        let url = format!("https://spook.one/pixel/api/v1/meme?name={name}");
        let meme_data: Meme = self.client.get(url).send().await?.json().await?;
        Ok(meme_data)
    }
}

/*
    let property = system_fonts::FontPropertyBuilder::new()
        .family("Impact")
        .build();
    let (font, _) = system_fonts::get(&property).unwrap();
    let font = Font::try_from_vec(font).unwrap();
*/
