use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Scenes {
    scenes: Vec<Scene>
}

#[derive(Serialize, Deserialize, Clone)]
struct Scene {
    id: u32,
    fight_scene_info: Option<FightSceneInfo>,
    story: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Clone)]
struct FightSceneInfo {
    enemy: Character,
    player: Character,
    on_death: u32,
    on_win: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Character {
    health: i32,
    attack_min: i32,
    attack_max: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Choice {
    text: String,
    next_scene_id: u32,
}

trait Executable {
    /// Takes current scene as `self` and map of all scenes, returns next scene.
    fn execute(self, scenes: &HashMap<u32, Scene>) -> Scene;
}

impl Executable for Scene {
    fn execute(self, scenes: &HashMap<u32, Scene>) -> Scene {
        print_scene_story(&self);
        print_scene_choices(&self);
        if self.fight_scene_info.is_none() {
            let choice = get_choice_from_user(self.choices.len() as u32);
            let next_scene_id: u32 = self.choices[choice as usize - 1].next_scene_id;
            return scenes[&next_scene_id].clone();
        }
        let mut info = self.fight_scene_info.unwrap();
        if info.player.health <= 0 {
            return scenes[&info.on_death].clone();
        }
        if info.enemy.health <= 0 {
            return scenes[&info.on_win].clone();
        }
        println!("Your health: {}; attack damage range: {}-{}", info.player.health, info.player.attack_min, info.player.attack_max);
        println!("Enemy health: {}; attack damage range: {}-{}", info.enemy.health, info.enemy.attack_min, info.enemy.attack_max);
        let choice = get_choice_from_user(self.choices.len() as u32);
        // choice 1 is always attack
        if choice == 1 {
            let mut rng = rand::thread_rng();
            let player_dmg = rng.gen_range(info.player.attack_min, info.player.attack_max);
            let enemy_dmg = rng.gen_range(info.enemy.attack_min, info.enemy.attack_max);
            println!("Player attacks and deals {} damage.", player_dmg);
            println!("Enemy attacks at the same time and deals {} damage.", enemy_dmg);
            info.enemy.health -= player_dmg;
            info.player.health -= enemy_dmg;
            return Scene {
                id: self.id,
                fight_scene_info: Option::from(info),
                story: self.story,
                choices: self.choices,
            };
        };
        let next_scene_id: u32 = self.choices[choice as usize - 1].next_scene_id;
        return scenes[&next_scene_id].clone();
    }
}

fn main() -> std::io::Result<()> {
    let args = get_args();

    let scenes = parse_scenes(&args[1])?;
    let scenes_map = scenes_into_map(scenes);

    play(scenes_map)?;

    Ok(())
}

fn scenes_into_map(scenes: Scenes) -> HashMap<u32, Scene> {
    let scenes_vec = scenes.scenes;
    let mut scenes_map = HashMap::new();
    for scene in scenes_vec {
        scenes_map.insert(scene.id, scene);
    }
    scenes_map
}

fn play(scenes: HashMap<u32, Scene>) -> Result<(), Error> {
    const INITIAL_SCENE_INDEX: u32 = 0;

    let mut current_scene: Scene = scenes.get(&INITIAL_SCENE_INDEX)
        .expect(format!("Story file must contain scene with id {}", INITIAL_SCENE_INDEX).as_str()).clone();
    while !is_end_scene(&current_scene) {
        current_scene = current_scene.execute(&scenes);
    }
    print_scene_story(&current_scene);
    Ok(())
}

fn is_end_scene(current_scene: &Scene) -> bool {
    current_scene.choices.is_empty()
}

fn print_scene_story(current_scene: &Scene) {
    println!("\n{}", current_scene.story);
}

fn print_scene_choices(current_scene: &Scene) {
    for choice in &current_scene.choices {
        println!("{}", choice.text);
    }
}

fn get_choice_from_user(num_of_choices: u32) -> u32 {
    loop {
        println!("What do you choose to do?");
        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Cannot read user input.");
        match choice.trim().parse::<u32>() {
            Ok(val) => {
                if val < 1 || val > num_of_choices {
                    continue;
                }
                break val;
            }
            Err(_) => continue
        };
    }
}

fn parse_scenes(story_file: &str) -> Result<Scenes, Error> {
    let mut input_file = File::open(story_file)?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;
    let scenes: Scenes = serde_json::from_str(&*data)?;
    Ok(scenes)
}

fn get_args() -> Vec<String> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 2 {
        panic!("Usage: {} <story file>", arguments[0])
    };
    arguments
}