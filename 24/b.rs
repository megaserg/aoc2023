use std::io;

fn parse_triple(s: &str) -> (f64, f64, f64) {
    let mut parts = s.split(',');
    let x = parts.next().unwrap().trim().parse::<f64>().unwrap();
    let y = parts.next().unwrap().trim().parse::<f64>().unwrap();
    let z = parts.next().unwrap().trim().parse::<f64>().unwrap();
    return (x, y, z);
}

fn d(a: &f64, b: &f64, c: &f64, d: &f64) -> f64 {
    return a * d - b * c;
}

fn intersect_line_plane(p: &(f64, f64, f64), v: &(f64, f64, f64), base0: &(f64, f64, f64), base1: &(f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = &p;
    let (dx, dy, dz) = &v;
    let (x0, y0, z0) = &base0;
    let (x1, y1, z1) = &base1;

    let nom = d(x0, x1, y0, y1) * d(x, x0, z, z0) + d(x0, x1, z0, z1) * d(x0, x, y0, y);
    let den = d(x0, x1, y0, y1) * d(x0, dx, z0, dz) - d(x0, x1, z0, z1) * d(x0, dx, y0, dy);
    let t = nom / den;

    let beta = (d(x0, x, y0, y) + d(x0, dx, y0, dy) * t) / d(x0, x1, y0, y1);
    let alpha = x / x0 + dx / x0 * t - x1 / x0 * beta;

    return (t, alpha, beta);
}

fn cross(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> (f64, f64, f64) {
    return (d(&a.1, &a.2, &b.1, &b.2), -d(&a.0, &a.2, &b.0, &b.2), d(&a.0, &a.1, &b.0, &b.1));
}

fn dot(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> f64 {
    return a.0 * b.0 + a.1 * b.1 + a.2 * b.2;
}

fn len_sq(a: &(f64, f64, f64)) -> f64 {
    return a.0 * a.0 + a.1 * a.1 + a.2 * a.2;
}

fn add(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> (f64, f64, f64) {
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
}

fn sub(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> (f64, f64, f64) {
    return (a.0 - b.0, a.1 - b.1, a.2 - b.2);
}

fn scale(a: &(f64, f64, f64), f: f64) -> (f64, f64, f64) {
    return (a.0 * f, a.1 * f, a.2 * f);
}

fn parallel(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> bool {
    return len_sq(&cross(a, b)) < 1e-10;
}

fn shift(p: &(f64, f64, f64), v: &(f64, f64, f64), t: f64) -> (f64, f64, f64) {
    return (p.0 + v.0 * t, p.1 + v.1 * t, p.2 + v.2 * t);
}

fn is_integer(x: f64) -> bool {
    return x.fract().abs() < 1e-10;
}

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
        // z = z0 + dz * t
        // (X0 - x1) * (DY - dy1)  = (Y0 - y1) * (DX - dx1)
        // (X0 - x1) * (DZ - dz1)  = (Z0 - z1) * (DX - dx1)
        // (Z0 - z1) * (DY - dy1)  = (Y0 - y1) * (DZ - dz1)

        // (X0 - x2) / (DX - dx2) = (Y0 - y2) / (DY - dy2)
        // (X0 - x2) / (DX - dx2) = (Z0 - z2) / (DZ - dz2)
        // (X0 - x3) / (DX - dx3) = (Y0 - y3) / (DY - dy3)
        // (X0 - x3) / (DX - dx3) = (Z0 - z3) / (DZ - dz3)


        // y1 + dy1 = Y0 + DY * t1
        // z1 + dz1 = Z0 + DZ * t1
        // x2 + dx2 = X0 + DX * t2
        // y2 + dy2 = Y0 + DY * t2
        // z2 + dz2 = Z0 + DZ * t2
        // x3 + dx3 = X0 + DX * t3
        // y3 + dy3 = Y0 + DY * t3
        // z3 + dz3 = Z0 + DZ * t3

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

    // let mut ans = 0;
    // for i in 0..n {
    //     let p1 = p[i];
    //     let v1 = v[i];
    //     let a1 = v1.1;
    //     let b1 = -v1.0;
    //     let c1 = p1.1 * v1.0 - p1.0 * v1.1;
    //
    //     for j in (i + 1)..n {
    //         let p2 = p[j];
    //         let v2 = v[j];
    //         eprintln!("testing {:?} x {:?}", p1, p2);
    //         let a2 = v2.1;
    //         let b2 = -v2.0;
    //         let c2 = p2.1 * v2.0 - p2.0 * v2.1;
    //
    //         let det = a1 * b2 - a2 * b1;
    //         let xd = -(c1 * b2 - c2 * b1);
    //         let yd = -(a1 * c2 - a2 * c1);
    //
    //         if det != 0.0 {
    //             let xc = (xd) / (det);
    //             let yc = (yd) / (det);
    //             eprintln!("{i}, {j} intersect at {xc}, {yc}");
    //             let t1 = (xc - p1.0) / (v1.0);
    //             if t1 < 0.0 {
    //                 eprintln!("outside for A");
    //                 continue;
    //             }
    //             let t2 = (xc - p2.0) / (v2.0);
    //             if t2 < 0.0 {
    //                 eprintln!("outside for B");
    //                 continue;
    //             }
    //             if MIN_TEST <= xc && xc <= MAX_TEST && MIN_TEST <= yc && yc <= MAX_TEST {
    //                 eprintln!("yes");
    //                 ans += 1;
    //             }
    //         }
    //     }
    // }

    eprintln!("n = {n}");

    for i in 0..n {
        let p1 = p[i];
        let v1 = v[i];

        for j in (i + 1)..n {
            let p2 = p[j];
            let v2 = v[j];
            // eprintln!("testing {:?} x {:?}", p1, p2);

            if parallel(&v1, &v2) {
                eprintln!("{i} and {j} are parallel");

                let base0 = v1;
                let base1 = sub(&p2, &p1);

                eprintln!("base0: {:?}", base0);
                eprintln!("base1: {:?}", base1);

                let normal = cross(&base0, &base1);
                assert!(len_sq(&normal) > 0.01);

                // let answer = (-3.0, 1.0, 2.0);
                // let dt = dot(&normal, &answer);
                // assert!(dt == 0.0);

                let mut k = 0;
                while k < n {
                    if k == i || k == j {
                        k += 1;
                        continue;
                    }
                    if !parallel(&v[k], &v1) { break; }
                    k += 1;
                }
                eprintln!("k = {k}");

                let mut l = 0;
                while l < n {
                    if l == i || l == j || l == k {
                        l += 1;
                        continue;
                    }
                    if !parallel(&v[l], &v1) { break; }
                    l += 1;
                }
                eprintln!("l = {l}");

                assert!(!parallel(&v[k], &v1));
                assert!(!parallel(&v[l], &v1));

                eprintln!("dot k: {}", dot(&normal, &v[k]));
                eprintln!("dot l: {}", dot(&normal, &v[l]));

                let (t0, a0, b0) = intersect_line_plane(&sub(&p[k], &p1), &v[k], &base0, &base1);
                assert!(t0 > 0.0);
                eprintln!("t0 = {t0}");

                let (t1, a1, b1) = intersect_line_plane(&sub(&p[l], &p1), &v[l], &base0, &base1);
                assert!(t1 > 0.0);
                eprintln!("t1 = {t1}");

                let fp0 = shift(&p[k], &v[k], t0);
                eprintln!("fp0 = {:?}, a0 = {}, b0 = {}", fp0, a0, b0);
                assert!(fp0 == add(&p1, &add(&scale(&base0, a0), &scale(&base1, b0))));
                let fp1 = shift(&p[l], &v[l], t1);
                eprintln!("fp1 = {:?}", fp1);
                assert!(fp1 == add(&p1, &add(&scale(&base0, a1), &scale(&base1, b1))));

                let v = scale(&sub(&fp1, &fp0), 1.0 / (t1 - t0));
                eprintln!("{:?}", v);

                let p = sub(&fp0, &scale(&v, t0));
                eprintln!("{:?}", p);
            }
        }
    }

    /*for i in 0..n {
        for t in 0..10 {
            eprintln!("t = {} : {:?}", t, shift(&p[i], &v[i], t as f64));
        }
    }*/

    for vx_i in -1000..=1000 {
        if vx_i % 100 == 0 { eprintln!("trying {vx_i}"); }
        for vy_i in -1000..=1000 {
            // for vz_i in -100..=100 {
            let vx = vx_i as f64;
            let vy = vy_i as f64;
            // let vz = vz_i as f64;

            let mut mat = 0;
            let mut vz_0 = 0.0;
            let mut px_0 = 0.0;
            let mut py_0 = 0.0;
            let mut pz_0 = 0.0;

            let K = 10;
            for i in 0..K {
                let p1 = p[i];
                let v1 = v[i];
                let (x1, y1, z1) = p1;
                let (dx1, dy1, dz1) = v1;

                for j in 0..K {
                    if i == j { continue; }

                    let p2 = p[j];
                    let v2 = v[j];
                    let (x2, y2, z2) = p2;
                    let (dx2, dy2, dz2) = v2;

                    let nom = d(&(x2 - x1), &(dx1 - vx), &(y2 - y1), &(dy1 - vy));
                    let den = d(&(dx1 - vx), &(dx2 - vx), &(dy1 - vy), &(dy2 - vy));
                    let t2 = nom / den;
                    let t1 = ((x2 - x1) + (dx2 - vx) * t2) / (dx1 - vx);

                    if t1 > 0.0 && is_integer(t1) && t2 > 0.0 && is_integer(t2) {
                        let vz = ((z2 + dz2 * t2) - (z1 + dz1 * t1)) / (t2 - t1);

                        if is_integer(vz) {
                            let px_1 = x1 + dx1 * t1 - vx * t1;
                            let py_1 = y1 + dy1 * t1 - vy * t1;
                            let pz_1 = z1 + dz1 * t1 - vz * t1;

                            let px_2 = x2 + dx2 * t2 - vx * t2;
                            let py_2 = y2 + dy2 * t2 - vy * t2;
                            let pz_2 = z2 + dz2 * t2 - vz * t2;

                            if px_1 == px_2 && py_1 == py_2 && pz_1 == pz_2 {
                                if vz_0 == 0.0 {
                                    vz_0 = vz;
                                    px_0 = px_1;
                                    py_0 = py_1;
                                    pz_0 = pz_1;
                                    mat += 1;
                                    // eprintln!("{px_1} {py_1} {pz_1}");
                                } else if vz == vz_0 && px_1 == px_0 && py_1 == py_0 && pz_1 == pz_0 {
                                    // eprintln!("{px_1} {py_1} {pz_1}");
                                    mat += 1;
                                } else {
                                    // eprintln!("{px_1} {py_1} {pz_1}");
                                }
                            }
                        }
                    }
                }
            }
            if mat > 2 {
                println!("{mat} - {px_0},{py_0},{pz_0} @ {vx_i},{vy_i},{vz_0} = {}", px_0 + py_0 + pz_0);
            }
            // }
        }
    }

    // println!("{ans}");
}
