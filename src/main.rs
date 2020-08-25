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
    current_page_num: i32,
    pages: HashMap<i32, Page>
}

impl GameState {
    fn new( pages: HashMap<i32, Page>) -> GameState {
        GameState {
            current_page_num: 1,
            pages
        }
    }

    fn game_loop(&mut self) {
        let mut current_page;
        loop {
            current_page = self.pages.get(&self.current_page_num).unwrap();
            println!("You are on page {}:", current_page.Number);

            println!("Story:{}", current_page.Text);
            println!();

            if !current_page.Final {
                println!("Your choices are:");
                for (num, item) in current_page.Moves.iter().enumerate() {
                    println!("{0}: {1}", num + 1, item.text);
                }

                let item_num = self.get_user_action();
    
                self.do_action(item_num);
            }
            else{
                match current_page.Success {
                    true => println!("You win."),
                    false => println!("You lose.")
                };
                break;
            }
        }
    }

    fn do_action(&mut self, item_num: i32) {
        let next_page_num = self.pages.get(&self.current_page_num).unwrap().Moves.get(item_num as usize).unwrap().next_page;
        self.current_page_num = next_page_num;
    }

    fn get_user_action(&mut self) -> i32 {
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
    
            let action_num : i32 = line.trim().parse().unwrap();
    
            if action_num >= 1 || action_num <= self.pages.get(&self.current_page_num).unwrap().Moves.len() as i32 {
                return action_num - 1;
            }
            else{
                continue;
            }
        }
    }
}

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
            //println!("Line -> {}", line);
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
                //println!("Line parts: 0 -> {0}, 1 -> {1}", part0, part1);
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
                        Final: false,
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
    let mut game = GameState::new( pages );
    game.game_loop();
}
