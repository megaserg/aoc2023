use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    let mut wfs = HashMap::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        if input.trim().is_empty() { break; }

        let (label, desc) = input.trim().split_once('{').unwrap();
        let mut wf: Vec<(Box<dyn Fn(&HashMap<String, i32>) -> bool>, String)> = Vec::new();
        let rules = desc.strip_suffix('}').unwrap().split(',');
        for rule in rules {
            if rule.contains(':') {
                let (crit, dest) = rule.split_once(':').unwrap();
                if crit.contains('<') {
                    let (name_str, bnd_str) = crit.split_once('<').unwrap();
                    let name = String::from(name_str);
                    let bnd: i32 = bnd_str.parse().unwrap();
                    wf.push((Box::new(move |x| x[&name] < bnd), String::from(dest)));
                } else if crit.contains('>') {
                    let (name_str, bnd_str) = crit.split_once('>').unwrap();
                    let name = String::from(name_str);
                    let bnd: i32 = bnd_str.parse().unwrap();
                    wf.push((Box::new(move |x| x[&name] > bnd), String::from(dest)));
                } else {
                    todo!();
                }
            } else {
                wf.push((Box::new(|_| true), String::from(rule)));
            }
        }
        wfs.insert(String::from(label), wf);

        input.clear();
    }

    let mut sum = 0;
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let tokens = input.trim().strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',');
        let mut d: HashMap<String, i32> = HashMap::new();
        let mut score = 0;
        for token in tokens {
            let (name, value_str) = token.split_once('=').unwrap();
            let value: i32 = value_str.parse().unwrap();
            score += value;
            d.insert(String::from(name), value);
        }

        let mut label: &String = &String::from("in");

        while label != "A" && label != "R" {
            let w = &wfs[label];
            for (crit, dest) in w {
                if crit(&d) {
                    label = dest;
                    break;
                }
            }
        }

        if label == "A" {
            sum += score;
        }

        input.clear();
    }
    println!("{sum}");
}