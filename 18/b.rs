use std::io;

fn main() {
    let mut input = String::new();

    let mut area: i64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut left = false;
    let mut right = false;
    let mut buf_l = 0;
    let mut buf_r = 0;

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (_, rest) = input.trim().split_once(' ').unwrap();
        let (_, hex) = rest.split_once(' ').unwrap();
        let dir = match hex.chars().nth(7).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => todo!()
        };
        let len = i64::from_str_radix(&hex[2..7], 16).unwrap();

        // RDL: +i+1 after +len / RUL: +i before -len
        // ..... .OOOO
        // .###. .###O
        // OOO#. ...#O
        // .###. .###O
        // ..... .OOOO

        // RDR: 0 / LUL: 0
        // ..... .OOO.
        // .##.. .##O.
        // .O#.. ..#O.
        // .O##. ..##.
        // .OOO. .....

        // LDR: -i before +len / LUR: -i after -len
        // OOOO. .....
        // O###. .###.
        // O#... .#OOO
        // O###. .###.
        // OOOO. .....

        // LDL: +len / RUR: +len
        // .OOO. .....
        // .O##. ..##.
        // .O#.. ..#O.
        // .##.. .##O.
        // ..... .OOO.

        if dir == 'R' {
            area -= len * (i);
            area += buf_r;
            buf_r = 0;
            right = true;
            left = false;
            j += len;
        } else if dir == 'L' {
            area += len * (i + 1);
            area += buf_l;
            buf_l = 0;
            left = true;
            right = false;
            j -= len;
        } else if dir == 'D' {
            if right {
                i += len;
                buf_l = i + 1; // RDL
                buf_r = 0; // RDR
            } else if left {
                buf_l = len; // LDL
                buf_r = -(i + 1); // LDR
                i += len;
            }
        } else if dir == 'U' {
            if left {
                i -= len;
                buf_l = 0; // LUL
                buf_r = -i; // LUR
            } else if right {
                buf_l = i; // RUL
                buf_r = len; // RUR
                i -= len;
            }
        } else {
            assert!(false);
        }

        eprintln!("{} {}, new area: {area}, L: {buf_l}, R: {buf_r}", dir, len);

        input.clear();
    }

    eprintln!("{buf_l} {buf_r}");

    println!("{area}");
}