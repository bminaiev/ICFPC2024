use std::{
    collections::btree_map::Range,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
    time::Instant,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    array_2d::Array2D,
    simulated_annealing::{SearchFor, SimulatedAnnealing},
    TEST_ID,
};

pub struct Task {
    pub walls: Array2D<bool>,
    pub start: (usize, usize),
}

pub fn read_task(test_id: usize) -> Task {
    let input = std::fs::read_to_string(format!("../lambdaman/lambdaman{test_id:02}.in")).unwrap();

    let lines = input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .collect::<Vec<_>>();

    let n = lines.len();
    let m = lines[0].len();
    let mut walls = Array2D::new(false, n, m);
    let mut start = (n, n);
    for i in 0..n {
        // assert_eq!(n, lines[i].len());
        for j in 0..m {
            if &lines[i][j..j + 1] == "#" {
                walls[i][j] = true;
            } else if &lines[i][j..j + 1] == "L" {
                start = (i, j);
            }
        }
    }
    assert_ne!(start, (n, n));
    Task { walls, start }
}

pub fn lambda_solver() {
    let test_id = TEST_ID;

    let Task { walls, start } = read_task(test_id);
    let n = walls.len();
    let mut to_visit = 0;
    for i in 0..n {
        for j in 0..n {
            if !walls[i][j] {
                to_visit += 1;
            }
        }
    }
    assert_ne!(start, (n, n));
    eprintln!("N = {}. To visit: {}", n, to_visit);

    let mut rng = ChaCha8Rng::seed_from_u64(787788);
    let mut path = vec![0; 50];
    for i in 0..path.len() {
        path[i] = rng.gen_range(0..4);
    }
    let mut score = check(&walls, start, &path, to_visit, false);
    eprintln!("Score = {}", score);
    let mut sa = SimulatedAnnealing::new(10.0, SearchFor::MinimumScore, 10.0, 0.1, score as f64);
    while score != 0 && sa.should_continue() {
        let pos = rng.gen_range(0..path.len());
        let old = path[pos];
        path[pos] = rng.gen_range(0..4);
        let new_score = check(&walls, start, &path, to_visit, false);
        if !sa.should_go(new_score as f64) {
            path[pos] = old;
        } else {
            score = new_score;
        }
    }
    eprintln!("Path: {:?}", path);
    check(&walls, start, &path, to_visit, true);
}

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [-1, 0, 1, 0];

pub const DEFAULT_STEPS_LIMIT: usize = 1_000_000;

pub fn eval(
    walls: &Array2D<bool>,
    start: (usize, usize),
    rng: &mut VerySimpleRng,
    steps: usize,
) -> Array2D<bool> {
    let n = walls.len();
    let m = walls[0].len();
    let mut seen = Array2D::new(false, n, m);
    seen[start.0][start.1] = true;
    let mut cur_pos = start;
    // let subpath = [
    //     [0, 0, 1, 1],
    //     [0, 0, 3, 3],
    //     [1, 1, 0, 0],
    //     [1, 1, 2, 2],
    //     [2, 2, 1, 1],
    //     [2, 2, 3, 3],
    //     [3, 3, 2, 2],
    //     [3, 3, 0, 0],
    // ];
    // let subpath = [[0, 0], [1, 1], [2, 2], [3, 3]];
    let subpath = [[0], [1], [2], [3]];
    let mut prev_dir: &[usize] = &[];
    for i in 0..steps {
        if prev_dir.is_empty() {
            prev_dir = &subpath[rng.gen_range(0..subpath.len() as u64)];
        }
        let dir = prev_dir[0];
        prev_dir = &prev_dir[1..];
        let next_pos = (
            cur_pos.0.overflowing_add_signed(DX[dir]).0,
            cur_pos.1.overflowing_add_signed(DY[dir]).0,
        );
        if next_pos.0 >= n || next_pos.1 >= m {
            continue;
        }
        if walls[next_pos.0][next_pos.1] {
            continue;
        }
        cur_pos = next_pos;
        seen[cur_pos.0][cur_pos.1] = true;
    }
    seen
}

