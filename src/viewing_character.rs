use ::ggez::{
    Context,
    GameResult,
    graphics::*,
    mint::Point2,
    input::keyboard::{
        KeyCode,
        KeyInput
    }
};
use crate::{
    *,
    GameState::*,
    exploring::*
};

pub const CHAR_DETAILS_RECT_POINTS: &[Point2<f32>] = &[
    Point2 { x: VIEWPORT_LEFT - 1.0, y: VIEWPORT_TOP - 1.0 },
    Point2 { x: VIEWPORT_RIGHT + 1.0, y: VIEWPORT_TOP - 1.0 },
    Point2 { x: VIEWPORT_RIGHT + 1.0, y: PARTYLIST_BOTTOM + 1.0, },
    Point2 { x: VIEWPORT_LEFT - 1.0, y: PARTYLIST_BOTTOM + 1.0, }
];

pub fn key_down_event(
    _ctx: &mut Context,
    input: KeyInput,
    _repeated: bool,
    game: &mut Game
) -> GameResult {
    let i = match &mut game.state {
        ViewingCharacter { i }
            => i,
        _   => unimplemented!()
    };
    match input.keycode {
        Some(KeyCode::Escape)
            => game.state = Exploring { anim: None, selected: Some(*i) },
        _   => {}
    }
    Ok(())
}

pub fn update(_ctx: &mut Context, _game: &mut Game) -> GameResult {
    Ok(())
}


pub fn draw(ctx: &mut Context, game: &mut Game) -> GameResult {
    let i = match &mut game.state {
        ViewingCharacter { i }
            => i,
        _   => unimplemented!()
    };

    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_rect(
        ctx, &mut canvas,
        CHAR_DETAILS_RECT_POINTS,
        Color::WHITE,
        Color::BLACK
    )?;

    if let Some(character) = game.party.get(*i) {
        draw_bitmap_text(
            &mut canvas,
            &character.name,
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 24.0
        );
        draw_bitmap_text(
            &mut canvas,
            format!("{} LV.{}", character.class.name(), character.lvl),
            &game.resources.font_regular,
            Color::WHITE,
            24.0, 40.0
        );
        draw_bitmap_text(
            &mut canvas,
            "HP",
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 72.0
        );
        draw_bitmap_text(
            &mut canvas,
            format!("{}/{}", character.hp, character.max_hp),
            &game.resources.font_regular,
            Color::WHITE,
            72.0, 72.0
        );
        draw_bitmap_text(
            &mut canvas,
            "MP",
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 88.0
        );
        draw_bitmap_text(
            &mut canvas,
            format!("{}/{}", character.mp, character.max_mp),
            &game.resources.font_regular,
            Color::WHITE,
            72.0, 88.0
        );
        draw_bitmap_text(
            &mut canvas,
            "ATK",
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 104.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.atk().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            88.0, 104.0
        );
        draw_bitmap_text(
            &mut canvas,
            "DEF",
            &game.resources.font_bold,
            Color::WHITE,
            152.0, 104.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.def().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            216.0, 104.0
        );
        draw_bitmap_text(
            &mut canvas,
            "MAG",
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 120.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.matk().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            88.0, 120.0
        );
        draw_bitmap_text(
            &mut canvas,
            "RES",
            &game.resources.font_bold,
            Color::WHITE,
            152.0, 120.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.mdef().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            216.0, 120.0
        );
        draw_bitmap_text(
            &mut canvas,
            "AGI",
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 136.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.agi().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            88.0, 136.0
        );
        draw_bitmap_text(
            &mut canvas,
            "LCK",
            &game.resources.font_bold,
            Color::WHITE,
            152.0, 136.0
        );
        draw_bitmap_text(
            &mut canvas,
            character.luck().to_string(),
            &game.resources.font_regular,
            Color::WHITE,
            216.0, 136.0
        );
    }
    draw_controls(
        ctx, &mut canvas,
        &game.resources,
        &[("[\x1b]", "BACK")]
    )?;

    canvas.finish(ctx)
}
