use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::sync::Arc;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[derive(Serialize, Deserialize)]
pub struct Scenes {
    scenes: Vec<Scene>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Scene {
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
    /// Takes current scene (as `self`) and map of all scenes, returns next scene.
    fn execute(self, scenes: &HashMap<u32, Scene>) -> Scene;
}

impl Executable for Scene {
    fn execute(self, scenes: &HashMap<u32, Scene>) -> Scene {
        print_scene_story(&self);
        print_scene_choices(&self);
        match self.fight_scene_info {
            Some(mut info) => {
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
                scenes[&next_scene_id].clone()
            }
            None => {
                let choice = get_choice_from_user(self.choices.len() as u32);
                let next_scene_id: u32 = self.choices[choice as usize - 1].next_scene_id;
                return scenes[&next_scene_id].clone();
            }
        }
    }
}

pub fn scenes_into_map(scenes: Scenes) -> HashMap<u32, Scene> {
    let scenes_vec = scenes.scenes;
    let mut scenes_map = HashMap::new();
    for scene in scenes_vec {
        scenes_map.insert(scene.id, scene);
    }
    scenes_map
}

pub fn play(scenes: HashMap<u32, Scene>) {
    let address = "127.0.0.1:5000";
    let mut listener = TcpListener::bind(address).await?;

    println!("Server is listening on {}", address);

    let (tx, _) = broadcast::channel(16);
    let tx = Arc::new(tx);

    loop {
        let (stream, client_address) = listener.accept().await?;
        println!("New client: {:?}", client_address);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let (rd, mut wr) = io::split(stream);
        let mut rd = BufReader::new(rd);

        tokio::spawn(async move {
            let mut buf = String::new();

            loop {
                buf.clear();
                match rd.read_line(&mut buf).await {
                    Ok(0) => {
                        println!("Client has closed the connection.");
                        return;
                    }
                    Ok(_) => {
                        println!("Server received: {}", buf);
                        tx.send(buf.clone()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Server failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let message = rx.recv().await.unwrap();
                println!("Sending message to client: {}", message);
                wr.write_all(message.as_bytes()).await.unwrap();
                wr.flush().await.unwrap();
            }
        });
    }

    while !is_end_scene(&current_scene) {
        current_scene = current_scene.execute(&scenes);
    }
    print_scene_story(&current_scene);
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

pub fn parse_scenes(story_file: &str) -> Result<Scenes, Error> {
    let mut input_file = File::open(story_file)?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;
    let scenes: Scenes = serde_json::from_str(&*data)?;
    Ok(scenes)
}

#[cfg(test)]
mod tests {
    use crate::{Choice, is_end_scene, Scene};

    #[test]
    fn scene_without_choices_is_end_scene() {
        let end_scene = Scene {
            id: 0,
            fight_scene_info: None,
            story: "".to_string(),
            choices: vec![],
        };
        assert!(is_end_scene(&end_scene));
    }

    #[test]
    fn scene_with_choices_is_not_end_scene() {
        let not_end_scene = Scene {
            id: 0,
            fight_scene_info: None,
            story: "some story text".to_string(),
            choices: vec![Choice { text: "another story text".to_string(), next_scene_id: 1 }],
        };
        assert!(!is_end_scene(&not_end_scene));
    }
}
