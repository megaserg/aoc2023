use std::io;

fn main() {
    let mut s = String::new();

    let mut sum = 0;
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    loop {
        s.clear();

        match io::stdin().read_line(&mut s) {
            Ok(0) => break, // Reached EOF (Ctrl+D or Ctrl+Z)
            Ok(_) => {
                let mut first_digit = -1;
                let mut last_digit = -1;
                for (i, c) in s.char_indices() {
                    if c.is_digit(10) {
                        if first_digit == -1 {
                            first_digit = c.to_digit(10).unwrap() as i32;
                        }
                        last_digit = c.to_digit(10).unwrap() as i32;
                    }
                    let mut cand = -1;
                    for (idx, word) in words.iter().enumerate() {
                        if s[i..].starts_with(word) {
                            cand = idx as i32;
                        }
                    }   
                    if cand != -1 {
                        if first_digit == -1 {
                            first_digit = cand;
                        }
                        last_digit = cand;
                    }
                }
                sum += first_digit * 10 + last_digit;
            }
            Err(error) => {
                eprintln!("Error reading s: {}", error);
                break;
            }
        }
    }

    println!("{}", sum);
}