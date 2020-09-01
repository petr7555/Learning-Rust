use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::prelude::*;

fn main() {
    const NUM_OF_PHILOSOPHERS: usize = 5;

    let mut handles = vec![];
    let mut sticks = vec![];
    for _ in 0..NUM_OF_PHILOSOPHERS {
        sticks.push(Mutex::new(0));
    }
    let sticks = Arc::new(sticks);

    for philosopher_num in 0..NUM_OF_PHILOSOPHERS {
        let sticks = Arc::clone(&sticks);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                {
                    let mut left_stick = sticks[philosopher_num].lock().unwrap();
                    let mut right_stick = sticks[(philosopher_num + 1) % 5].lock().unwrap();

                    // eating
                    println!("Philosopher {} started eating", philosopher_num);
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 50)));
                    println!("Philosopher {} stopped eating", philosopher_num);

                    *left_stick += 1;
                    *right_stick += 1;
                }

                // thinking
                println!("Philosopher {} started thinking", philosopher_num);
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 50)));
                println!("Philosopher {} stopped thinking", philosopher_num);
            }
        }));
    }

    handles.into_iter().for_each(|handle| { handle.join().unwrap(); });
    sticks.iter().for_each(|stick| {
        println!("Stick was used {} times", stick.lock().unwrap());
    });
}