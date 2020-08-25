extern crate num;
#[macro_use]
extern crate num_derive;

use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, BufWriter};

enum ResourceType {
    Gold(u32),
    Wood(u32),
}

enum Buildings {
    Farm,
}

enum GameAction {
    Build,
    Harvest(ResourceType),
    Pass,
}

#[derive(Debug)]
struct Resources {
    gold: u32,
    wood: u32,
}

struct GameState {
    current_turn: i32,
    end_turn: i32,
    resources: Resources,
    buildings: Vec<Buildings>,
}

impl GameState {
    fn new(end_turn: i32) -> GameState {
        GameState {
            current_turn: 0,
            end_turn,
            resources: Resources { gold: 0, wood: 0 },
            buildings: Vec::new(),
        }
    }

    fn game_loop(&mut self) {
        while self.current_turn < self.end_turn {
            println!("Turn {} started.", self.current_turn);

            let action: GameAction = get_user_action();

            self.do_action(action);

            // clear screen

            self.current_turn += 1;
        }
    }

    fn do_action(&mut self, action: GameAction) {
        match action {
            GameAction::Build => {
                if self.resources.gold >= 1000 && self.resources.wood >= 500 {
                    self.resources = Resources {
                        gold: self.resources.gold - 800,
                        wood: self.resources.wood - 400,
                    };
                    self.do_build_action(Buildings::Farm);
                }
            }
            GameAction::Harvest(resource) => self.do_harvest_action(resource),
            _ => println!("Action not implemented"),
        }
    }

    fn do_build_action(&mut self, building: Buildings) {
        self.buildings.push(building);
    }

    fn do_harvest_action(&mut self, resources: ResourceType) {
        match resources {
            ResourceType::Gold(gold) => self.resources.gold += gold,
            ResourceType::Wood(wood) => self.resources.wood += wood,
        }

        println!("Your resources: {:?}", self.resources);
    }
}

fn get_user_action() -> GameAction {
    loop {
        // println!("Choose your action:");
        // println!("1: Build");
        // println!("2: Harvest");

        // let mut action: String = String::new();
        // stdin().read_line(&mut action).unwrap();

        // let action_result = action.trim().parse::<i32>();
        // match action_result {
        //     Ok(act) => match act {
        //         1 => return GameAction::Build,
        //         2 => return GameAction::Harvest(get_harvest_action()),
        //         3 => return GameAction::Pass,
        //         _ => continue,
        //     },
        //     Err(_) => continue,
        // }
    }
}

// fn get_harvest_action() -> ResourceType {
//     loop {
//         println!("What do you want to harvest:");
//         println!("1: Gold");
//         println!("2: Wood");

//         let mut action: String = String::new();
//         stdin().read_line(&mut action).unwrap();

//         let action_result = action.trim().parse::<i32>();
//         match action_result {
//             Ok(act) => match act {
//                 1 => return ResourceType::Gold(500),
//                 2 => return ResourceType::Wood(500),
//                 _ => continue,
//             },
//             Err(_) => continue,
//         }
//     }
// }

struct Page {
    Number: i32,
    Final: bool,
    Text: String,
    Moves: Vec<Move>,
    Success: bool
}
struct Move {
    next_page: i32,
    text: String
}

impl Page {
    fn from(file: File) -> HashMap<i32, Page> {
        let mut pages = HashMap::new();
        let mut moves = Vec::new();
        let mut buf = HashMap::new();
        let reader = BufReader::new(file);

        for line in reader.lines().map(|l| l.unwrap()) {
            println!("Line -> {}", line);
            if line.contains(":") && line.contains(";"){
                let parts: Vec<&str> = line.split(';').collect();
                let parts0 : Vec<&str> = parts[0].split(":").collect();
                let parts1 : Vec<&str> = parts[1].split(":").collect();
                moves.push( Move {
                    next_page: parts0[1].parse().unwrap(),
                    text: String::from(parts1[1])
                } )
            }
            else if line.contains(":") {
                let parts: Vec<&str> = line.split(':').collect();
                let part0 = String::from(parts[0]);
                let part1 = String::from(parts[1]);
                println!("Line parts: 0 -> {0}, 1 -> {1}", part0, part1);
                buf.insert( part0, part1 );
            }

            if line == "---" {
                let final_attr = buf.get("Final").unwrap();
                let number_attr = buf.get("Page").unwrap().parse().unwrap();
                let text_attr = String::from( buf.get("Text").unwrap() );
                if final_attr == "Y" {
                    let success_attr: bool = buf.get("Success").unwrap().parse::<bool>().unwrap();

                    pages.insert( number_attr, Page {
                        Number: number_attr,
                        Final: true,
                        Text: text_attr,
                        Moves: Vec::new(),
                        Success: success_attr
                    });
                }
                else if final_attr == "N" {
                    pages.insert( number_attr, Page {
                        Number: number_attr,
                        Final: true,
                        Text: text_attr,
                        Moves: moves,
                        Success: false
                    });
                    moves = Vec::new();
                }
                else{
                    panic!("Invalid format of pages.");
                }
                buf.clear();
            }
        }

        pages
    }
}

fn main() {
    let pages = Page::from( File::open( String::from("pages.txt")).unwrap() );
    let mut game = GameState::new( pages.len() as i32);
    game.game_loop();
}
