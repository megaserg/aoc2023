use std::io;

fn is_pow2(n: i32) -> bool {
    n & (n - 1) == 0
}

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    let mut nbytes = 1;
    let mut maxn = 0;
    let mut maxm = 0;

    while nbytes > 0 {
        let mut a: Vec<Vec<char>> = Vec::new();

        loop {
            nbytes = io::stdin().read_line(&mut input).unwrap();
            if nbytes == 0 || input.trim().len() == 0 { break; }
            a.push(input.trim().chars().collect());
            input.clear();
        }

        let n = a.len();
        let m = a[0].len();

        let mut hv: Vec<i32> = vec![0; m];
        let mut hh: Vec<i32> = vec![0; n];
        for i in 0..n {
            for j in 0..m {
                hv[j] = hv[j] * 2 + if a[i][j] == '#' { 1 } else { 0 };
                hh[i] = hh[i] * 2 + if a[i][j] == '#' { 1 } else { 0 };
            }
        }

        let mut added = false;
        for i in 1..n {
            let mut sym = true;
            let mut diffs = 0;
            for k in 1..=i {
                if i + k - 1 < n && hh[i - k] != hh[i + k - 1] {
                    if is_pow2(hh[i - k] ^ hh[i + k - 1]) {
                        diffs += 1;
                    } else {
                        sym = false;
                        break;
                    }
                }
            }
            if sym && diffs == 1 {
                eprintln!("after {i} rows");
                assert_ne!(i, n);
                assert!(!added);
                added = true;
                sum += i * 100;
            }
        }
        for j in 1..m {
            let mut sym = true;
            let mut diffs = 0;
            for k in 1..=j {
                if j + k - 1 < m && hv[j - k] != hv[j + k - 1] {
                    if is_pow2(hv[j - k] ^ hv[j + k - 1]) {
                        diffs += 1;
                    } else {
                        sym = false;
                        break;
                    }
                }
            }
            if sym && diffs == 1 {
                eprintln!("after {j} columns");
                assert_ne!(j, m);
                assert!(!added);
                added = true;
                sum += j;
            }
        }

        maxn = std::cmp::max(maxn, n);
        maxm = std::cmp::max(maxm, m);
        eprintln!();
    }

    eprintln!("{maxn}, {maxm}");
    println!("{sum}");
}