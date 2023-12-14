use std::io;
use std::collections::HashMap;

fn get_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("red".to_string(), 0);
    map.insert("green".to_string(), 0);
    map.insert("blue".to_string(), 0);
    map
}

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    loop {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // Reached EOF (Ctrl+D or Ctrl+Z)
            Ok(_) => {
                let (_, game) = input.split_once(':').unwrap();
                let sets = game.split(';');

                let mut m = get_map();

                for set in sets {
                    let balls = set.split(',');
                    for ball in balls {
                        let (num_str, color) = ball.trim().split_once(' ').unwrap();
                        let num: i32 = num_str.parse().unwrap();
                        m.entry(color.to_string()).and_modify(|mx| {
                            *mx = std::cmp::max(num, *mx);
                        });
                    }
                }

                sum += m["red"] * m["green"] * m["blue"];
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    println!("{}", sum);
}