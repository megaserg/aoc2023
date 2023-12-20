use std::collections::HashMap;
use std::io;

const BILLION: u32 = 1_000_000_000;

fn hash(a: &Vec<Vec<u32>>) -> u32 {
    let n = a.len();
    let m = a[0].len();
    let mut x: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            x = (x * 7 + a[i][j]) % 100000007;
        }
    }
    return x;
}

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<u32>> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().map(|c| match c {
            'O' => 2,
            '#' => 1,
            '.' => 0,
            _ => todo!()
        }).collect());
        input.clear();
    }

    let n = a.len();
    let m = a[0].len();

    let mut pos = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 2 {
                pos.push((i as i32, j as i32));
            }
        }
    }

    let mut seen: HashMap<u32, u32> = HashMap::new();

    let ii: [i32; 4] = [-1, 0, 1, 0];
    let jj: [i32; 4] = [0, -1, 0, 1];

    let mut cycle: u32 = 0;
    loop {
        for step in 0..4 {
            match step % 4 {
                0 => pos.sort_by_key(|x| x.0),
                1 => pos.sort_by_key(|x| x.1),
                2 => pos.sort_by_key(|x| -x.0),
                3 => pos.sort_by_key(|x| -x.1),
                _ => todo!()
            }

            let mut out = Vec::new();
            for rock in pos {
                let (i, j) = rock;
                let mut si = i as usize;
                let mut sj = j as usize;
                let di = ii[(step % 4) as usize];
                let dj = jj[(step % 4) as usize];
                let mut ni = (si as i32 + di) as usize;
                let mut nj = (sj as i32 + dj) as usize;
                while ni < n && nj < m && a[ni][nj] == 0 {
                    a[ni][nj] = 2;
                    a[si][sj] = 0;
                    si = ni;
                    sj = nj;
                    ni = (si as i32 + di) as usize;
                    nj = (sj as i32 + dj) as usize;
                }
                out.push((si as i32, sj as i32));
            }
            pos = out;
        }

        cycle += 1;

        // eprintln!("after {cycle} cycles");
        // for i in 0..n {
        //     for j in 0..m {
        //         eprint!("{}", match a[i][j] {
        //             2 => 'O',
        //             1 => '#',
        //             0 => '.',
        //             _ => todo!()
        //         });
        //     }
        //     eprintln!()
        // }

        let h = hash(&a);
        if seen.contains_key(&h) {
            eprintln!("cycle {} same as cycle {}", cycle, seen[&h]);
            let len = cycle - seen[&h];
            if (BILLION - cycle) % len == 0 {
                break;
            }
        } else {
            seen.insert(h, cycle);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 2 {
                ans += n - i;
            }
        }
    }
    println!("{ans}");
}