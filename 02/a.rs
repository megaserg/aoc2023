use std::io;
use std::collections::HashMap;

fn main() {
    let max_num: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        map.insert("red", 12);
        map.insert("green", 13);
        map.insert("blue", 14);
        map
    };

    let mut input = String::new();

    let mut sum = 0;
    loop {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // Reached EOF (Ctrl+D or Ctrl+Z)
            Ok(_) => {
                let (game_name, game) = input.split_once(':').unwrap();
                let (_, game_id_str) = game_name.split_once(' ').unwrap();
                let game_id: i32 = game_id_str.parse().unwrap();
                let sets = game.split(';');
                let mut good = true;
                for set in sets {
                    let balls = set.split(',');
                    for ball in balls {
                        let (num_str, color) = ball.trim().split_once(' ').unwrap();
                        let num: i32 = num_str.parse().unwrap();
                        if num > max_num[color] {
                            good = false;
                        }
                    }
                }
                if good {
                    sum += game_id;
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    println!("{}", sum);
}