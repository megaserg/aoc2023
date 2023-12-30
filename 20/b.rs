use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::io;
use std::process::exit;
use Signal::{HIGH, LOW};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Signal {
    LOW,
    HIGH,
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    if a > b { return gcd(b, a); }
    return gcd(b % a, a);
}

fn main() {
    let mut input = String::new();

    let mut flips: HashMap<String, Signal> = HashMap::new();
    let mut conjs: HashSet<String> = HashSet::new();
    let mut ins: HashMap<String, HashMap<String, Signal>> = HashMap::new();
    let mut out: HashMap<String, Vec<String>> = HashMap::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (mut from, tos_str) = input.trim().split_once(" -> ").unwrap();
        let tos: Vec<String> = tos_str.split(',').map(|x| String::from(x.trim())).collect();
        if from.starts_with('%') {
            from = from.strip_prefix("%").unwrap();
            flips.insert(String::from(from), LOW);
        }
        if from.starts_with('&') {
            from = from.strip_prefix("&").unwrap();
            conjs.insert(String::from(from));
        }
        for w in &tos {
            if !ins.contains_key(w) { ins.insert(w.clone(), HashMap::new()); }
            ins.get_mut(w).unwrap().insert(String::from(from), LOW);
        }
        out.insert(String::from(from), tos.clone());
        input.clear();
    }

    eprintln!("{:?}", ins);
    eprintln!("{:?}", out);

    // let mut path = Vec::new();
    // dfs(String::from("rt"), &out, &flips, &conjs, &mut path);

    let mut step = 0;
    let mut prev = 0;

    let mut stop = false;
    let mut periods: HashMap<String, i64> = HashMap::new();
    loop {
        step += 1;
        if step % 10000 == 0 {
            eprintln!("{step}");
        }
        let mut queue = VecDeque::new();
        queue.push_back((String::from("button"), String::from("broadcaster"), LOW));
        while !queue.is_empty() {
            let (from, to, value) = queue.pop_front().unwrap();

            if value == LOW && ins["hj"].contains_key(&to) {
                if !periods.contains_key(&to) {
                    eprintln!("LOW to {}! {} steps", to, step);
                    periods.insert(to.clone(), step as i64);
                    if periods.len() == ins["hj"].len() {
                        stop = true;
                    }
                }
            }
            if stop { break; }

            // eprintln!("{from} -{:?}-> {to}", value);
            if to == "broadcaster" {
                for w in out.get(&to).unwrap() {
                    queue.push_back((to.clone(), w.clone(), value));
                }
            } else if flips.contains_key(&to) {
                if value == LOW {
                    if *flips.get(&to).unwrap() == LOW {
                        flips.insert(to.clone(), HIGH);
                        for w in out.get(&to).unwrap() {
                            queue.push_back((to.clone(), w.clone(), HIGH));
                        }
                    } else {
                        flips.insert(to.clone(), LOW);
                        for w in out.get(&to).unwrap() {
                            queue.push_back((to.clone(), w.clone(), LOW));
                        }
                    }
                }
            } else if conjs.contains(&to) {
                ins.get_mut(&to).unwrap().insert(from, value);
                let mut all_high = true;
                for (_, state) in ins.get(&to).unwrap() {
                    if *state != HIGH {
                        all_high = false;
                        break;
                    }
                }
                if all_high {
                    for w in out.get(&to).unwrap() {
                        queue.push_back((to.clone(), w.clone(), LOW));
                    }
                } else {
                    for w in out.get(&to).unwrap() {
                        queue.push_back((to.clone(), w.clone(), HIGH));
                    }
                }
            }
        }
        if stop { break; }
    }

    let mut lcm: i64 = 1;
    for period in periods.values() {
        lcm = lcm * period / gcd(lcm, *period);
    }
    println!("{lcm}");
}

fn dfs(v: String, out: &HashMap<String, Vec<String>>, flips: &HashMap<String, Signal>, conjs: &HashSet<String>, path: &mut Vec<String>) {
    // if conjs.contains(&v) {
    if path.contains(&v) {
        eprintln!("path {:?} {}", path.iter().map(|x| if flips.contains_key(x) { String::from("%") + x } else if conjs.contains(x) { String::from("&") + x } else { x.clone() }).collect::<Vec<String>>(), v);
        return;
    }
    path.push(v.clone());
    if !out.contains_key(&v) {
        eprintln!("term {:?}", path.iter().map(|x| if flips.contains_key(x) { String::from("%") + x } else if conjs.contains(x) { String::from("&") + x } else { x.clone() }).collect::<Vec<String>>());
        path.pop();
        return;
    }
    for w in &out[&v] {
        dfs(w.clone(), out, flips, conjs, path);
    }
    path.pop();
}