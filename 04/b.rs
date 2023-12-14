use std::collections::{HashSet, VecDeque};
use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    let mut mults: VecDeque<i32> = VecDeque::new();
    mults.push_back(1);

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (_, game) = input.trim().split_once(':').unwrap();
        let (wins, mys) = game.split_once('|').unwrap();

        let w: HashSet<i32> = wins.split_whitespace()
            .map(|win| win.parse::<i32>().unwrap())
            .collect();

        let score = mys.split_whitespace()
            .map(|my| my.parse::<i32>().unwrap())
            .filter(|my| w.contains(my))
            .count();

        let mult = mults.pop_front().unwrap();
        sum += mult;

        if mults.is_empty() {
            mults.push_back(1);
        }
        for i in 0..score {
            if i == mults.len() {
                mults.push_back(1);
            }
            mults[i] += mult;
        }

        input.clear();
    }

    println!("{}", sum);
}