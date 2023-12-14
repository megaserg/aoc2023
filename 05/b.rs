use std::cmp::min;
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
    let seeds: Vec<u64> = seeds_str.trim().split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    let mut pairs: Vec<(u64, u64)> = seeds.chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
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

        mappings.sort_by_key(|m| m.src);

        let mut out: Vec<(u64, u64)> = Vec::new();

        for (seed_start, seed_len) in &pairs {
            let mut c = *seed_start;
            let mut d = *seed_start + *seed_len - 1;
            for mapping in &mappings {
                if c > d { break; }
                let a = mapping.src;
                let b = mapping.src + mapping.len - 1;
                let x = mapping.dst;
                if a <= c && c <= b {
                    let offset = c - a;
                    let length = min(b, d) - c + 1;
                    let new_c = x + offset;
                    out.push((new_c, length));
                    eprintln!("slice [{c}..{d}] intersects-1 mapping ({a}..{b}->{x}) - mapped {c}.. to {new_c}.. of len {length}");
                    c += length;
                    eprintln!("slice is now [{c}..{d}]");
                } else if c <= a && a <= d && d <= b {
                    let new_a = mapping.dst;
                    let length = d - a + 1;
                    out.push((new_a, length));
                    eprintln!("slice [{c}..{d}] intersects-2 mapping ({a}..{b}->{x}) - mapped {a}.. to {new_a}.. of len {length}");
                    d = a - 1;
                    eprintln!("slice is now [{c}..{d}]");
                } else if c <= a && b <= d {
                    let keep_len = a - c;
                    out.push((c, keep_len));
                    let new_a = mapping.dst;
                    let length = b - a + 1;
                    out.push((new_a, length));
                    eprintln!("slice [{c}..{d}] intersects-3 mapping ({a}..{b}->{x}) - kept {c}.. of len {keep_len} mapped {a}.. to {new_a}.. of len {length}");
                    c = b + 1;
                    eprintln!("slice is now [{c}..{d}]");
                }
            }
            if c <= d {
                out.push((c, d - c + 1));
                eprintln!("pushing leftover slice [{c}..{d}]");
            }
        }

        eprintln!();
        eprintln!("pre: {:?}", pairs);
        eprintln!("now: {:?}", out);
        eprintln!();

        pairs = out;
    }

    println!("{}", pairs.iter().map(|(x, _)| x).min().unwrap());
}