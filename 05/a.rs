use std::io;

struct Mapping {
    dst: u64,
    src: u64,
    len: u64,
}

fn main() {
    let mut input = String::new();

    _ = io::stdin().read_line(&mut input);
    let (_, seeds_str) = input.split_once(':').unwrap();
    let mut seeds: Vec<u64> = seeds_str.trim().split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    input.clear();
    _ = io::stdin().read_line(&mut input);
    for _ in 0..7 {
        input.clear();
        _ = io::stdin().read_line(&mut input);
        assert!(input.contains("map"));

        let mut mappings: Vec<Mapping> = Vec::new();
        loop {
            input.clear();
            let nbytes = io::stdin().read_line(&mut input).unwrap();
            if nbytes == 0 || input.trim().len() == 0 {
                break;
            }
            let mapping: Vec<u64> = input.trim().split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            mappings.push(Mapping {
                dst: mapping[0],
                src: mapping[1],
                len: mapping[2],
            });
        }

        let mut out: Vec<u64> = Vec::new();
        for seed in seeds {
            let mut found = false;
            for mapping in &mappings {
                if mapping.src <= seed && seed < mapping.src + mapping.len {
                    let offset = seed - mapping.src;
                    out.push(mapping.dst + offset);
                    found = true;
                }
            }
            if !found {
                out.push(seed);
            }
        }
        seeds = out;
    }

    println!("{}", seeds.iter().min().unwrap());
}