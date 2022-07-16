use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use log::info;

use crate::{Battlesnake, Board, Game,GameState,Node};

pub fn get_info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "mrpiguy",
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
    return ""
}

 
fn build_tree(gs: GameState,max_depth: u32, depth: u32, dir:String, snake:String) -> Node {
    //Checks if depth of tree is same as number of snakes
    if depth >= max_depth {
        let score = score_func(&gs,&gs.you.id);
        return Node {gs: gs,children:Vec::new(), depth:depth,score:score,snake:snake, dir:dir}
    }
    //If not, recursively add children to current node
    let moves = gs.board.snakes[depth as usize % gs.board.snakes.len()].move_options(gs.board.width, gs.board.height, &gs.board.snakes).into_iter().filter(|&(_, v)| v).map(|(k, _)| k).collect::<Vec<_>>();
    let mut c = Vec::new();
    //For each possible move in the current game state, create a new child for the current node
    for movee in moves { 
        let mut new_gs = gs.clone();
        let mut curr_snake = new_gs.board.snakes[depth as usize].clone();
        curr_snake.move_snake(movee.to_string(), &gs.board.food);
        c.push(build_tree(new_gs,max_depth,depth+1,movee.to_string(),curr_snake.id));
    }
    //If current snake is me, get maximum score of child, otherwise get minimum score of child
    let score = if c.len() == 0 {
        0
    } else if snake ==  gs.you.id {
        getOptimumScore(1,&c)
    } else {
        getOptimumScore(0,&c)
    };

    return Node {gs:gs, children:c,depth:depth,score:score, snake:snake, dir:dir}

}

fn getOptimumScore(opt: u32, children:&Vec<Node>) -> i32{
    //Looks at children nodes and picks either the min or max score to pass up the tree
    //if opt is 0 -> minimize
    //if opt is 1 -> maximize
    let mut score = if opt == 0 {
        1000
    } else {
        -1000
    };
    for child in children.into_iter(){
        if (opt == 0 && child.score < score) || (opt==1 && child.score > score) {
            score = child.score;
        }
    }

    return score
}

//This function will eventually evaluate gamestates and assign a score
fn score_func(gs: &GameState, snakeid:&String) -> i32{
    return 1
}