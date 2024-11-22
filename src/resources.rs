use ::ggez::{
    Context,
    GameResult,
    graphics::Image
};
use ::std::include_bytes;
use crate::*;

// images
const VIEWPORT_MASK: &[u8] = include_bytes!("../res/img/viewport_mask.png");

const FONT_REGULAR: &[u8] = include_bytes!("../res/font/font_regular.png");
const FONT_BOLD: &[u8] = include_bytes!("../res/font/font_bold.png");

pub struct Resources {
    pub font_regular: BitmapFont,
    pub font_bold: BitmapFont,
    pub viewport_mask: Image
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        Ok(Resources {
            font_regular: BitmapFont::from_bytes(ctx, FONT_REGULAR, 16, 16)?,
            font_bold: BitmapFont::from_bytes(ctx, FONT_BOLD, 16, 16)?,
            viewport_mask: Image::from_bytes(ctx, VIEWPORT_MASK)?
        })
    }
}
