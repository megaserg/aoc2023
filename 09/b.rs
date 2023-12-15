use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let mut xs: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let n = xs.len();

        xs.insert(0, -1);

        let mut a: Vec<Vec<i32>> = Vec::new();
        a.push(Vec::new());
        a.push(xs);

        for i in 1..=(n-1) {
            let mut ys = vec![-1];

            for j in 1..=(n-i) {
                ys.push(a[i][j+1] - a[i][j]);
            }
            a.push(ys);
        }

        eprintln!("{:?}", a);

        a[n][0] = 0;
        for i in (1..=(n-1)).rev() {
            let x = a[i][1];
            let d = a[i+1][0];
            a[i][0] = x - d;
        }

        eprintln!("{:?}", a);

        eprintln!("{}", a[1][0]);

        sum += a[1][0];

        input.clear();
    }

    println!("{sum}");
}