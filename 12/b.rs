use std::io;

fn main() {
    let mut input = String::new();

    let mut sum: u64 = 0;

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (cond, xs_str) = input.trim().split_once(' ').unwrap();

        let mut s: Vec<char> = cond.chars().collect();
        s.push('?');
        let xs: Vec<usize> = xs_str.split(',').map(|x| x.parse().unwrap()).collect();
        let n = s.len();
        let m = xs.len();

        let s: Vec<char> = s.iter().cloned().cycle().take(n * 5 - 1).collect();
        let xs: Vec<usize> = xs.iter().cloned().cycle().take(m * 5).collect();
        let n = s.len();
        let m = xs.len();

        let mut lon = vec![0; n + 1];
        for i in (0..n).rev() {
            if s[i] == '#' || s[i] == '?' {
                lon[i] = lon[i + 1] + 1;
            } else {
                lon[i] = 0;
            }
        }

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
                    if l <= lon[i] {
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