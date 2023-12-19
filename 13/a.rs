use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    let mut nbytes = 1;

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

        let mut hv: Vec<u32> = vec![0; m];
        let mut hh: Vec<u32> = vec![0; n];
        for i in 0..n {
            for j in 0..m {
                hv[j] = (hv[j] * 2 + if a[i][j] == '#' {1} else {0}) % 100000007;
                hh[i] = (hh[i] * 2 + if a[i][j] == '#' {1} else {0}) % 100000007;
            }
        }

        let mut added = false;
        for i in 1..n {
            let mut sym = true;
            for k in 1..=i {
                if i+k-1 < n && hh[i-k] != hh[i+k-1] {
                    sym = false;
                    break;
                }
            }
            if sym {
                assert_ne!(i, n);
                assert!(!added);
                added = true;
                sum += i * 100;
                eprintln!("after {i} rows");
            }
        }
        for j in 1..m {
            let mut sym = true;
            for k in 1..=j {
                if j+k-1 < m && hv[j-k] != hv[j+k-1] {
                    sym = false;
                    break;
                }
            }
            if sym {
                assert_ne!(j, m);
                assert!(!added);
                added = true;
                sum += j;
                eprintln!("after {j} columns");
            }
        }
    }

    println!("{sum}");
}