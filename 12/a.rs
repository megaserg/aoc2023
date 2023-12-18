use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (cond, xs_str) = input.trim().split_once(' ').unwrap();
        let s: Vec<char> = cond.chars().collect();
        let xs: Vec<usize> = xs_str.split(',').map(|x| x.parse().unwrap()).collect();
        let n = s.len();
        let m = xs.len();

        // eprintln!("{:?}", s);
        // eprintln!("{:?}", xs);

        let mut d = vec![vec![0; m + 1]; n + 1];
        d[0][0] = 1;
        for i in 0..n {
            for j in 0..=m {
                if d[i][j] == 0 { continue; }

                let c = s[i];
                if c == '.' || c == '?' {
                    d[i + 1][j] += d[i][j];
                }
                if j < m && (c == '#' || c == '?') {
                    let l = xs[j];
                    let mut mat = true;
                    if i + l > n {
                        // eprintln!("{i}, {j}: no match 1");
                        mat = false;
                    }
                    if mat {
                        for p in 0..l {
                            if s[i + p] == '.' {
                                // eprintln!("{i}, {j}: no match 2");
                                mat = false;
                                break;
                            }
                        }
                    }
                    if mat {
                        if i + l < n {
                            if s[i + l] == '.' || s[i + l] == '?' {
                                d[i + l + 1][j + 1] += d[i][j];
                            }
                        } else {
                            assert_eq!(i + l, n);
                            d[i + l][j + 1] += d[i][j];
                        }
                    }
                }
            }
        }

        // for i in 0..=n {
        //     for j in 0..=m {
        //         eprint!("{} ", d[i][j]);
        //     }
        //     eprintln!();
        // }
        eprintln!("{}", d[n][m]);

        sum += d[n][m];

        input.clear();
    }

    println!("{sum}");
}