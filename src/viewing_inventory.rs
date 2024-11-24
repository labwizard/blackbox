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

pub const INVENTORY_RECT_POINTS: &[Point2<f32>] = &[
    Point2 { x: VIEWPORT_LEFT - 1.0, y: VIEWPORT_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: VIEWPORT_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: VIEWPORT_BOTTOM + 1.0, },
    Point2 { x: VIEWPORT_LEFT - 1.0, y: VIEWPORT_BOTTOM + 1.0, }
];
pub const ITEM_DETAILS_RECT_POINTS: &[Point2<f32>] = &[
    Point2 { x: VIEWPORT_LEFT - 1.0, y: PARTYLIST_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: PARTYLIST_TOP - 1.0 },
    Point2 { x: PARTYLIST_RIGHT + 1.0, y: PARTYLIST_BOTTOM + 1.0, },
    Point2 { x: VIEWPORT_LEFT - 1.0, y: PARTYLIST_BOTTOM + 1.0, }
];
pub const ITEMS_PER_SCREEN: usize = 17;

pub fn no_condition(_item: Item) -> bool {
    true
}

fn answer_request(
    game: &mut Game,
    item_i: Option<usize>
) {
    let mut requester = match &mut game.state {
        ViewingInventory { requester, .. } => requester,
        _   => unimplemented!()
    };
    if let Some(item_i) = item_i {
        match requester.as_mut() {
            ViewingCharacter { i, selected } => {
                if let Some(character) = game.party.get_mut(*i) {
                    let inv_item = game.inventory.get_mut(item_i).take().copied();
                    let equipped_item = character.item_mut(*selected).take();
                    *character.item_mut(*selected) = inv_item;
                    if let Some(item) = equipped_item {
                        game.inventory.push(item);
                    }
                }
            },
            _ => {}
        }
    }
    game.state = ::std::mem::take(&mut requester);
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
        let (i, condition) = match &game.state {
            ViewingInventory { i, condition, .. }
                => (i, condition),
            _   => unimplemented!()
        };
        match input.keycode {
            Some(KeyCode::Escape) => {
                answer_request(game, None);
            },
            Some(KeyCode::Return) => {
                let (i, condition) = (*i, condition.as_ref());
                if let Some(item) = game.inventory.get(i) {
                    if condition(*item, game) {
                        answer_request(game, Some(i));
                    }
                }
            },
            _ => unimplemented!()
        }
    } else {
        let i = match &mut game.state {
            ViewingInventory { i, .. }
                => i,
            _   => unimplemented!()
        };
        match input.keycode {
            Some(KeyCode::Up)
                => *i = (*i + game.inventory.len() - 1) % game.inventory.len(),
            Some(KeyCode::Down)
                => *i = (*i + 1) % game.inventory.len(),
            _   => {}
        }
    }
    Ok(())
}

pub fn update(_ctx: &mut Context, _game: &mut Game) -> GameResult {
    Ok(())
}


pub fn draw(ctx: &mut Context, game: &Game) -> GameResult {
    let (i, condition) = match &game.state {
        ViewingInventory { i, condition, .. }
            => (*i, condition),
        _   => unimplemented!()
    };

    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    canvas.set_sampler(Sampler::nearest_clamp());

    draw_rect(
        ctx, &mut canvas,
        INVENTORY_RECT_POINTS,
        Color::WHITE,
        Color::BLACK
    )?;

    let len = game.inventory.len();
    let skip = i.saturating_sub(ITEMS_PER_SCREEN / 2)
        .min(len.saturating_sub(ITEMS_PER_SCREEN));
    for (j, item) in game.inventory.iter().skip(skip).take(ITEMS_PER_SCREEN)
        .enumerate()
    {
        let y = 24.0 + j as f32 * 16.0;
        if i == j + skip {
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
            match condition(*item, &*game) {
                true => Color::WHITE,
                false => GRAY
            },
            56.0, y
        );
    }

    draw_rect(
        ctx, &mut canvas,
        ITEM_DETAILS_RECT_POINTS,
        Color::WHITE,
        Color::BLACK
    )?;
    if let Some(item) = game.inventory.get(i) {
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

    draw_controls(
        ctx, &mut canvas,
        &game.resources,
        &[("[\n]", "CHOOSE"), ("[\x1b]", "BACK")]
    )?;

    canvas.finish(ctx)
}
