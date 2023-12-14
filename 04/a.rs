use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (_, game) = input.trim().split_once(':').unwrap();
        let (wins, mys) = game.split_once('|').unwrap();

        let w: HashSet<i32> = wins.split_whitespace()
            .map(|win| { win.parse::<i32>().unwrap() })
            .collect();

        let pow = mys.split_whitespace()
            .map(|my| { my.parse::<i32>().unwrap() })
            .filter(|my| { w.contains(my) })
            .count();

        if pow > 0 {
            sum += 1 << (pow - 1);
        }

        input.clear();
    }

    println!("{}", sum);
}