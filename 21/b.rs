use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let n: i32 = a.len() as i32;
    let m: i32 = a[0].len() as i32;

    let mut si: i32 = 0;
    let mut sj: i32 = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i as usize][j as usize] == 'S' {
                (si, sj) = (i as i32, j as i32);
            }
        }
    }

    // let di: [i32; 4] = [-1, 0, 1, 0];
    // let dj: [i32; 4] = [0, 1, 0, -1];

    // let mut q: HashSet<(i32, i32)> = HashSet::new();
    // q.insert((si, sj));

    const STEPS: i64 = 26501365;

    // for step in 0..(65+131+131+131+131+131+131) { //(128+64+3) {
    //     if step % 100 == 0 { eprintln!("{step}"); }
    //
    //     let mut nq = HashSet::new();
    //     for (i, j) in &q {
    //         for k in 0..4 {
    //             let ni = i + di[k];
    //             let nj = j + dj[k];
    //             if a[((ni % n + n) % n) as usize][((nj % m + m) % m) as usize] != '#' {
    //                 nq.insert((ni, nj));
    //             }
    //         }
    //     }
    //     q = nq;
    // }
    // eprintln!("{}", q.len());

    // for i in 0..n {
    //     for j in 0..m {
    //         if a[i as usize][j as usize] == '#' {
    //             print!("#");
    //         } else if q.contains(&(i, j)) {
    //             print!("O");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let k = (STEPS - 65) / 131;
    let a = (k + 1) * (k + 1);
    let b = k * k;
    let c = (a + b - 1) / 2;

    let x = 3762; // obtained by sim above for 65 steps, an "odd" square
    let y = 3697; // obtained by sims for 65+2*131 steps and for 65+4*131 steps,
    let z = 7401; // then solving system of equations for "even" squares and "mid" squares
    let ans = a * x + b * y + c * z;

    println!("{ans}");
}