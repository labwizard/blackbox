use ::ggez::{
    Context,
    GameResult,
    graphics::*,
    input::keyboard::{
        KeyCode,
        KeyInput
    }
};
use crate::{
    *,
    GameState::*,
    ItemSlot::*,
    exploring::*
};

pub const CHAR_DETAILS_BORDER_RECT: Rect = Rect {
    x: VIEWPORT_LEFT - 1.0,
    y: VIEWPORT_TOP - 1.0,
    w: VIEWPORT_WIDTH + 2.0,
    h: (PARTYLIST_BOTTOM - VIEWPORT_TOP) + 2.0
};
pub const SLOTS: &[ItemSlot] = &[
    Weapon,
    Shield,
    Armor
];
pub const SLOT_LABELS: &[&'static str] = &[
    "WEAPON",
    "SHIELD",
    "ARMOR"
];

pub fn key_down_event(
    _ctx: &mut Context,
    input: KeyInput,
    _repeated: bool,
    game: &mut Game
) -> GameResult {
    let (i, selected) = match &mut game.state {
        ViewingCharacter { i, selected }
            => (i, selected),
        _   => unimplemented!()
    };
    match input.keycode {
        Some(KeyCode::Escape)
            => game.state = Exploring { anim: None, selected: Some(*i) },
        Some(KeyCode::Return) => {
            let (i, selected) = (*i, *selected);
            game.state = ViewingInventory {
                i: 0,
                requester: Box::new(::std::mem::take(&mut game.state)),
                condition: Box::new(move |item, game| {
                    game.party.get(i).map_or(
                        false,
                        |character| item.equippable_to(character, selected)
                    )
                })
            };
        }
        Some(KeyCode::Up) => *selected = match selected {
            Weapon => Armor,
            Shield => Weapon,
            Armor => Shield
        },
        Some(KeyCode::Down) => *selected = match selected {
            Weapon => Shield,
            Shield => Armor,
            Armor => Weapon
        },
        _   => {}
    }
    Ok(())
}

pub fn update(_ctx: &mut Context, _game: &mut Game) -> GameResult {
    Ok(())
}


pub fn draw(ctx: &mut Context, game: &Game) -> GameResult {
    let (i, selected) = match &game.state {
        ViewingCharacter { i, selected }
            => (i, selected),
        _   => unimplemented!()
    };

    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_rect(
        ctx, &mut canvas,
        &rect_points(CHAR_DETAILS_BORDER_RECT),
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
        for (i, (stat, val, max)) in [
            ("HP", character.hp, character.max_hp),
            ("MP", character.mp, character.max_mp),
        ].iter().enumerate() {
            let y = 72.0 + i as f32 * 16.0;
            draw_bitmap_text(
                &mut canvas,
                stat,
                &game.resources.font_bold,
                Color::WHITE,
                24.0, y
            );
            draw_bitmap_text(
                &mut canvas,
                format!("{}/{}", val, max),
                &game.resources.font_regular,
                Color::WHITE,
                72.0, y
            );
        }
        for (i, (stat, val, base)) in [
            ("ATK", character.atk(), character.base_atk),
            ("DEF", character.def(), character.base_def),
            ("MAG", character.matk(), character.base_matk),
            ("RES", character.mdef(), character.base_mdef),
            ("AGI", character.agi(), character.base_agi),
            ("LUC", character.luck(), character.base_luck),
        ].iter().enumerate() {
            let y = 104.0 + i as f32 * 16.0;
            draw_bitmap_text(
                &mut canvas,
                stat,
                &game.resources.font_bold,
                Color::WHITE,
                24.0, y
            );
            draw_bitmap_text(
                &mut canvas,
                val.to_string(),
                &game.resources.font_regular,
                Color::WHITE,
                88.0, y
            );
            if *val != *base {
                draw_bitmap_text(
                    &mut canvas,
                    format!("(BASE {})", base),
                    &game.resources.font_regular,
                    Color::WHITE,
                    152.0, y
                );
            }
        }
        for (i, (slot, label)) in SLOTS.iter().zip(SLOT_LABELS).enumerate() {
            let y = 216.0 + i as f32 * 16.0;
            draw_bitmap_text(
                &mut canvas,
                label,
                &game.resources.font_bold,
                Color::WHITE,
                24.0, y
            );
            draw_bitmap_text(
                &mut canvas,
                match character.item(*slot) {
                    Some(item) => item.name(),
                    None => "````"
                },
                &game.resources.font_regular,
                Color::WHITE,
                136.0, y
            );
            if slot == selected {
                draw_bitmap_text(
                    &mut canvas,
                    "<",
                    &game.resources.font_bold,
                    Color::WHITE,
                    392.0, y
                );
            }
        }
    }
    draw_controls(
        ctx, &mut canvas,
        &game.resources,
        &[("[\n]", "CHOOSE"), ("[\x1b]", "BACK")]
    )?;

    canvas.finish(ctx)
}