fn count_visited(visited: &Array2D<bool>) -> usize {
    let mut res = 0;
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] {
                res += 1;
            }
        }
    }
    res
}

pub struct VerySimpleRng {
    state: u64,
    mult: u64,
    modulo: u64,
    // real_rng: ChaCha8Rng,
}

impl VerySimpleRng {
    pub fn seed_from_u64(seed: u64) -> Self {
        let mut real_rng = ChaCha8Rng::seed_from_u64(seed);
        let seed = real_rng.gen_range(1..2147483647);
        let mult = real_rng.gen_range(1..2147483647);
        Self {
            state: seed,
            mult,
            modulo: 2147483647,
        }
    }

    pub fn gen_range(&mut self, r: std::ops::Range<u64>) -> usize {
        self.state = self.state.wrapping_mul(self.mult).wrapping_rem(self.modulo);
        let from = r.start;
        let to = r.end;
        (from + self.state % (to - from)) as usize
        // self.real_rng.gen_range(r) as usize
    }
}

pub fn find_good_seed(walls: &Array2D<bool>, start: (usize, usize)) -> u64 {
    let need_to_visit = walls.len() * walls[0].len() - count_visited(walls);
    const MAX_SEED: u64 = 1_000_000;
    let max_found = Arc::new(AtomicUsize::new(0));
    let start_time = Instant::now();
    let mut visited_per_seed: Vec<_> = (0..MAX_SEED)
        .into_par_iter()
        .map(|seed| {
            let cur_max_found = max_found.load(std::sync::atomic::Ordering::Relaxed);
            {
                if cur_max_found >= need_to_visit {
                    return (0, seed);
                }
            }
            let vis1m = count_visited(&eval(
                walls,
                start,
                &mut VerySimpleRng::seed_from_u64(seed),
                1_000_000,
            ));
            if vis1m > cur_max_found {
                max_found.store(vis1m, std::sync::atomic::Ordering::Relaxed);
            }
            if seed % 1000 == 0 {
                eprintln!(
                    "Seed = {}. Visited 1m = {}/{}. Max: {}",
                    seed, vis1m, need_to_visit, cur_max_found
                );
            }
            (vis1m, seed)
        })
        .collect();
    eprintln!("Time: {:?}", start_time.elapsed());
    visited_per_seed.sort();
    visited_per_seed.reverse();
    let best = visited_per_seed[0];
    eprintln!(
        "Best seed: {}. Visited 1m = {}/{}",
        best.1, best.0, need_to_visit
    );
    best.1
}

fn check(
    walls: &Array2D<bool>,
    start: (usize, usize),
    path: &[usize],
    mut to_visit: usize,
    vis: bool,
) -> usize {
    let n = walls.len();
    let m = walls[0].len();
    let mut seen = Array2D::new(false, n, m);
    seen[start.0][start.1] = true;
    to_visit -= 1;
    let mut cur_pos = start;
    for i in 0..10_000 {
        let dir = path[i % path.len()];
        let next_pos = (
            cur_pos.0.overflowing_add_signed(DX[dir]).0,
            cur_pos.1.overflowing_add_signed(DY[dir]).0,
        );
        if next_pos.0 >= n || next_pos.1 >= m {
            continue;
        }
        if walls[next_pos.0][next_pos.1] {
            continue;
        }
        cur_pos = next_pos;
        if !seen[cur_pos.0][cur_pos.1] {
            seen[cur_pos.0][cur_pos.1] = true;
            to_visit -= 1;
        }
        if to_visit == 0 {
            return 0;
        }
    }
    if vis {
        for i in 0..seen.len() {
            for j in 0..seen[i].len() {
                if walls[i][j] {
                    eprint!("#");
                } else if start == (i, j) {
                    eprint!("L");
                } else if seen[i][j] {
                    eprint!(".");
                } else {
                    eprint!(" ");
                }
            }
            eprintln!()
        }
    }
    to_visit
}
