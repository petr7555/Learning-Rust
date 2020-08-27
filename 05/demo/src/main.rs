use std::sync::mpsc::channel;
use rayon::prelude::*;

fn main() {
    let (sender, receiver) = channel();

    (0..5).into_par_iter().for_each_with(sender, |s, x| s.send(x).unwrap());

    let mut res: Vec<_> = receiver.iter().collect();
    res.sort();

    assert_eq!(&res[..], &[0, 1, 2, 3, 4])
}