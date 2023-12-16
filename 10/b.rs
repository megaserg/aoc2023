use std::collections::{HashMap};
use std::io;
use Side::{East, North, South, West};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Side {
    North,
    South,
    East,
    West,
}

fn main() {
    let conn: HashMap<char, [Side;2]> = HashMap::from([
        ('|', [North, South]),
        ('-', [East, West]),
        ('L', [North, East]),
        ('J', [North, West]),
        ('7', [South, West]),
        ('F', [South, East]),
    ]);

    let opp: HashMap<Side, Side> = HashMap::from([
        (North, South),
        (South, North),
        (East, West),
        (West, East),
    ]);

    let dir: HashMap<Side, (i32, i32)> = HashMap::from([
        (North, (-1, 0)),
        (South, (1, 0)),
        (East, (0, 1)),
        (West, (0, -1)),
    ]);

    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let n = a.len();
    let m = a[0].len();

    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 'S' {
                for c in conn.keys() {
                    for id in 0..conn[c].len() {
                        a[i][j] = *c;
                        let mut si = i;
                        let mut sj = j;
                        let mut d = conn[c][id];
                        let mut success = false;
                        let mut steps = 0;
                        let mut area: i32 = 0;
                        loop {
                            let ni = (si as i32 + dir[&d].0) as usize;
                            let nj = (sj as i32 + dir[&d].1) as usize;
                            if (0..n).contains(&ni) && (0..m).contains(&nj) && a[ni][nj] != '.' &&
                                (opp[&d] == conn[&a[ni][nj]][0] || opp[&d] == conn[&a[ni][nj]][1]) {
                                let y = si as i32;
                                if a[si][sj] == '-' {
                                    if d == East {
                                        area -= y;
                                    } else {
                                        area += y - 1;
                                    }
                                } else if a[si][sj] == 'L' {
                                    if d == East {
                                        area -= y;
                                    }
                                } else if a[si][sj] == 'J' {
                                    if d == North {
                                        area -= y;
                                    }
                                } else if a[si][sj] == 'F' {
                                    if d == South {
                                        area += y - 1;
                                    }
                                } else if a[si][sj] == '7' {
                                    if d == West {
                                        area += y - 1;
                                    }
                                }

                                si = ni;
                                sj = nj;
                                if opp[&d] == conn[&a[ni][nj]][0] {
                                    d = conn[&a[ni][nj]][1];
                                } else {
                                    d = conn[&a[ni][nj]][0];
                                }
                                steps += 1;
                            } else {
                                break;
                            }
                            if si == i && sj == j {
                                success = true;
                                break;
                            }
                        }
                        if area < 0 {
                            success = false;
                        }
                        if success {
                            eprintln!("{} {}", c, steps);
                            println!("{}", area);
                        }
                    }
                }
            }
        }
    }
}