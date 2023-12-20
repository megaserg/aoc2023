use std::collections::{BTreeSet, HashMap, HashSet};
use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<u32>> = Vec::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect());
        input.clear();
    }

    let n = a.len();
    let m = a[0].len();

    eprintln!("{n} {m}");

    let di = [1, 0, -1, 0];
    let dj = [0, 1, 0, -1];

    let mut d: HashMap<(i32, i32, i32), u32> = HashMap::new();
    d.insert((0, 0, 0), 0);
    d.insert((0, 0, 1), 0);

    let mut q: BTreeSet<(u32, (i32, i32, i32))> = BTreeSet::new();
    q.insert((0, (0, 0, 0)));
    q.insert((0, (0, 0, 1)));

    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();

    while !q.is_empty() {
        let node = *q.first().unwrap();

        let (zd, v) = node;
        let (i, j, s) = v;
        let vd = zd;

        q.remove(&node);

        seen.insert((i, j, s));


        for ds in 0..4 {
            if ds == s || (ds + 2) % 4 == s { continue; }
            let mut acc = 0;

            for k in 1..=3 {
                let ni = i + di[ds as usize] * k;
                let nj = j + dj[ds as usize] * k;
                if (ni as usize) < n && (nj as usize) < m {
                    acc += a[ni as usize][nj as usize];
                }
            }

            for k in 4..=10 {
                let ni = i + di[ds as usize] * k;
                let nj = j + dj[ds as usize] * k;
                if (ni as usize) < n && (nj as usize) < m {
                    acc += a[ni as usize][nj as usize];
                    if !d.contains_key(&(ni, nj, ds)) {
                        d.insert((ni, nj, ds), vd + acc);
                        q.insert((vd + acc, (ni, nj, ds)));
                    } else if d[&(ni, nj, ds)] > vd + acc {
                        q.remove(&(d[&(ni, nj, ds)], (ni, nj, ds)));
                        d.insert((ni, nj, ds), vd + acc);
                        q.insert((vd + acc, (ni, nj, ds)));
                    }
                }
            }
        }
    }

    println!("{}", std::cmp::min(d[&((n - 1) as i32, (m - 1) as i32, 0)], d[&((n - 1) as i32, (m - 1) as i32, 1)]));
}