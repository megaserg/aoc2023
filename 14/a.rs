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

    let mut ans = 0;
    for j in 0..m {
        let mut place = 0;
        for i in 0..n {
            if a[i][j] == 'O' {
                ans += n - place;
                place += 1;
            } else if a[i][j] == '#' {
                place = i+1;
            }
        }
    }

    println!("{ans}");
}