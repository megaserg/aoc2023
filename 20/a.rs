use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use Signal::{HIGH, LOW};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Signal {
    LOW,
    HIGH,
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

    let mut lows = 0;
    let mut highs = 0;

    for step in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((String::from("button"), String::from("broadcaster"), LOW));
        while !queue.is_empty() {
            let (from, to, value) = queue.pop_front().unwrap();

            eprintln!("{from} -{:?}-> {to}", value);
            if value == LOW {
                lows += 1;
            } else {
                highs += 1;
            }
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
    }

    eprintln!("{lows} {highs}");
    println!("{}", lows * highs);
}