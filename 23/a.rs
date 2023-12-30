use std::collections::HashSet;
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
        if a[0][j as usize] == '.' {
            let mut vis = HashSet::new();
            let ans = dfs(0, j, n, m, &a, &mut vis);
            println!("{}", ans);
            break;
        }
    }
}

const DI: [i32; 4] = [-1, 0, 1, 0];
const DJ: [i32; 4] = [0, -1, 0, 1];

fn dfs(i: i32, j: i32, n: i32, m: i32, a: &Vec<Vec<char>>, vis: &mut HashSet<(i32, i32)>) -> i32 {
    if i == n - 1 {
        return 0;
    }
    vis.insert((i, j));
    let mut ans = i32::MIN;
    for k in 0..4 {
        let ni = i + DI[k];
        let nj = j + DJ[k];
        if 0 <= ni && ni < n && 0 <= nj && nj < m && !vis.contains(&(ni, nj)){
            let c = a[i as usize][j as usize];
            let d = a[ni as usize][nj as usize];
            if c == '.' || c == '>' && k == 3 || c == '<' && k == 1 || c == 'v' && k == 2 || c == '^' && k == 0 {
                if d != '#' {
                    ans = std::cmp::max(ans, 1 + dfs(ni, nj, n, m, a, vis));
                }
            }
        }
    }
    vis.remove(&(i, j));
    return ans;
}