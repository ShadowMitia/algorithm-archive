use rand::prelude::*;

#[derive(Clone, Copy)]
struct Point2 {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

fn select_array(hutchinson_op: &[Vec<Point3>], probabilities: &[f64]) -> Vec<Point3> {
    let mut rng = rand::thread_rng();
    let mut rnd = rng.gen::<f64>();

    for (i, probability) in probabilities.iter().enumerate() {
        if rnd < *probability {
            return hutchinson_op[i].clone();
        }
        rnd -= probability;
    }

    return vec![];
}

fn chaos_game(
    iters: usize,
    initial_location: Point2,
    hutchinson_op: &[Vec<Point3>],
    probabilities: &[f64],
) -> Vec<Point2> {
    let mut p = Point3 {
        x: initial_location.x,
        y: initial_location.y,
        z: 1.0,
    };
    (0..iters)
        .into_iter()
        .map(|_| {
            let old_point = p;
            let operation = select_array(hutchinson_op, probabilities);
            p.x = operation[0].x * p.x + operation[0].y * p.y + operation[0].z * p.z;
            p.y = operation[1].x * p.x + operation[1].y * p.y + operation[1].z * p.z;
            p.z = operation[2].x * p.x + operation[2].y * p.y + operation[2].z * p.z;
            Point2 {
                x: old_point.x,
                y: old_point.y,
            }
        })
        .collect()
}

fn main() {
    let barnsley_hutchinson = vec![
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.16, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        vec![
            Point3::new(0.85, 0.04, 0.0),
            Point3::new(-0.04, 0.85, 1.60),
            Point3::new(0.0, 0.0, 1.0),
        ],
        vec![
            Point3::new(0.20, -0.26, 0.0),
            Point3::new(0.23, 0.22, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        vec![
            Point3::new(-0.15, 0.25, 0.0),
            Point3::new(0.26, 0.24, 0.44),
            Point3::new(0.0, 0.0, 1.0),
        ],
    ];

    let barnsley_probabilities = vec![0.01, 0.85, 0.07, 0.07];

    let mut out = String::new();

    for point in chaos_game(
        10_000,
        Point2 { x: 0.0, y: 0.0 },
        &barnsley_hutchinson,
        &barnsley_probabilities,
    ) {
        out += format!("{}\t{}\n", point.x, point.y).as_str();
    }

    std::fs::write("./out.dat", out).unwrap();
}
