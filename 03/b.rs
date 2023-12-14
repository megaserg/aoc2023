use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    let mut a: Vec<Vec<char>> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        a.push(input.trim().chars().collect());
        input.clear();
    }

    let n = a.len();
    let m = a[0].len();

    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    for i in 0..n {
        let mut x = 0;
        let mut gear: (i32, i32) = (-1, -1);
        for j in 0..m {
            if a[i][j].is_digit(10) {
                x = x * 10 + a[i][j].to_digit(10).unwrap();
                for di in [-1, 0, 1].iter() {
                    for dj in [-1, 0, 1].iter() {
                        let ni = (i as i32 + di) as usize;
                        let nj = (j as i32 + dj) as usize;
                        if 0 <= ni && ni < n && 0 <= nj && nj < m {
                            if a[ni][nj] == '*' {
                                gear = (ni as i32, nj as i32);
                            }
                        }
                    }
                }
            } else {
                if x > 0 && gear != (-1, -1) {
                    gears.entry(gear).or_default().push(x);
                }
                x = 0;
                gear = (-1, -1);
            }
        }
        if x > 0 && gear != (-1, -1) {
            gears.entry(gear).or_default().push(x);
        }
    }

    let mut sum = 0;

    for (_, v) in gears {
        println!("{:?}", v);
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }

    println!("{}", sum);
}