use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::spaceship::{estimate_dist_simple, Point, Precalc};

#[derive(Clone, Copy)]
struct PointWithId {
    id: usize,
    p: Point,
}

// fn dist(p1: Point, p2: Point) -> i64 {
//     let dx = p1.x - p2.x;
//     let dy = p1.y - p2.y;
//     dx.abs() + dy.abs()
// }

pub fn solve_tsp(pts: &[Point], order: Option<Vec<usize>>) -> Vec<usize> {
    let mut a = vec![PointWithId {
        id: usize::MAX,
        p: Point::ZERO,
    }];
    let order = match order {
        Some(order) => order,
        None => (0..pts.len()).collect(),
    };
    for i in order {
        a.push(PointWithId { id: i, p: pts[i] });
    }
    eprintln!("Creating precalc...");
    let precalc = Precalc::new(1000, true);
    eprintln!("Precalc created");
    let dist = |p1: Point, p2: Point| -> i64 { estimate_dist_simple(p1, p2, &precalc) as i64 };
    let mut sum_len = a.windows(2).map(|w| dist(w[0].p, w[1].p)).sum::<i64>();
    eprintln!("Initial sum_len: {}", sum_len);

    let mut rng = ChaCha8Rng::seed_from_u64(787788);
    let start = Instant::now();
    let mut it = 0;
    while start.elapsed().as_secs_f64() < 30.0 {
        it += 1;
        let from = rng.gen_range(1..a.len());
        let to = rng.gen_range(from + 1..a.len() + 1);
        let cur_dist = dist(a[from - 1].p, a[from].p)
            + (if to == a.len() {
                0
            } else {
                dist(a[to - 1].p, a[to].p)
            });
        let new_dist = dist(a[from - 1].p, a[to - 1].p)
            + (if to == a.len() {
                0
            } else {
                dist(a[from].p, a[to].p)
            });
        if new_dist < cur_dist {
            a[from..to].reverse();
            sum_len -= cur_dist - new_dist;
            eprintln!("{it}. New sum len: {sum_len}");
        }
    }

    a[1..].iter().map(|p| p.id).collect()
}
