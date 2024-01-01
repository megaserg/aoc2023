use std::collections::{HashMap, HashSet};
use std::io;

const MAXN: usize = 2000;
const MAXM: usize = 8000;

fn main() {
    let mut input = String::new();

    let mut id: HashMap<String, usize> = HashMap::new();

    let mut n = 0;
    let mut m = 0;

    let mut first: [usize; MAXN] = [0; MAXN];
    let mut next: [usize; MAXM] = [0; MAXM];
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
            next[m] = first[i];
            first[i] = m;
            ii[m] = i;
            jj[m] = j;
            m += 1;
            next[m] = first[j];
            first[j] = m;
            ii[m] = j;
            jj[m] = i;
        }

        input.clear();
    }

    eprintln!("{n} {m}");

    let mut vis = vec![false; n + 1];

    let mut tin = vec![0; n + 1];
    let mut tout = vec![0; n + 1];

    let mut poss: Vec<usize> = Vec::new();

    for u in 1..=m {
        if u % 2 == 0 { continue; }

        let mut exc = HashSet::new();
        exc.insert(u);
        exc.insert(u + 1);

        let mut time = 0;

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res1 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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
        let res2 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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
        let res3 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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

        let mut time = 0;

        for i in 1..=n { vis[i] = false; }
        let mut path = HashSet::new();
        let res1 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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
        let res2 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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
        let res3 = dfs(ii[u], jj[u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
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
                let mut time = 0;
                for i in 1..=n { vis[i] = false; }
                let mut path = HashSet::new();
                let r = dfs(ii[*u], jj[*u], &mut time, &first, &next, &jj, &mut tin, &mut tout, &mut vis, &exc, &mut path);
                if !r {
                    eprintln!("{} {} {}", *u, *v, *w);
                    let sz1: usize = vis.iter().map(|x| if *x {1} else {0}).sum();
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
    time: &mut usize,
    first: &[usize; MAXN],
    next: &[usize; MAXM],
    jj: &[usize; MAXM],
    tin: &mut Vec<usize>,
    tout: &mut Vec<usize>,
    vis: &mut Vec<bool>,
    exc: &HashSet<usize>,
    path: &mut HashSet<usize>,
) -> bool {
    vis[v] = true;

    *time += 1;
    tin[v] = *time;

    if v == target {
        return true;
    }

    let mut u = first[v];
    while u != 0 {
        if exc.contains(&u) {
            u = next[u];
            continue;
        }
        let w = jj[u];
        if !vis[w] {
            path.insert(u);
            if dfs(w, target, time, &first, &next, &jj, tin, tout, vis, exc, path) {
                return true;
            }
            path.remove(&u);
        }
        u = next[u];
    }

    *time += 1;
    tout[v] = *time;
    return false;
}