use adventure_game::{parse_scenes, scenes_into_map, play};
use std::env;

fn main() {
    let args = get_args();

    let scenes = parse_scenes(&args[1]).expect("Unable to parse scenes");
    let scenes_map = scenes_into_map(scenes);

    play(scenes_map);
}

fn get_args() -> Vec<String> {
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() != 2 {
        panic!("Usage: {} <story file>", arguments[0])
    };
    arguments
}