use ::ggez::{
    *,
    graphics::*,
    input::keyboard::*
};
use ::std::mem::take;
use crate::{
    *,
    drawing::*,
    model::{
        *,
        ItemSlot::*
    },
    scene::{
        *,
        explore::*
    }
};

#[derive(Clone, Debug)]
pub struct ViewCharacterScene {
    pub i: usize,
    pub selected: ItemSlot
}

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

fn expect_view_character(scene: &Scene) -> &ViewCharacterScene {
    match scene {
        Scene::ViewCharacter(view_char_scene) => view_char_scene,
        _ => unimplemented!()
    }
}
fn expect_view_character_mut(scene: &mut Scene) -> &mut ViewCharacterScene {
    match scene {
        Scene::ViewCharacter(view_char_scene) => view_char_scene,
        _ => unimplemented!()
    }
}

pub fn key_down_event(
    _ctx: &mut Context,
    input: KeyInput,
    _repeated: bool,
    game: &mut Game
) -> GameResult {
    let scene = expect_view_character_mut(&mut game.scene);
    match input.keycode {
        Some(KeyCode::Escape) => {
            game.scene = Scene::Explore(ExploreScene {
                anim: None,
                selected: Some(scene.i)
            });
        },
        Some(KeyCode::Return) => {
            game.scene = Scene::ViewInventory(ViewInventoryScene {
                i: 0,
                pred: ItemPredicate::Equippable(scene.i, scene.selected),
                parent: Box::new(take(&mut game.scene))
            });
        }
        Some(KeyCode::Up) => scene.selected = match scene.selected {
            Weapon => Armor,
            Shield => Weapon,
            Armor => Shield
        },
        Some(KeyCode::Down) => scene.selected = match scene.selected {
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
    let (state, scene) = (&game.state, expect_view_character(&game.scene));
    // set up canvas
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_rect(
        ctx, &mut canvas,
        &rect_points(CHAR_DETAILS_BORDER_RECT),
        Color::WHITE,
        Color::BLACK
    )?;

    // draw character stats
    if let Some(character) = state.party.get(scene.i) {
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
            if *slot == scene.selected {
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
    // draw control panel
    draw_controls(
        ctx, &mut canvas,
        &game.resources,
        &[("[\n]", "CHOOSE"), ("[\x1b]", "BACK")]
    )?;

    canvas.finish(ctx)
}
