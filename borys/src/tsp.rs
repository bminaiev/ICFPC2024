use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    simulated_annealing::{SearchFor, SimulatedAnnealing},
    spaceship::{estimate_dist_simple, Point, Precalc},
};

#[derive(Clone, Copy)]
struct PointWithId {
    id: usize,
    p: Point,
}

fn dist_simple(p1: Point, p2: Point) -> i64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx.abs() + dy.abs()
}

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
    let precalc = Precalc::new(2000, true);
    eprintln!("Precalc created");
    let dist = |p1: Point, p2: Point| -> i64 { estimate_dist_simple(p1, p2, &precalc) as i64 };
    let dist3 = |p1: Point, p2: Point, p3: Point| -> f64 {
        let est1 = estimate_dist_simple(p1, p2, &precalc) as f64;
        let est2 = estimate_dist_simple(p2, p3, &precalc) as f64;
        let scal_mul = (p3.x - p2.x) * (p2.x - p1.x) + (p3.y - p2.y) * (p2.y - p1.y);
        let d1 = (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt();
        let d2 = (((p3.x - p2.x).pow(2) + (p3.y - p2.y).pow(2)) as f64).sqrt();
        if d1 * d2 == 0.0 {
            return est1 + est2;
        }
        let angle = scal_mul as f64 / (d1 * d2);
        assert!(
            (-1.1..=1.1).contains(&angle),
            "Angle: {}, d1={d1}, d2={d2}",
            angle
        );
        let coef = 2.0 - angle;
        (est1 + est2) * coef
    };
    let mut position = vec![0; pts.len()];
    for i in 1..a.len() {
        position[a[i].id] = i;
    }

    let mut rng = ChaCha8Rng::seed_from_u64(787788);
    eprintln!("Calculating closest...");
    const NEIGHBOURS: usize = 100;
    let mut at_most_dist = 0;
    for _it in 0..50 {
        let me = rng.gen_range(0..pts.len());
        let mut dists: Vec<_> = (0..pts.len())
            .map(|other| dist_simple(pts[me], pts[other]))
            .collect();
        dists.sort_unstable();
        at_most_dist = at_most_dist.max(dists[NEIGHBOURS]);
    }
    let closest: Vec<_> = (0..pts.len())
        .into_par_iter()
        .map(|me| {
            let mut neighbors: Vec<_> = a
                .iter()
                .filter(|p| dist_simple(pts[me], p.p) <= at_most_dist)
                .collect();
            neighbors.sort_by_key(|&other| {
                if other.id == me || other.id == usize::MAX {
                    i64::MAX
                } else {
                    dist_simple(pts[me], other.p)
                }
            });
            neighbors.truncate(30);
            neighbors.into_iter().map(|p| p.id).collect::<Vec<_>>()
        })
        .collect();
    eprintln!("Closest calculated");
    let mut sum_len = a
        .windows(3)
        .map(|w| dist3(w[0].p, w[1].p, w[2].p))
        .sum::<f64>();
    eprintln!("Initial sum_len: {}", sum_len);

    // let start = Instant::now();
    let mut it = 0;
    let mut sa =
        SimulatedAnnealing::new(600.0, SearchFor::MinimumScore, 100.01, 0.01, sum_len as f64);
    while sa.should_continue() {
        it += 1;
        let from = rng.gen_range(2..a.len() - 2);
        let from_id = a[from].id;
        let neis = &closest[from_id];
        let to_id = neis[rng.gen_range(0..neis.len())];
        let to = position[to_id];
        // let to = rng.gen_range(from + 1..a.len() - 1);
        if from + 3 >= to || to + 3 >= a.len() {
            // TODO: fix it.
            continue;
        }
        let positions = [from - 2, from - 1, to - 2, to - 1];
        let cur_dist: f64 = positions
            .iter()
            .map(|&i| dist3(a[i].p, a[i + 1].p, a[i + 2].p))
            .sum();
        a.swap(from, to - 1);
        a.swap(from + 1, to - 2);
        let new_dist: f64 = positions
            .iter()
            .map(|&i| dist3(a[i].p, a[i + 1].p, a[i + 2].p))
            .sum();
        a.swap(from, to - 1);
        a.swap(from + 1, to - 2);
        let new_score = sum_len - cur_dist + new_dist;
        if sa.should_go(new_score) {
            a[from..to].reverse();
            for pos in from..to {
                position[a[pos].id] = pos;
            }
            // let recalculated = a
            //     .windows(3)
            //     .map(|w| dist3(w[0].p, w[1].p, w[2].p))
            //     .sum::<f64>();
            // assert!((recalculated - new_score).abs() <= 0.1);
            sum_len = new_score;
        }
    }

    a[1..].iter().map(|p| p.id).collect()
}
