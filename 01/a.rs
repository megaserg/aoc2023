use std::io;

fn main() {
    let mut input = String::new();

    let mut sum = 0;
    loop {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // Reached EOF (Ctrl+D or Ctrl+Z)
            Ok(_) => {
                let mut first_digit = -1;
                let mut last_digit = -1;
                for c in input.chars() {
                    if c.is_digit(10) {
                        if first_digit == -1 {
                            first_digit = c.to_digit(10).unwrap() as i32;
                        }
                        last_digit = c.to_digit(10).unwrap() as i32;
                    }
                }
                sum += first_digit * 10 + last_digit;
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    println!("{}", sum);
}