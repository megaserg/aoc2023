use std::io;

const EPS: f32 = 1e-5;

fn main() {
    let mut input = String::new();

    _ = io::stdin().read_line(&mut input);
    let (_, times_str) = input.split_once(":").unwrap();
    let times: Vec<u32> = times_str.trim().split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    input.clear();
    _ = io::stdin().read_line(&mut input);
    let (_, dists_str) = input.split_once(":").unwrap();
    let dists: Vec<u32> = dists_str.trim().split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut ans = 1.0;
    for (time, dist) in times.iter().zip(&dists) {
        // x * (time - x) = -x^2 + time*x
        // -b/2a = time/2
        // x^2 - time*x + d = 0
        // time +/- sqrt(time^2 - 4*d) / 2
        let t: f32 = *time as f32;
        let d: f32 = *dist as f32;
        let l: f32 = ((t - (t * t - 4.0 * d).sqrt()) / 2.0 + EPS).ceil();
        let r: f32 = ((t + (t * t - 4.0 * d).sqrt()) / 2.0 - EPS).floor();

        let span = r - l + 1.0;
        ans *= span;
    }

    println!("{}", ans);
    }