use std::io;

fn hash(s: &str) -> usize {
    let mut v: usize = 0;
    for c in s.chars() {
        v = ((v + c as usize) * 17) % 256;
    }
    return v;
}

fn main() {
    let mut input = String::new();

    let mut map: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let cmds = input.trim().split(',');
        for cmd in cmds {
            let lst = cmd.chars().last().unwrap();
            if lst.is_digit(10) {
                let (label, foc_str) = cmd.split_once('=').unwrap();
                let foc = foc_str.parse().unwrap();
                let bx_id = hash(label);
                let mut found = false;
                for pair in &mut map[bx_id] {
                    if pair.0 == label {
                        pair.1 = foc;
                        found = true;
                        break;
                    }
                }
                if !found {
                    map[bx_id].push((label.to_string(), foc));
                }
            } else {
                assert_eq!(lst, '-');
                let (label, _) = cmd.split_once('-').unwrap();
                let bx_id = hash(label);
                map[bx_id].retain(|(l, _)| l != label);
            }
        }
        input.clear();
    }

    let mut sum = 0;
    for (bx_id, bx) in map.iter().enumerate() {
        for (slot_id, (_, f)) in bx.iter().enumerate() {
            sum += (bx_id + 1) * (slot_id + 1) * f;
        }
    }
    println!("{sum}");
}