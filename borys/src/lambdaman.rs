use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    array_2d::Array2D,
    simulated_annealing::{SearchFor, SimulatedAnnealing},
    TEST_ID,
};

pub fn lambda_solver() {
    let test_id = TEST_ID;

    let input = std::fs::read_to_string(format!("../lambdaman/lambdaman{test_id:02}.in")).unwrap();

    let lines = input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .collect::<Vec<_>>();

    let n = lines.len();
    let mut walls = Array2D::new(false, n, n);
    let mut start = (n, n);
    let mut to_visit = n * n;
    for i in 0..n {
        assert_eq!(n, lines[i].len());
        for j in 0..n {
            if &lines[i][j..j + 1] == "#" {
                walls[i][j] = true;
                to_visit -= 1;
            } else if &lines[i][j..j + 1] == "L" {
                start = (i, j);
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

fn check(
    walls: &Array2D<bool>,
    start: (usize, usize),
    path: &[usize],
    mut to_visit: usize,
    vis: bool,
) -> usize {
    let n = walls.len();
    let mut seen = Array2D::new(false, n, n);
    seen[start.0][start.1] = true;
    to_visit -= 1;
    const DX: [isize; 4] = [0, 1, 0, -1];
    const DY: [isize; 4] = [-1, 0, 1, 0];
    let mut cur_pos = start;
    for i in 0..10_000 {
        let dir = path[i % path.len()];
        let next_pos = (
            cur_pos.0.overflowing_add_signed(DX[dir]).0,
            cur_pos.1.overflowing_add_signed(DY[dir]).0,
        );
        if next_pos.0 >= n || next_pos.1 >= n {
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
