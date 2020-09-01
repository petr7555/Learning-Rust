use std::sync::{Arc, mpsc, Mutex};
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
    let (sender, receiver) = mpsc::channel();

    for philosopher_num in 0..NUM_OF_PHILOSOPHERS {
        let sticks = Arc::clone(&sticks);
        let sender = sender.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                {
                    let mut left_stick = sticks[philosopher_num].lock().unwrap();
                    let mut right_stick = sticks[(philosopher_num + 1) % 5].lock().unwrap();

                    // eating
                    sender.send(format!("Philosopher {} started eating", philosopher_num)).unwrap();
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 50)));
                    sender.send(format!("Philosopher {} stopped eating", philosopher_num)).unwrap();

                    *left_stick += 1;
                    *right_stick += 1;
                }

                // thinking
                sender.send(format!("Philosopher {} started thinking", philosopher_num)).unwrap();
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(20, 50)));
                sender.send(format!("Philosopher {} stopped thinking", philosopher_num)).unwrap();
            }
        }));
    }
    // closes sender, so receiver does not expect more messages
    drop(sender);

    handles.into_iter().for_each(|handle| { handle.join().unwrap(); });
    receiver.iter().for_each(|msg| {
        println!("{}", msg);
    });

    sticks.iter().for_each(|stick| {
        println!("Stick was used {} times", stick.lock().unwrap());
    });
}