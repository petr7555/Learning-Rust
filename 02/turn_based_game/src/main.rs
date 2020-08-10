use std::io;

enum GameAction {
    Build(BuildingType),
    Harvest,
    Mine(ResourceType),
}

enum ResourceType {
    Gold(u32),
    Wood(u32),
}

enum BuildingType {
    Farm,
}

const FARM_GOLD_PRICE: u32 = 1000;
const FARM_WOOD_PRICE: u32 = 500;

const FARM_PRODUCTION: u32 = 100;

struct Game {
    current_turn: u32,
    end_turn: u32,
    resources: Resources,
    buildings: Vec<BuildingType>,
}

#[derive(Debug)]
struct Resources {
    gold: u32,
    wood: u32,
    food: u32,
}

impl Game {
    fn new(end_turn: u32) -> Game {
        Game {
            current_turn: 0,
            end_turn,
            resources: Resources {
                gold: 0,
                wood: 0,
                food: 0,
            },
            buildings: Vec::new(),
        }
    }

    fn game_loop(&mut self) {
        while self.current_turn < self.end_turn {
            println!("\nTurn {} started.", self.current_turn + 1);
            self.do_start_of_turn_actions();
            self.print_resources();

            let action = Game::get_game_action();
            self.do_action(action);

            self.current_turn += 1;
        }
        println!("\nGame ended.");
    }

    fn do_start_of_turn_actions(&mut self) {
        for building in &self.buildings {
            match building {
                BuildingType::Farm => {
                    self.resources.food += FARM_PRODUCTION;
                }
            }
        }
    }

    fn do_action(&mut self, action: GameAction) {
        match action {
            GameAction::Build(building_type) => self.do_build_action(building_type),
            GameAction::Harvest => println!("Harvesting..."),
            GameAction::Mine(resource_type) => self.do_mine_action(resource_type),
        }
    }

    fn do_build_action(&mut self, building_type: BuildingType) {
        use BuildingType::*;
        match building_type {
            Farm => {
                if self.resources.gold >= FARM_GOLD_PRICE && self.resources.wood >= FARM_WOOD_PRICE
                {
                    self.resources.gold -= FARM_GOLD_PRICE;
                    self.resources.wood -= FARM_WOOD_PRICE;
                    println!("Building farm...");
                    self.buildings.push(building_type);
                } else {
                    println!("Not enough resources!");
                }
            }
        };
        self.print_resources();
    }

    fn do_mine_action(&mut self, resource_type: ResourceType) {
        use ResourceType::*;
        match resource_type {
            Gold(amount) => {
                println!("Mining {} gold...", amount);
                self.resources.gold += amount;
            }
            Wood(amount) => {
                println!("Mining {} wood...", amount);
                self.resources.wood += amount;
            }
        };
        self.print_resources();
    }

    fn print_resources(&mut self) {
        println!(
            "Your resources:\n\
                  {:?}",
            self.resources
        );
    }
    fn get_game_action() -> GameAction {
        loop {
            let mut action = String::new();
            println!(
                "Choose your action:\n\
                  1. Build\n\
                  2. Harvest\n\
                  3. Mine"
            );
            io::stdin().read_line(&mut action).unwrap();

            match action.trim().parse::<u32>() {
                Ok(entered_number) => match entered_number {
                    1 => return GameAction::Build(Game::get_building_type()),
                    2 => return GameAction::Harvest,
                    3 => return GameAction::Mine(Game::get_resource_type()),
                    _ => continue,
                },
                Err(_) => continue,
            };
        }
    }

    fn get_building_type() -> BuildingType {
        loop {
            let mut action = String::new();
            println!(
                "What do you want to build:\n\
                  1. Farm"
            );
            io::stdin().read_line(&mut action).unwrap();

            match action.trim().parse::<u32>() {
                Ok(entered_number) => match entered_number {
                    1 => return BuildingType::Farm,
                    _ => continue,
                },
                Err(_) => continue,
            };
        }
    }

    fn get_resource_type() -> ResourceType {
        loop {
            let mut action = String::new();
            println!(
                "What do you want to mine:\n\
                  1. Gold\n\
                  2. Wood"
            );
            io::stdin().read_line(&mut action).unwrap();

            use ResourceType::*;
            match action.trim().parse::<u32>() {
                Ok(entered_number) => match entered_number {
                    1 => return Gold(500),
                    2 => return Wood(500),
                    _ => continue,
                },
                Err(_) => continue,
            };
        }
    }
}

fn main() {
    let mut game = Game::new(10);
    game.game_loop();
}
