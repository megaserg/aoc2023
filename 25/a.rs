use std::collections::{HashMap, HashSet};
use std::io;

const MAXN: usize = 2000;
const MAXM: usize = 8000;

fn main() {
    let mut input = String::new();

    let mut id: HashMap<String, usize> = HashMap::new();

    let mut n = 0;
    let mut m = 0;

    let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; MAXN];
    let mut ii: [usize; MAXM] = [0; MAXM];
    let mut jj: [usize; MAXM] = [0; MAXM];

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (v, ws) = input.trim().split_once(':').unwrap();

        let ws = ws.trim().split_whitespace();

        let v = String::from(v);
        if !id.contains_key(&v) {
            n += 1;
            id.insert(v.clone(), n);
        };
        let i = *id.get(&v).unwrap();

        for w in ws {
            let w = String::from(w);
            if !id.contains_key(&w) {
                n += 1;
                id.insert(w.clone(), n);
            };
            let j = *id.get(&w).unwrap();
            m += 1;
            g[i].push((m, j));
            ii[m] = i;
            jj[m] = j;
            m += 1;
            g[j].push((m, i));
            ii[m] = j;
            jj[m] = i;
        }

        input.clear();
    }

    eprintln!("{n} {m}");

    let mut vis = vec![false; n + 1];

    let mut poss: Vec<usize> = Vec::new();

    for u in 1..=m {
        if u % 2 == 0 { continue; }

        let mut exc = HashSet::new();
        exc.insert(u);
        exc.insert(u + 1);

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res1 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        assert!(res1);

        for edge in path {
            exc.insert(edge);
            if edge % 2 == 0 {
                exc.insert(edge - 1);
            } else {
                exc.insert(edge + 1);
            }
        }

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res2 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        assert!(res2);

        for edge in path {
            exc.insert(edge);
            if edge % 2 == 0 {
                exc.insert(edge - 1);
            } else {
                exc.insert(edge + 1);
            }
        }

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res3 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        if !res3 {
            poss.push(u);
        }
    }

    let poss_set: HashSet<usize> = poss.clone().into_iter().collect();
    let mut new_poss: Vec<usize> = Vec::new();

    for u_ptr in &poss {
        let u = *u_ptr;

        let mut exc = HashSet::new();
        exc.insert(u);
        exc.insert(u + 1);

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res1 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        assert!(res1);

        let mut contains_bridge = false;
        for edge in path {
            if poss_set.contains(&edge) {
                contains_bridge = true;
                exc.insert(edge);
                if edge % 2 == 0 {
                    exc.insert(edge - 1);
                } else {
                    exc.insert(edge + 1);
                }
            }
        }
        if !contains_bridge {
            eprintln!("fake bridge 1!");
            continue;
        }

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res2 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        assert!(res2);

        let mut contains_bridge = false;
        for edge in path {
            if poss_set.contains(&edge) {
                contains_bridge = true;
                exc.insert(edge);
                if edge % 2 == 0 {
                    exc.insert(edge - 1);
                } else {
                    exc.insert(edge + 1);
                }
            }
        }
        if !contains_bridge {
            eprintln!("fake bridge 2!");
            continue;
        }

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res3 = dfs(ii[u], jj[u], &g, &mut vis, &exc, &mut path);
        if !res3 {
            eprintln!("u = {u}");
            new_poss.push(u);
        }
    }

    for u in &new_poss {
        for v in &new_poss {
            if *u >= *v { continue; };
            for w in &new_poss {
                if *v >= *w { continue; }
                let mut exc = HashSet::new();
                exc.insert(*u);
                exc.insert(*u + 1);
                exc.insert(*v);
                exc.insert(*v + 1);
                exc.insert(*w);
                exc.insert(*w + 1);
                for i in 1..=n { vis[i] = false; }
                let mut path = HashSet::new();
                let r = dfs(ii[*u], jj[*u], &g, &mut vis, &exc, &mut path);
                if !r {
                    eprintln!("{} {} {}", *u, *v, *w);
                    let sz1: usize = vis.iter().map(|x| if *x { 1 } else { 0 }).sum();
                    let sz2: usize = n - sz1;
                    eprintln!("{} x {}", sz1, sz2);
                    println!("{}", sz1 * sz2);
                }
            }
        }
    }
}

fn dfs(
    v: usize,
    target: usize,
    g: &Vec<Vec<(usize, usize)>>,
    vis: &mut Vec<bool>,
    exc: &HashSet<usize>,
    path: &mut HashSet<usize>,
) -> bool {
    vis[v] = true;

    if v == target {
        return true;
    }

    for (u_ptr, w_ptr) in &g[v] {
        let u = *u_ptr;
        let w = *w_ptr;
        if exc.contains(&u) {
            continue;
        }
        if !vis[w] {
            path.insert(u);
            if dfs(w, target, &g, vis, exc, path) {
                return true;
            }
            path.remove(&u);
        }
    }

    return false;
}