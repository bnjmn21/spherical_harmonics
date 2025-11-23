use std::fs::{self, File};

use sphrs::{Coordinates, sh};
use stl_io::{Triangle, Vector};

const L_M_PAIRS: &[(i64, i64)] = &[
    (0, 0),
    (1, 0),
    (2, 0),
    (2, 1),
    (3, 0),
    (3, 2),
    (3, 3),
    (4, 0),
    (4, 3),
    (4, 4),
];
const SUPPORT_BALL_SIZE: f32 = 0.05; // relative to the size of sphere.stl

fn main() {
    let mut stl_file = File::open("sphere.stl").unwrap();
    let unit_sphere = stl_io::create_stl_reader(&mut stl_file)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    drop(stl_file);

    fs::create_dir_all("results").unwrap();

    for (l, m) in L_M_PAIRS {
        println!("computing l={l}, m={m}");
        let sph = unit_sphere.iter().map(|tri| {
            let vertices = tri.vertices.map(|v| {
                let val = sh(*l, *m, &Coordinates::cartesian(v.0[0], v.0[1], v.0[2]))
                    .re
                    .abs();
                Vector(v.0.map(|e| e * val.max(SUPPORT_BALL_SIZE)))
            });
            Triangle {
                normal: compute_normal(&vertices),
                vertices,
            }
        });
        let mut result_file = File::create(format!("results/sph_{l}_{m}.stl")).unwrap();
        stl_io::write_stl(&mut result_file, sph).unwrap();
        drop(result_file);
    }
}

fn compute_normal(vertices: &[Vector<f32>; 3]) -> Vector<f32> {
    // https://math.stackexchange.com/questions/305642/how-to-find-surface-normal-of-a-triangle
    let v = sub_vec(vertices[1], vertices[0]);
    let w = sub_vec(vertices[2], vertices[0]);
    let n = Vector([
        (v[1] * w[2]) - (v[2] * w[1]),
        (v[2] * w[0]) - (v[0] * w[2]),
        (v[0] * w[1]) - (v[1] * w[0]),
    ]);
    let n_len = (n.0[0].powi(2) + n.0[1].powi(2) + n.0[2].powi(2)).sqrt();
    Vector([n.0[0] / n_len, n.0[1] / n_len, n.0[2] / n_len])
}

fn sub_vec(a: Vector<f32>, b: Vector<f32>) -> Vector<f32> {
    Vector([a.0[0] - b.0[0], a.0[1] - b.0[1], a.0[2] - b.0[2]])
}
