use std::collections::HashSet;
use std::io;

#[derive(Clone)]
struct Block {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

fn paint(a: &mut [[[usize; 310]; 10]; 10], b: &Block, color: usize) {
    for x in b.x1..=b.x2 {
        for y in b.y1..=b.y2 {
            for z in b.z1..=b.z2 {
                a[x][y][z] = color;
            }
        }
    }
}

fn can_move(id: usize, a: &[[[usize; 310]; 10]; 10], b: &Block) -> bool {
    assert!(id > 0);
    for x in b.x1..=b.x2 {
        for y in b.y1..=b.y2 {
            for z in b.z1..=b.z2 {
                if z == 0 || !(a[x][y][z - 1] == 0 || a[x][y][z - 1] == id) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn move_down(id: usize, a: &mut [[[usize; 310]; 10]; 10], b: &mut Vec<Block>) {
    assert!(id > 0);
    paint(a, &b[id - 1], 0);
    b[id - 1].z1 -= 1;
    b[id - 1].z2 -= 1;
    paint(a, &b[id - 1], id);
}

fn main() {
    let mut input = String::new();

    let mut maxx = 0;
    let mut maxy = 0;
    let mut maxz = 0;

    let mut a: [[[usize; 310]; 10]; 10] = [[[0; 310]; 10]; 10];
    let mut b = Vec::new();

    let mut n = 0;
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (c1, c2) = input.trim().split_once('~').unwrap();
        let (x1, d1) = c1.split_once(',').unwrap();
        let (y1, z1) = d1.split_once(',').unwrap();
        let (x2, d2) = c2.split_once(',').unwrap();
        let (y2, z2) = d2.split_once(',').unwrap();
        let x1: usize = x1.parse().unwrap();
        let y1: usize = y1.parse().unwrap();
        let z1: usize = z1.parse().unwrap();
        let x2: usize = x2.parse().unwrap();
        let y2: usize = y2.parse().unwrap();
        let z2: usize = z2.parse().unwrap();

        maxx = std::cmp::max(maxx, x1);
        maxx = std::cmp::max(maxx, x2);
        maxy = std::cmp::max(maxy, y1);
        maxy = std::cmp::max(maxy, y2);
        maxz = std::cmp::max(maxz, z1);
        maxz = std::cmp::max(maxz, z2);

        assert!(x1 == x2 && y1 == y2 && z1 <= z2 || x1 == x2 && z1 == z2 && y1 <= y2 || y1 == y2 && z1 == z2 && x1 <= x2);

        n += 1;
        b.push(Block { x1, y1, z1, x2, y2, z2 });
        paint(&mut a, &b[n - 1], n);

        input.clear();
    }

    loop {
        let mut moved = false;
        for id in 1..=n {
            while can_move(id, &mut a, &b[id - 1]) {
                move_down(id, &mut a, &mut b);
                moved = true;
            }
        }
        if !moved { break; }
    }

    let mut ans = 0;
    for id in 1..=n {
        let mut aa = a.clone();
        let mut bb = b.clone();
        paint(&mut aa, &bb[id - 1], 0);

        let mut moving = HashSet::new();
        loop {
            let mut moved = false;
            for jd in 1..=n {
                if id == jd { continue; }
                while can_move(jd, &mut aa, &bb[jd - 1]) {
                    moving.insert(jd);
                    move_down(jd, &mut aa, &mut bb);
                    moved = true;
                }
            }
            if !moved { break; }
        }

        ans += moving.len();
    }

    eprintln!("{maxx} {maxy} {maxz}");
    println!("{ans}");
}