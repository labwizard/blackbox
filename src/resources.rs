use ::ggez::{
    Context,
    GameResult
};
use ::std::include_bytes;
use crate::*;

const FONT_REGULAR: &[u8] = include_bytes!("../res/font/font_regular.png");
const FONT_BOLD: &[u8] = include_bytes!("../res/font/font_bold.png");

pub struct Resources {
    pub font_regular: BitmapFont,
    pub font_bold: BitmapFont
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Resources> {
        Ok(Resources {
            font_regular: BitmapFont::from_bytes(ctx, FONT_REGULAR, 16, 16)?,
            font_bold: BitmapFont::from_bytes(ctx, FONT_BOLD, 16, 16)?
        })
    }
}
