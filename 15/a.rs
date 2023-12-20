use std::io;

fn hash(s: &str) -> u32{
    let mut v = 0;
    for c in s.chars() {
        v = ((v + c as u32) * 17) % 256;
    }
    return v;
}

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let cmds = input.trim().split(',');
        for cmd in cmds {
            sum += hash(cmd);
        }
        input.clear();
    }

    println!("{sum}");
}
