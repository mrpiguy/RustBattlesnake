use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use log::info;

use crate::{Battlesnake, Board, Game,GameState};

pub fn get_info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "cbierer",
        "color": "#0b6623",
        "head": "default",
        "tail": "bolt",
    });
}

pub fn get_move(gs: GameState) -> &'static str {
    //minimax(gs);
    get_random_safe_move(gs)
}

//a function to make the snake work and not kill itself
fn get_random_safe_move(gs:GameState) -> &'static str{
    let possible_moves = gs.you.move_options(gs.board.width,gs.board.height,&gs.board.snakes);
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();
    info!("{} MOVE {}", gs.game.id, chosen);
    chosen
}

fn minimax(gs:GameState) -> &'static str{

}