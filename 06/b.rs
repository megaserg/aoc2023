use std::io;

const EPS: f64 = 1e-5;

fn main() {
    let mut input = String::new();

    _ = io::stdin().read_line(&mut input);
    let (_, times_str) = input.split_once(":").unwrap();
    let time: u64 = times_str.trim().replace(" ", "").parse::<u64>().unwrap();

    input.clear();
    _ = io::stdin().read_line(&mut input);
    let (_, dists_str) = input.split_once(":").unwrap();
    let dist: u64 = dists_str.trim().replace(" ", "").parse::<u64>().unwrap();

    let t = time as f64;
    let d = dist as f64;
    let l = ((t - (t * t - 4.0 * d).sqrt()) / 2.0 + EPS).ceil();
    let r = ((t + (t * t - 4.0 * d).sqrt()) / 2.0 - EPS).floor();
    let span = r - l + 1.0;

    println!("{}", span);
}