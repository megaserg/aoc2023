use std::io;

fn main() {
    let mut input = String::new();

    let mut i = 0;
    let mut is: Vec<i64> = Vec::new();
    let mut js: Vec<i64> = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        for (j, c) in input.trim().chars().enumerate() {
            if c == '#' {
                is.push(i);
                js.push(j as i64);
            }
        }
        i += 1;
        input.clear();
    }

    let si = get_sum(&is);
    js.sort();
    let sj = get_sum(&js);

    println!("{}", si + sj);
}

fn get_sum(is: &Vec<i64>) -> i64 {
    let n = is.len() as i64;
    let mut si = 0;
    let mut di = 0;
    let mut prev_i: i64 = -1;

    for (k, &i) in is.iter().enumerate() {
        if i > prev_i + 1 {
            di += (i - (prev_i + 1)) * 999999;
        }
        let k = k as i64;
        si += (i + di) * (k - (n - k - 1));
        prev_i = i;
    }
    si
}