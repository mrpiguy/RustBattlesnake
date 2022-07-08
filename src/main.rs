#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
mod logic;


#[get("/")]
fn handle_index() -> Json<Value> {
    Json(logic::get_info())
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>) -> Status {
    info!("{} START", start_req.game.id);
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    let mut gamestate = GameState{
        
        game:(&move_req.game).clone(),
        turn:(&move_req.turn).clone(),
        board:(&move_req.board).clone(),
        you:(&move_req.you).clone()
    };
    let chosen = logic::get_move(gamestate
    );

    Json(json!({ "move": chosen }))
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>) -> Status {
    info!("{} END", end_req.game.id);
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "BattlesnakeOfficial/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}

//Structures

//GameState struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

//Coordinate used for storing coordinates within other structs
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coord {
    x: u32,
    y: u32,
}
impl PartialEq for Coord{
    fn eq(&self, other:&Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

//Game object used to store all data about a game
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

//Board struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

//Snake Struct
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
}

impl Battlesnake{

    fn move_options(&self,width:u32,height:u32,snakes: &Vec<Battlesnake>)-> HashMap<&'static str, bool> {
        
        let mut possible_moves: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
        .into_iter()
        .collect();

        //Puts all coordinates with snake bodies (excluding tail) into a vector
        let mut snake_bodies = vec![];
        for snake in snakes{
            for body_part in snake.body[..snake.body.len()-1].iter(){
                snake_bodies.push(body_part)
            }
        }

        //Finds moves that don't result in instant death
        let head = &self.head;
        let neck = &self.body[1];
        if neck.y > head.y || head.y+1 == height || snake_bodies.contains(&&Coord{x:head.x,y:head.y+1}) {
            possible_moves.insert("up", false);
        }
        if neck.y < head.y || head.y == 0 || snake_bodies.contains(&&Coord{x:head.x,y:head.y-1}){
            possible_moves.insert("down", false);
        }
        if neck.x < head.x || head.x == 0 || snake_bodies.contains(&&Coord{x:head.x-1,y:head.y}){
            possible_moves.insert("left", false);
        }
        if neck.x > head.x || head.x+1 == width || snake_bodies.contains(&&Coord{x:head.x+1,y:head.y}){
            possible_moves.insert("right", false);
        }

        possible_moves
    }

    fn move_snake(&mut self, direction : String, food: &Vec<Coord>){
        
    
        //Update head position
        if direction == "up"{
            self.head.y = self.head.y+1;
        }
        else if direction == "down"{
            self.head.y = self.head.y-1;
        }
        else if direction == "left"{
            self.head.x = self.head.x-1;
        }
        else if direction == "right"{
            self.head.x = self.head.x+1;
        }
    
        
        self.body.insert(0,self.head.clone());
        self.body.pop();
            
    
        //Modify health, length and body accordingly depending if new spot has food
        if food.contains(&self.head){
            self.health = 100;
            self.length = self.length + 1;
            self.body.push(self.body[self.body.len()-1].clone());
        }
        else{
            self.health = self.health - 1;
            
        }
    }
}



