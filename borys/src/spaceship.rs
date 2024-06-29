use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::alloc::System;
use std::collections::HashSet;
use std::io::Write;
use std::rc::Rc;
use std::time::Instant;

use crate::protocol;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };

    pub fn get_coord(&self, coord: usize) -> i64 {
        match coord {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid coord"),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

fn read_input(id: usize) -> Vec<Point> {
    let filename = format!("../spaceship/spaceship{:02}.in", id);
    let input = std::fs::read_to_string(filename).unwrap();
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let x = parts.next()?.parse().ok()?;
            let y = parts.next()?.parse().ok()?;
            Some(Point { x, y })
        })
        .collect()
}

fn conv_dir(c: u8) -> Point {
    let digit = (c - b'1') as usize;
    assert!(digit < 9);
    let x = (digit % 3) as i64 - 1;
    let y = (digit / 3) as i64 - 1;
    Point { x, y }
}

fn read_solution(id: usize) -> Vec<Point> {
    for suffix in ["_borys", ""].iter() {
        let filename = format!("../spaceship/spaceship{:02}{suffix}.out", id);
        eprintln!("Reading from file: {:?}", filename);
        if let Ok(input) = std::fs::read_to_string(filename) {
            return input.trim().bytes().map(conv_dir).collect();
        }
    }
    panic!("No solution found");
}

fn check_solution(pts: &[Point], solution: &[Point], vis_file: &str) {
    let mut need_to_visit: HashSet<Point> = pts.iter().cloned().collect();
    let mut f = std::fs::File::create(vis_file).unwrap();
    {
        writeln!(f, "{}", pts.len()).unwrap();
        for &pt in pts {
            writeln!(f, "{} {}", pt.x, pt.y).unwrap();
        }
        writeln!(f, "{}", solution.len() + 1).unwrap();
    }
    eprintln!("Total need visit: {}", need_to_visit.len());
    let mut pos = Point { x: 0, y: 0 };
    let mut velocity = Point { x: 0, y: 0 };
    need_to_visit.remove(&pos);
    {
        writeln!(f, "{} {}", pos.x, pos.y).unwrap();
    }
    let mut move_id = 0;
    for &dir in solution {
        move_id += 1;
        velocity += dir;
        pos += velocity;
        need_to_visit.remove(&pos);
        writeln!(f, "{} {}", pos.x, pos.y).unwrap();
    }
    assert!(need_to_visit.is_empty())
}

pub fn spaceship_draw() {
    for task_id in 1..=25 {
        eprintln!("Task: {}", task_id);
        let pts = read_input(task_id);
        // eprintln!("Points: {:?}", pts);
        let solution = read_solution(task_id);

        eprintln!("Need to visit {}, sol len: {}", pts.len(), solution.len());
        // eprintln!("Solution: {:?}", solution);
        let vis_file = format!("../spaceship/spaceship{:02}.viz", task_id);
        check_solution(&pts, &solution, &vis_file);
    }
}

const MAX_VELOCITY: i64 = 20;

