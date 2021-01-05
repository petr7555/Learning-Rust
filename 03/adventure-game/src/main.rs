use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

type Choice = (String, u32);

struct Scene {
    story: String,
    choices: Vec<Choice>,
}

impl Scene {
    fn from(story: String, choices: Vec<Choice>) -> Scene {
        Scene {
            story,
            choices,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = get_args();

    let scenes = parse_scenes(&args)?;

    play(&scenes)?;

    Ok(())
}

fn play(scenes: &HashMap<u32, Scene>) -> Result<(), Error> {
    const INITIAL_SCENE_INDEX: u32 = 0;

    let mut current_scene: &Scene = &scenes[&INITIAL_SCENE_INDEX];
    while !current_scene.choices.is_empty() {
        println!("{}", current_scene.story);
        for (choice, _) in &current_scene.choices {
            println!("{}", choice);
        }
        let choice = get_choice_from_user(current_scene.choices.len() as u32)?;
        let next_scene: u32 = current_scene.choices[choice as usize - 1].1;
        current_scene = &scenes[&next_scene];
    }
    println!("{}", current_scene.story);
    Ok(())
}

pub fn get_choice_from_user(num_of_choices: u32) -> Result<u32, Error> {
    loop {
        println!("What do you choose to do?");
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)?;
        match choice.trim().parse::<u32>() {
            Ok(val) => {
                if val < 1 || val > num_of_choices {
                    continue;
                }
                break Ok(val);
            }
            Err(_) => continue
        };
    }
}

fn parse_scenes(arguments: &[String]) -> Result<HashMap<u32, Scene>, Error> {
    let input_file = File::open(&arguments[1])?;
    let input_reader = BufReader::new(input_file);

    let mut scenes = HashMap::new();

    let mut lines: Vec<String> = Vec::new();
    for line in input_reader.lines() {
        let current_line = line?;

        if current_line.is_empty() {
            let id: u32 = lines[0].parse().unwrap();
            let story = String::from(&lines[1]);
            let choices = lines.iter().skip(2).map(|line| {
                let choice_and_next_scene: Vec<&str> = line.split(';').collect();
                (String::from(choice_and_next_scene[0]), choice_and_next_scene[1].trim().parse().unwrap())
            }).collect();
            let scene = Scene::from(
                story,
                choices);
            scenes.insert(id, scene);
            lines.clear();
        } else {
            lines.push(current_line);
        }
    }
    Ok(scenes)
}

fn get_args() -> Vec<String> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 2 {
        panic!("Usage: {} <story file>", arguments[0])
    };
    arguments
}