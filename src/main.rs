extern crate num;
#[macro_use]
extern crate num_derive;

use std::io;

enum ResourceType {
    Gold(u32),
    Wood(u32)
}

enum Buildings {
    Farm
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
    buildings: Vec::<Buildings>
}

impl GameState {
    fn new(end_turn: i32) -> GameState {
        GameState {
            current_turn: 0,
            end_turn,
            resources: Resources { gold: 0, wood: 0 },
            buildings: Vec::new()
        }
    }

    fn game_loop(&mut self) {
        while self.current_turn < self.end_turn {
            println!("Turn {} started.", self.current_turn);

            let action: GameAction = get_user_action();

            self.do_action(action);

            self.current_turn += 1;
        }
    }

    fn do_action(&mut self, action: GameAction) {
        match action {
            GameAction::Build => {
                if self.resources.gold >= 1000 && self.resources.wood >= 500 {
                    self.resources = Resources {
                        gold: self.resources.gold - 1000,
                        wood: self.resources.wood - 500,
                    };
                    self.do_build_action(Buildings::Farm);
                }
            },
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
            ResourceType::Wood(wood) => self.resources.wood += wood
        }

        println!("Your resources: {:?}", self.resources);
    }
}

fn get_user_action() -> GameAction {
    loop {
        println!("Choose your action:");
        println!("1: Build");
        println!("2: Harvest");

        let mut action: String = String::new();
        io::stdin().read_line(&mut action).unwrap();

        let action_result = action.trim().parse::<i32>();
        match action_result {
            Ok(act) => {
                match act {
                    1 => return GameAction::Build,
                    2 => return GameAction::Harvest(get_harvest_action()),
                    3 => return GameAction::Pass,
                    _ => continue
                }
            }
            Err(_) => continue,
        }
    }
}

fn get_harvest_action() -> ResourceType  {
    loop {
        println!("What do you want to harvest:");
        println!("1: Gold");
        println!("2: Wood");

        let mut action: String = String::new();
        io::stdin().read_line(&mut action).unwrap();

        let action_result = action.trim().parse::<i32>();
        match action_result {
            Ok(act) => {
                match act {
                    1 => return ResourceType::Gold(500),
                    2 => return ResourceType::Wood(500),
                    _ => continue
                }
            }
            Err(_) => continue,
        }
    }
}

fn main() {
    let mut game = GameState::new(10);
    game.game_loop();
}
