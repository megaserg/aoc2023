use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    _ = io::stdin().read_line(&mut input);
    let cmd = String::from(input.trim());
    input.clear();

    // empty line
    _ = io::stdin().read_line(&mut input);
    input.clear();

    let mut l: HashMap<String, String> = HashMap::new();
    let mut r: HashMap<String, String> = HashMap::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (node, edges_str) = input.trim().split_once('=').unwrap();
        let name = node.trim();
        let (left, right) = edges_str.split_once(',').unwrap();
        l.insert(String::from(name), String::from(left.trim().strip_prefix('(').unwrap()));
        r.insert(String::from(name), String::from(right.trim().strip_suffix(')').unwrap()));
        input.clear();
    }

    let mut node = "AAA";
    let mut step = 0;

    for c in cmd.chars().cycle() {
        if c == 'L' {
            node = l.get(node).unwrap();
        } else if c == 'R' {
            node = r.get(node).unwrap();
        }
        step += 1;
        if node == "ZZZ" {
            break;
        }
    }

    println!("{}", step);
}
