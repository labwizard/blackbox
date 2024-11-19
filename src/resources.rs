use ::ggez::{
    GameResult,
    context::Has,
    graphics::{
        GraphicsContext,
        Image
    }
};
use ::std::include_bytes;

const GAME_FRAME: &[u8] = include_bytes!("../resources/game_frame.png");

pub struct Resources {
    pub game_frame: Image
}

impl Resources {
    pub fn new(ctx: &impl Has<GraphicsContext>) -> GameResult<Resources> {
        Ok(Resources {
            game_frame: Image::from_bytes(ctx, GAME_FRAME)?
        })
    }
}
