use std::io;

fn parse_triple(s: &str) -> (f64, f64, f64) {
    let mut parts = s.split(',');
    let x = parts.next().unwrap().trim().parse::<f64>().unwrap();
    let y = parts.next().unwrap().trim().parse::<f64>().unwrap();
    let z = parts.next().unwrap().trim().parse::<f64>().unwrap();
    return (x, y, z);
}

// const MIN_TEST: f64 = 7.0;
// const MAX_TEST: f64 = 27.0;

const MIN_TEST: f64 = 200000000000000.0;
const MAX_TEST: f64 = 400000000000000.0;

fn main() {
    let mut input = String::new();

    let mut p = Vec::new();
    let mut v = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (pos, vel) = input.trim().split_once(" @ ").unwrap();
        p.push(parse_triple(pos));
        v.push(parse_triple(vel));

        // x = x0 + dx * t
        // y = y0 + dy * t
        // t = (x - x0) / dx
        // y = y0 + dy / dx * (x - x0)
        // (y - y0) * dx = (x - x0) * dy
        // dy * x + (-dx) * y + (y0 * dx - x0 * dy) = 0

        // a = a0 + da * t
        // b = b0 + db * t
        // t? x0 + dx * t = a0 + da * t
        // t = -(a0 - x0) / (da - dx)
        input.clear();
    }

    let n = p.len();

    let mut ans = 0;
    for i in 0..n {
        let p1 = p[i];
        let v1 = v[i];
        let a1 = v1.1;
        let b1 = -v1.0;
        let c1 = p1.1 * v1.0 - p1.0 * v1.1;

        for j in (i + 1)..n {
            let p2 = p[j];
            let v2 = v[j];
            eprintln!("testing {:?} x {:?}", p1, p2);
            let a2 = v2.1;
            let b2 = -v2.0;
            let c2 = p2.1 * v2.0 - p2.0 * v2.1;

            let det = a1 * b2 - a2 * b1;
            let xd = -(c1 * b2 - c2 * b1);
            let yd = -(a1 * c2 - a2 * c1);

            if det != 0.0 {
                let xc = (xd) / (det);
                let yc = (yd) / (det);
                eprintln!("{i}, {j} intersect at {xc}, {yc}");
                let t1 = (xc - p1.0) / (v1.0);
                if t1 < 0.0 {
                    eprintln!("outside for A");
                    continue;
                }
                let t2 = (xc - p2.0) / (v2.0);
                if t2 < 0.0 {
                    eprintln!("outside for B");
                    continue;
                }
                if MIN_TEST <= xc && xc <= MAX_TEST && MIN_TEST <= yc && yc <= MAX_TEST {
                    eprintln!("yes");
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
