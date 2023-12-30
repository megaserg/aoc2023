use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let n = a.len() as i32;
    let m = a[0].len() as i32;

    for j in 0..m {
        eprintln!("j = {j}");
        if a[0][j as usize] == '.' {
            let mut vis: Vec<Vec<bool>> = vec![vec![false; m as usize]; n as usize];
            let mut ans = i32::MIN;
            dfs(0, j, n, m, &a, &mut vis, 0, &mut ans);
            println!("{}", ans);
            break;
        }
    }
}

const DI: [i32; 4] = [-1, 0, 1, 0];
const DJ: [i32; 4] = [0, -1, 0, 1];

fn dfs(i: i32, j: i32, n: i32, m: i32, a: &Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>, acc: i32, ans: &mut i32) {
    if i == n - 1 {
        if acc > *ans {
            eprintln!("{acc}");
            *ans = acc;
        }
        return;
    }
    vis[i as usize][j as usize] = true;
    for k in 0..4 {
        let ni = i + DI[k];
        let nj = j + DJ[k];
        if 0 <= ni && ni < n && 0 <= nj && nj < m && a[ni as usize][nj as usize] != '#' && !vis[ni as usize][nj as usize] {
            dfs(ni, nj, n, m, a, vis, acc + 1, ans);
        }
    }
    vis[i as usize][j as usize] = false;
}