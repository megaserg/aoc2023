use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut input = String::new();

    _ = io::stdin().read_line(&mut input);
    let cmd = String::from(input.trim());
    input.clear();

    // empty line
    _ = io::stdin().read_line(&mut input);
    input.clear();

    let mut nodes: HashSet<String> = HashSet::new();
    let mut l: HashMap<String, String> = HashMap::new();
    let mut r: HashMap<String, String> = HashMap::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (node, edges_str) = input.trim().split_once('=').unwrap();
        let name = node.trim();
        nodes.insert(String::from(name));
        let (left, right) = edges_str.split_once(',').unwrap();
        l.insert(String::from(name), String::from(left.trim().strip_prefix('(').unwrap()));
        r.insert(String::from(name), String::from(right.trim().strip_suffix(')').unwrap()));
        input.clear();
    }

    eprintln!("{:?}", nodes);
    let mut xs = Vec::new();
    for node in nodes.iter().filter(|n| n.ends_with("A")) {
        let mut v = node;
        eprintln!("{}", node);

        let mut step: u64 = 0;

        let mut visited = HashMap::new();

        for c in cmd.chars().cycle() {
            if c == 'L' {
                v = l.get(v).unwrap();
            } else if c == 'R' {
                v = r.get(v).unwrap();
            }
            step += 1;
            if v.ends_with("Z") {
                let state = (v, step % cmd.len() as u64);
                if visited.contains_key(&state) {
                    eprintln!("offending: {:?}", state);
                    let prev = *visited.get(&state).unwrap();
                    eprintln!("prev: {}", prev);
                    eprintln!("curr: {}", step);
                    eprintln!("period: {}", step - prev);
                    assert_eq!(step, 2 * prev);
                    xs.push(prev);
                    break;
                }
                visited.insert(state, step);
            }
        }

        eprintln!("{:?}", visited);
    }

    let mut ans: u64 = 1;
    for x in xs {
        ans = lcm(ans, x);
    }

    println!("{}", ans);
}

fn lcm(x: u64, y: u64) -> u64 {
    return x * y / gcd(x, y);
}

fn gcd(x: u64, y: u64) -> u64 {
    if x > y { return gcd(y, x); }
    if x == 0 { return y; }
    return gcd(y % x, x);
}