fn solve1d(time: usize, mut vel: i64, mut pos: i64) -> Option<Vec<i64>> {
    pos -= vel * time as i64;
    let mut deltas = vec![];
    for cur_time in 0..time {
        let coef = (time - cur_time) as i64;
        let possible_pos = [(pos + coef).abs(), pos.abs(), (pos - coef).abs()];
        let mut best_delta_idx = 3;
        for i in 0..3 {
            let next_velocity = vel + i as i64 - 1;
            if next_velocity.abs() > MAX_VELOCITY {
                continue;
            }
            if best_delta_idx == 3 || possible_pos[i] < possible_pos[best_delta_idx] {
                best_delta_idx = i;
            }
        }
        let best_delta = best_delta_idx as i64 - 1;
        deltas.push(best_delta);
        pos -= coef * best_delta;
        vel += best_delta;
    }
    if pos == 0 {
        Some(deltas)
    } else {
        None
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Solution {
    sol_len: usize,
    velocity: Point,
    pos: Point,
    time: usize,
    prev_sol: Option<Rc<Solution>>,
}

fn reconstruct_solution(sol: Rc<Solution>, start: Point) -> Vec<Point> {
    let mut all_parts = vec![sol.clone()];
    let mut cur_sol = sol;
    while let Some(prev_sol) = cur_sol.prev_sol.clone() {
        all_parts.push(prev_sol.clone());
        cur_sol = prev_sol;
    }
    all_parts.reverse();
    let mut res = vec![];
    let mut pos = start;
    let mut velocity = Point::ZERO;
    for w in all_parts.windows(2) {
        let prev = &w[0];
        let cur = &w[1];
        assert_eq!(prev, cur.prev_sol.as_ref().unwrap());
        assert_eq!(prev.pos, cur.prev_sol.as_ref().unwrap().pos);
        let sol_x = solve1d(cur.time, prev.velocity.x, cur.pos.x - prev.pos.x).unwrap();
        let sol_y = solve1d(cur.time, prev.velocity.y, cur.pos.y - prev.pos.y).unwrap();
        assert_eq!(sol_x.len(), sol_y.len());
        for (&dx, &dy) in sol_x.iter().zip(sol_y.iter()) {
            res.push(Point { x: dx, y: dy });
            velocity += Point { x: dx, y: dy };
            pos += velocity;
        }
        assert_eq!(pos, cur.pos);
        assert_eq!(velocity, cur.velocity);
    }
    res
}

fn solve_fixed_perm(pts: &[Point], start: Point, beam_width: usize) -> Vec<Point> {
    // eprintln!("Fixed perm: {}", pts.len());

    let mut solutions = vec![Rc::new(Solution {
        sol_len: 0,
        velocity: Point::ZERO,
        pos: start,
        prev_sol: None,
        time: 0,
    })];

    for (iter, need_p) in pts.iter().enumerate() {
        const MAX_TIME: usize = 5000;

        let mut new_solutions = vec![];
        for prev_sol in solutions.iter() {
            let mut good = 10;
            for time in 0..MAX_TIME {
                let mut pos = prev_sol.pos;
                let mut velocity = prev_sol.velocity;

                let sol_x = solve1d(time, velocity.x, need_p.x - pos.x);
                let sol_y = solve1d(time, velocity.y, need_p.y - pos.y);
                if let (Some(sol_x), Some(sol_y)) = (sol_x, sol_y) {
                    assert_eq!(sol_x.len(), sol_y.len());
                    for (&dx, &dy) in sol_x.iter().zip(sol_y.iter()) {
                        velocity += Point { x: dx, y: dy };
                        pos += velocity;
                    }
                    assert_eq!(pos, *need_p);

                    let new_sol = Rc::new(Solution {
                        sol_len: prev_sol.sol_len + sol_x.len(),
                        velocity,
                        pos,
                        prev_sol: Some(prev_sol.clone()),
                        time,
                    });
                    new_solutions.push(new_sol);

                    good -= 1;
                    if good == 0 {
                        break;
                    }
                }
            }
        }
        solutions = new_solutions;
        solutions.sort_by_key(|sol| sol.sol_len);
        solutions.truncate(beam_width);
        // eprintln!(
        //     "Iter: {iter}/{}, sum = {}, time = {}, velocity = {:?}",
        //     pts.len(),
        //     solutions[0].sol_len,
        //     solutions[0].time,
        //     solutions[0].velocity
        // );
    }

    let solution = reconstruct_solution(solutions[0].clone(), start);

    // eprintln!("Full solution len: {}", solution.len());

    solution
}

fn convert_solution(sol: &[Point]) -> String {
    let mut s = vec![];
    for &dir in sol {
        let digit = (dir.y + 1) * 3 + (dir.x + 1) + 1;
        s.push(digit as u8 + b'0');
    }
    std::str::from_utf8(&s).unwrap().to_string()
}

fn save_solution(test_id: usize, sol: &[Point]) {
    let filename = format!("../spaceship/spaceship{:02}_borys.out", test_id);
    std::fs::write(filename, convert_solution(sol)).unwrap();
}

fn solve(pts: &[Point], prev_sol: &[Point], task_id: usize, vis_file: &str) -> Vec<Point> {
    let mut not_seen: HashSet<Point> = pts.iter().cloned().collect();
    let mut pos = Point::ZERO;
    let mut order = vec![];
    if not_seen.remove(&pos) {
        order.push(pos);
    }
    let mut velocity = Point::ZERO;
    for &dir in prev_sol {
        velocity += dir;
        pos += velocity;
        if not_seen.remove(&pos) {
            order.push(pos);
        }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(787788);

    // const MID_LEN: usize = 4;

    // for i in 0..order.len() - MID_LEN {
    //     const BEAM: usize = 5;
    //     eprintln!("Check: {i}");
    //     for _it in 0..10 {
    //         let mut mid: Vec<Point> = order[i..i + MID_LEN].iter().cloned().collect();
    //         let cur = solve_fixed_perm(&mid, order[i], BEAM).len();
    //         mid[1..MID_LEN - 1].shuffle(&mut rng);
    //         let new = solve_fixed_perm(&mid, order[i], BEAM).len();
    //         if new < cur {
    //             eprintln!("WOW!");
    //             for j in 0..MID_LEN {
    //                 order[i + j] = mid[j];
    //             }
    //         }
    //     }
    // }

    const BASE_BEAM: usize = 100;
    let mut cur_sol_len = solve_fixed_perm(&order, Point::ZERO, BASE_BEAM).len();
    let start = Instant::now();
    for iter in 1.. {
        if iter % 100 == 0 {
            eprintln!("Iter: {}", iter);
        }
        if start.elapsed().as_secs() > 1000 {
            break;
        }
        // let sol = solve_fixed_perm(&order);
        let from = rng.gen_range(0..order.len());
        let len = rng.gen_range(2..=5);
        let to = from + len;
        if to > order.len() {
            continue;
        }
        order[from..to].reverse();
        // let mut new_order = order.clone();
        // let elem = new_order.remove(pos);
        // let new_pos = rng.gen_range(0..new_order.len() + 1);
        // new_order.insert(new_pos, elem);
        let new_sol = solve_fixed_perm(&order, Point::ZERO, BASE_BEAM);
        if new_sol.len() < cur_sol_len {
            cur_sol_len = new_sol.len();
            eprintln!("New sol len: {}", cur_sol_len);
            if new_sol.len() < prev_sol.len() {
                check_solution(&pts, &new_sol, &vis_file);
                save_solution(task_id, &new_sol);
            } else {
                eprintln!("Not going to save it..");
            }
        } else {
            order[from..to].reverse();
        }
    }
    solve_fixed_perm(&order, Point::ZERO, BASE_BEAM)
}

fn run_python_viz(test_id: usize) {
    std::process::Command::new("python3")
        .arg("../spaceship/draw_sol.py")
        .arg(test_id.to_string())
        .output()
        .unwrap();
}

async fn send_solution(test_id: usize, sol: &[Point]) {
    let sol_str = convert_solution(sol);
    let msg = format!("solve spaceship{test_id} {sol_str}");
    eprintln!("MESSAGE:\n{msg}");
    protocol::send_msg(&msg).await.unwrap();
}

pub async fn spaceship_solve() -> bool {
    eprintln!("Hello");

    let task_id = 18;

    let mut my_score_f = std::fs::File::create("my_score.txt").unwrap();
    for task_id in task_id..=task_id {
        // let vis_file = format!("../spaceship/spaceship{:02}.viz", task_id);
        let vis_file = "spaceship00.viz";

        eprintln!("Task: {}", task_id);
        let pts = read_input(task_id);

        eprintln!("Points: {:?}", pts);
        let solution = read_solution(task_id);

        let new_solution = solve(&pts, &solution, task_id, &vis_file);
        // check_solution(&pts, &new_solution, &vis_file);
        // save_solution(task_id, &new_solution);

        writeln!(my_score_f, "{task_id}: {}", new_solution.len()).unwrap();
        my_score_f.flush().unwrap();

        // send_solution(task_id, &new_solution).await;

        // eprintln!("Need to visit {}, sol len: {}", pts.len(), solution.len());
        // eprintln!("Solution: {:?}", solution);
        // check_solution(&pts, &solution, &vis_file);
    }

    // run_python_viz(0);
    true
}
