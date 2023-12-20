use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let i = 0;
    let j = 0;
    let di = 0;
    let dj = 1;

    let mut visited = HashSet::new();
    dfs(i, j, di, dj, &a, &mut visited);

    let cells: HashSet<(i32, i32)> = visited.iter().map(|&(i, j, _, _)| (i, j)).collect();

    println!("{}", cells.len());
}

fn dfs(
    i: i32,
    j: i32,
    mut di: i32,
    mut dj: i32,
    a: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32, i32, i32)>,
) {
    if !(0..a.len()).contains(&(i as usize)) { return; }
    if !(0..a[0].len()).contains(&(j as usize)) { return; }

    if visited.contains(&(i, j, di, dj)) { return; }
    visited.insert((i, j, di, dj));

    // eprintln!("{:?}", (i, j, di, dj));

    match a[i as usize][j as usize] {
        '.' => {
            dfs(i + di, j + dj, di, dj, a, visited);
        }
        '/' => {
            (di, dj) = (-dj, -di);
            dfs(i + di, j + dj, di, dj, a, visited);
        }
        '\\' => {
            (di, dj) = (dj, di);
            dfs(i + di, j + dj, di, dj, a, visited);
        }
        '|' => {
            if dj == 0 {
                dfs(i + di, j + dj, di, dj, a, visited);
            } else {
                (di, dj) = (-1, 0);
                dfs(i + di, j + dj, di, dj, a, visited);
                (di, dj) = (1, 0);
                dfs(i + di, j + dj, di, dj, a, visited);
            }
        }
        '-' => {
            if di == 0 {
                dfs(i + di, j + dj, di, dj, a, visited);
            } else {
                (di, dj) = (0, -1);
                dfs(i + di, j + dj, di, dj, a, visited);
                (di, dj) = (0, 1);
                dfs(i + di, j + dj, di, dj, a, visited);
            }
        }
        _ => todo!()
    }
}