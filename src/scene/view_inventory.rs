use ::ggez::{
    *,
    graphics::*,
    input::keyboard::*
};
use ::std::mem::take;
use crate::{
    *,
    drawing::*,
    model::*,
    scene::{
        *,
        explore::*
    }
};

#[derive(Clone, Debug)]
pub struct ViewInventoryScene {
    pub i: usize,
    pub parent: Box<Scene>,
    pub pred: ItemPredicate
}

pub const INVENTORY_BORDER_RECT: Rect = Rect {
    x: VIEWPORT_LEFT - 1.0,
    y: VIEWPORT_TOP - 1.0,
    w: (PARTYLIST_RIGHT - VIEWPORT_LEFT) + 2.0,
    h: VIEWPORT_HEIGHT + 2.0
};
pub const ITEM_DETAILS_BORDER_RECT: Rect = Rect {
    x: VIEWPORT_LEFT - 1.0,
    y: PARTYLIST_TOP - 1.0,
    w: (PARTYLIST_RIGHT - VIEWPORT_LEFT) + 2.0,
    h: PARTYLIST_HEIGHT + 2.0
};
pub const ITEMS_PER_SCREEN: usize = 17;

fn expect_view_inventory(scene: &Scene) -> &ViewInventoryScene {
    match scene {
        Scene::ViewInventory(view_inv_scene) => view_inv_scene,
        _ => unimplemented!()
    }
}
fn expect_view_inventory_mut(scene: &mut Scene) -> &mut ViewInventoryScene {
    match scene {
        Scene::ViewInventory(view_inv_scene) => view_inv_scene,
        _ => unimplemented!()
    }
}

fn answer_request(
    game: &mut Game,
    item_i: Option<usize>
) {
    let (state, scene) = (&mut game.state, expect_view_inventory_mut(&mut game.scene));
    if let Some(item_i) = item_i {
        match scene.parent.as_mut() {
            Scene::ViewCharacter(ViewCharacterScene { i, selected }) => {
                if let Some(character) = state.party.get_mut(*i) {
                    let inv_item = state.inventory.get_mut(item_i).take().copied();
                    let equipped_item = character.item_mut(*selected).take();
                    *character.item_mut(*selected) = inv_item;
                    if let Some(item) = equipped_item {
                        state.inventory.push(item);
                    }
                }
            },
            _ => {}
        }
    }
    game.scene = take(&mut scene.parent);
}

pub fn key_down_event(
    _ctx: &mut Context,
    input: KeyInput,
    _repeated: bool,
    game: &mut Game
) -> GameResult {
    if matches!(
        input.keycode,
        Some(KeyCode::Escape | KeyCode::Return)
    ) {
        let (state, scene) = (&game.state, expect_view_inventory(&game.scene));
        match input.keycode {
            Some(KeyCode::Escape) => answer_request(game, None),
            Some(KeyCode::Return) => {
                if let Some(item) = state.inventory.get(scene.i) {
                    if scene.pred.matches(*item, &game) {
                        answer_request(game, Some(scene.i));
                    }
                }
            },
            _ => unimplemented!()
        }
    } else {
        let (state, scene) = (&game.state, expect_view_inventory_mut(&mut game.scene));
        let i = &mut scene.i;
        match input.keycode {
            Some(KeyCode::Up)
                => *i = (*i + state.inventory.len() - 1) % state.inventory.len(),
            Some(KeyCode::Down)
                => *i = (*i + 1) % state.inventory.len(),
            _   => {}
        }
    }
    Ok(())
}

pub fn update(_ctx: &mut Context, _game: &mut Game) -> GameResult {
    Ok(())
}

pub fn draw(ctx: &mut Context, game: &Game) -> GameResult {
    let (state, scene) = (&game.state, expect_view_inventory(&game.scene));
    // set up canvas
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_rect(
        ctx, &mut canvas,
        &rect_points(INVENTORY_BORDER_RECT),
        Color::WHITE,
        Color::BLACK
    )?;

    // draw list of items
    let len = state.inventory.len();
    let skip_len = scene.i.saturating_sub(ITEMS_PER_SCREEN / 2)
        .min(len.saturating_sub(ITEMS_PER_SCREEN));
    for (j, item) in state.inventory.iter().skip(skip_len).take(ITEMS_PER_SCREEN)
        .enumerate()
    {
        let y = 24.0 + j as f32 * 16.0;
        if scene.i == j + skip_len {
            draw_bitmap_text(
                &mut canvas,
                ">",
                &game.resources.font_bold,
                Color::WHITE,
                24.0, y
            );
        }
        draw_bitmap_text(
            &mut canvas,
            item.name(),
            &game.resources.font_regular,
            match scene.pred.matches(*item, game) {
                true => Color::WHITE,
                false => GRAY
            },
            56.0, y
        );
    }
    // draw item details
    draw_rect(
        ctx, &mut canvas,
        &rect_points(ITEM_DETAILS_BORDER_RECT),
        Color::WHITE,
        Color::BLACK
    )?;
    if let Some(item) = state.inventory.get(scene.i) {
        draw_bitmap_text(
            &mut canvas,
            item.name(),
            &game.resources.font_bold,
            Color::WHITE,
            24.0, 348.0
        );
        for (i, line) in item.desc().iter().enumerate() {
            draw_bitmap_text(
                &mut canvas,
                line,
                &game.resources.font_regular,
                Color::WHITE,
                24.0, 364.0 + i as f32 * 16.0
            );
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
