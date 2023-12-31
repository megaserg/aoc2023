use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let n = a.len();
    let m = a[0].len();

    let mut si: i32 = 0;
    let mut sj: i32 = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 'S' {
                (si, sj) = (i as i32, j as i32);
            }
        }
    }

    let di: [i32; 4] = [-1, 0, 1, 0];
    let dj: [i32; 4] = [0, 1, 0, -1];

    let mut q: HashSet<(i32, i32)> = HashSet::new();
    q.insert((si, sj));
    for _ in 0..64 {
        let mut nq = HashSet::new();
        for (i, j) in q {
            for k in 0..4 {
                let ni = (i + di[k]) as usize;
                let nj = (j + dj[k]) as usize;
                if ni < n && nj < m && a[ni][nj] != '#' {
                    nq.insert((ni as i32, nj as i32));
                }
            }
        }
        q = nq;
    }
    println!("{}", q.len());
}