use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    let mut wfs = HashMap::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        if input.trim().is_empty() { break; }

        let (label, desc) = input.trim().split_once('{').unwrap();
        let mut wf: Vec<((char, String, u64), String)> = Vec::new();
        let rules = desc.strip_suffix('}').unwrap().split(',');
        for rule in rules {
            if rule.contains(':') {
                let (crit, dest) = rule.split_once(':').unwrap();
                if crit.contains('<') {
                    let (name_str, bnd_str) = crit.split_once('<').unwrap();
                    let name = String::from(name_str);
                    let bnd: u64 = bnd_str.parse().unwrap();
                    wf.push((('<', name, bnd), String::from(dest)));
                } else if crit.contains('>') {
                    let (name_str, bnd_str) = crit.split_once('>').unwrap();
                    let name = String::from(name_str);
                    let bnd: u64 = bnd_str.parse().unwrap();
                    wf.push((('>', name, bnd), String::from(dest)));
                } else {
                    todo!();
                }
            } else {
                wf.push((('=', String::from(""), 0), String::from(rule)));
            }
        }
        wfs.insert(String::from(label), wf);

        input.clear();
    }

    let mut state = HashMap::new();
    state.insert(String::from("x"), (1, 4000));
    state.insert(String::from("m"), (1, 4000));
    state.insert(String::from("a"), (1, 4000));
    state.insert(String::from("s"), (1, 4000));

    let mut sum = 0;
    explore(&String::from("in"), state, &wfs, &mut sum);
    println!("{sum}");
}

fn explore(
    label: &String,
    mut state: HashMap<String, (u64, u64)>,
    wfs: &HashMap<String, Vec<((char, String, u64), String)>>,
    sum: &mut u64,
) {
    if label == "A" {
        let mut prod: u64 = 1;
        for (_, (min, max)) in state {
            prod *= max - min + 1;
        }
        *sum += prod;
        return;
    }
    if label == "R" {
        return;
    }

    for (crit, dest) in &wfs[label] {
        let (sign, name, value) = crit;
        match sign {
            '<' => {
                let mut tmp = state.clone();
                tmp.get_mut(name).unwrap().1 = *value - 1;
                explore(dest, tmp, wfs, sum);
                state.get_mut(name).unwrap().0 = *value;
            }
            '>' => {
                let mut tmp = state.clone();
                tmp.get_mut(name).unwrap().0 = *value + 1;
                explore(dest, tmp, wfs, sum);
                state.get_mut(name).unwrap().1 = *value;
            }
            _ => {
                let tmp = state.clone();
                explore(dest, tmp, wfs, sum);
            }
        }
    }
}