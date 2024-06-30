use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::alloc::System;
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::default;
use std::io::Write;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::time::Instant;

use crate::local_solver::LocalSolver;
use crate::tsp::solve_tsp;
use crate::{protocol, TEST_ID};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn get_coord(&self, coord: usize) -> i64 {
        match coord {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid coord"),
        }
    }

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
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

pub fn read_input(id: usize) -> Vec<Point> {
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

pub fn read_solution(id: usize) -> Vec<Point> {
    {
        let gena_path = format!("../gena/{:02}.out", id);
        if let Ok(input) = std::fs::read_to_string(gena_path) {
            let input: Vec<_> = input.split_whitespace().collect();
            let input = input[input.len() - 1];
            return input.bytes().map(conv_dir).collect();
        }
    }
    for suffix in ["_borys", ""].iter() {
        let filename = format!("../spaceship/spaceship{:02}{suffix}.out", id);
        eprintln!("Reading from file: {:?}", filename);
        if let Ok(input) = std::fs::read_to_string(filename) {
            return input.trim().bytes().map(conv_dir).collect();
        }
    }
    panic!("No solution found");
}

pub(crate) fn check_solution_and_save(pts: &[Point], solution: &[Point], vis_file: &str) {
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

pub(crate) fn check_solution(pts: &[Point], solution: &[Point]) {
    let mut need_to_visit: HashSet<Point> = pts.iter().cloned().collect();
    eprintln!("Total need visit: {}", need_to_visit.len());
    let mut pos = Point { x: 0, y: 0 };
    let mut velocity = Point { x: 0, y: 0 };
    need_to_visit.remove(&pos);
    for &dir in solution {
        velocity += dir;
        pos += velocity;
        need_to_visit.remove(&pos);
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
        check_solution_and_save(&pts, &solution, &vis_file);
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

    // const BASE_BEAM: usize = 100;
    // let mut cur_sol_len = solve_fixed_perm(&order, Point::ZERO, BASE_BEAM).len();
    // let start = Instant::now();
    // for iter in 1.. {
    //     if iter % 100 == 0 {
    //         eprintln!("Iter: {}", iter);
    //     }
    //     if start.elapsed().as_secs() > 1000 {
    //         break;
    //     }
    //     // let sol = solve_fixed_perm(&order);
    //     let from = rng.gen_range(0..order.len());
    //     let len = rng.gen_range(2..=5);
    //     let to = from + len;
    //     if to > order.len() {
    //         continue;
    //     }
    //     order[from..to].reverse();
    //     // let mut new_order = order.clone();
    //     // let elem = new_order.remove(pos);
    //     // let new_pos = rng.gen_range(0..new_order.len() + 1);
    //     // new_order.insert(new_pos, elem);
    //     let new_sol = solve_fixed_perm(&order, Point::ZERO, BASE_BEAM);
    //     if new_sol.len() < cur_sol_len {
    //         cur_sol_len = new_sol.len();
    //         eprintln!("New sol len: {}", cur_sol_len);
    //         if new_sol.len() < prev_sol.len() {
    //             check_solution(&pts, &new_sol, &vis_file);
    //             save_solution(task_id, &new_sol);
    //         } else {
    //             eprintln!("Not going to save it..");
    //         }
    //     } else {
    //         order[from..to].reverse();
    //     }
    // }
    let precalc = Precalc::new(100, false);
    solve_fixed_perm_precalc(&order, &precalc)
}

#[derive(Clone, Debug)]
struct State {
    pos: Point,
    v_range: Range<Point>,
    time: usize,
}

fn solve_fixed_perm_precalc(pts: &[Point], precalc: &Precalc) -> Vec<Point> {
    let mut states = vec![State {
        pos: Point::ZERO,
        v_range: Point::ZERO..Point { x: 1, y: 1 },
        time: 0,
    }];
    for (i, next_p) in pts.iter().enumerate() {
        let state = states.last().unwrap();
        let need_x = next_p.x - state.pos.x;
        let mut good = 5;
        for time in 0.. {
            let default_xs_range =
                state.v_range.start.x * time as i64..(state.v_range.end.x - 1) * time as i64;
            let x_range = need_x - default_xs_range.end..need_x - default_xs_range.start + 1;
            let default_ys_range =
                state.v_range.start.y * time as i64..(state.v_range.end.y - 1) * time as i64;
            let y_range = next_p.y - default_ys_range.end..next_p.y - default_ys_range.start + 1;
            let new_vx = precalc.get_vs_range(time, x_range);
            let new_vy = precalc.get_vs_range(time, y_range);
            if new_vx.is_empty() || new_vy.is_empty() {
                continue;
            }
            good -= 1;
            if good == 0 {
                states.push(State {
                    pos: *next_p,
                    v_range: Point {
                        x: new_vx.start,
                        y: new_vy.start,
                    }..Point {
                        x: new_vx.end,
                        y: new_vy.end,
                    },
                    time: state.time + time,
                });
                break;
            }
        }
        eprintln!(
            "Iter: {i}/{}, state = {:?}",
            pts.len(),
            states.last().unwrap()
        );
    }
    let mut res = vec![];
    let mut cur_v = states.last().unwrap().v_range.start;
    for w in states.windows(2).rev() {
        let prev = &w[0];
        let cur = &w[1];
        let sol_x = precalc.reconstruct(
            prev.v_range.start.x..prev.v_range.end.x,
            cur.time - prev.time,
            cur.pos.x - prev.pos.x,
            cur_v.x,
        );
        let sol_y = precalc.reconstruct(
            prev.v_range.start.y..prev.v_range.end.y,
            cur.time - prev.time,
            cur.pos.y - prev.pos.y,
            cur_v.y,
        );
        assert_eq!(sol_x.len(), sol_y.len());
        let mut res_part = vec![];
        for (&dx, &dy) in sol_x.iter().zip(sol_y.iter()) {
            res_part.push(Point { x: dx, y: dy });
            cur_v += Point { x: -dx, y: -dy };
        }
        res.extend(res_part.into_iter().rev());
        assert!(prev.v_range.start.x <= cur_v.x && cur_v.x < prev.v_range.end.x);
        assert!(prev.v_range.start.y <= cur_v.y && cur_v.y < prev.v_range.end.y);
    }
    {
        let mut pos = Point::ZERO;
        let mut velocity = Point::ZERO;
        for &dir in res.iter() {
            velocity += dir;
            pos += velocity;
        }
        assert_eq!(pos, *pts.last().unwrap());
    }
    res
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

fn do_test() -> bool {
    // x -> all velocities
    let mut can: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    can.insert(0, vec![3, 4].into_iter().collect());
    for time in 0..10 {
        let mut new_can: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
        for (&x, velocities) in can.iter() {
            for &vel in velocities {
                for vel_delta in -1..=1 {
                    let new_vel = vel + vel_delta;
                    let new_x = x + new_vel;
                    new_can.entry(new_x).or_default().insert(new_vel);
                }
            }
        }
        can = new_can;
        eprintln!("Time: {time}");
        for (&x, velocities) in can.iter() {
            eprintln!("x = {x}: {velocities:?}");
            let min_v = velocities.iter().min().unwrap();
            let max_v = velocities.iter().max().unwrap();
            assert_eq!(max_v - min_v + 1, velocities.len() as i64);
        }
    }
    true
}

fn add_to_range(r: &mut Range<i64>, x: i64) {
    if r.is_empty() {
        *r = x..x + 1;
    } else {
        r.start = r.start.min(x);
        r.end = r.end.max(x + 1);
    }
}

fn join_range(r1: &Range<i64>, r2: &Range<i64>) -> Range<i64> {
    if r1.is_empty() {
        return r2.clone();
    }
    if r2.is_empty() {
        return r1.clone();
    }
    r1.start.min(r2.start)..r1.end.max(r2.end)
}
pub struct Precalc {
    possible_v: Vec<Vec<Range<i64>>>,
    max_xs: Vec<i64>,
    simple: Vec<i64>,
}

impl Precalc {
    pub fn new(max_time: usize, only_simple: bool) -> Self {
        let mut possible_v = vec![vec![]; max_time + 1];
        let mut max_xs = vec![0i64];
        if !only_simple {
            possible_v[0].push(0..1);
            for time in 0..max_time {
                let new_max_x = max_xs[time] + time as i64 + 1;
                possible_v[time + 1] = vec![0..0; new_max_x as usize * 2 + 1];
                max_xs.push(new_max_x);
                for xi in 0..possible_v[time].len() {
                    let x = xi as i64 - max_xs[time];
                    for v in possible_v[time][xi].clone() {
                        for dv in -1..=1 {
                            let new_v = v + dv;
                            let new_x = x + new_v;
                            let new_xi = (new_x + new_max_x) as usize;
                            add_to_range(&mut possible_v[time + 1][new_xi], new_v);
                        }
                    }
                }
            }
        }
        let mut simple = vec![0; max_time * max_time];
        for delta in 0..simple.len() {
            let mut sum = 0;
            for time in 0.. {
                sum += time;
                if sum >= delta {
                    simple[delta] = time as i64;
                    break;
                }
            }
        }
        Self {
            possible_v,
            max_xs,
            simple,
        }
    }

    pub fn get_simple(&self, v: Point) -> usize {
        let vx = v.x.unsigned_abs() as usize;
        let vy = v.y.unsigned_abs() as usize;
        self.simple[vx].max(self.simple[vy]) as usize
    }

    pub fn get_x_range(&self, time: usize) -> Range<i64> {
        let max_x = self.max_xs[time];
        -max_x..max_x + 1
    }

    pub fn get_vs(&self, time: usize, x: i64) -> Range<i64> {
        let max_x = self.max_xs[time];
        if x.abs() > max_x {
            return 0..0;
        }
        let xi = (x + max_x) as usize;
        self.possible_v[time][xi].clone()
    }

    pub fn get_vs_range(&self, time: usize, x_range: Range<i64>) -> Range<i64> {
        let mut res = 0..0;
        // TODO: optimize
        for x in x_range {
            let cur = self.get_vs(time, x);
            res = join_range(&res, &cur);
        }
        res
    }

    pub fn is_possible(&self, time: usize, x: i64, start_v: i64, end_v: i64) -> bool {
        let x = x - time as i64 * start_v;
        let need_v = end_v - start_v;
        let vs = self.get_vs(time, x);
        // eprintln!("Is possible time={time} x={x} start_v={start_v} end_v={end_v} vs={vs:?}");
        vs.start <= need_v && need_v < vs.end
    }

    pub fn is_possible_any_v(&self, time: usize, x: i64, start_v: i64) -> bool {
        let x = x - time as i64 * start_v;
        let vs = self.get_vs(time, x);
        !vs.is_empty()
    }

    pub fn reconstruct(
        &self,
        prev_v_range: Range<i64>,
        time: usize,
        mut x: i64,
        need_v: i64,
    ) -> Vec<i64> {
        eprintln!("Reconstruct: prev_v={prev_v_range:?} time={time} need_x={x} need_v={need_v}");
        for first_v in prev_v_range {
            if self.is_possible(time, x, first_v, need_v) {
                let mut cur_v = first_v;
                let mut res = vec![];
                for t in 0..time {
                    for dv in -1..=1 {
                        let new_v = cur_v + dv;
                        if self.is_possible(time - t - 1, x - new_v, new_v, need_v) {
                            res.push(dv);
                            cur_v = new_v;
                            x -= new_v;
                            break;
                        }
                    }
                    assert_eq!(res.len(), t + 1);
                }
                return res;
            }
        }
        unreachable!();
    }
}

fn do_test2() -> bool {
    let precalc = Precalc::new(10, false);
    for time in 0..=10 {
        eprintln!("Time: {time}");
        let xs = precalc.get_x_range(time);
        for x in xs {
            let vs = precalc.get_vs(time, x);
            eprintln!("x = {x}: {vs:?}");
        }
    }
    true
}

fn do_tsp(test_id: usize, pts: &[Point]) {
    let order_filename = format!("../spaceship/spaceship{:02}_order.txt", test_id);

    let order: Option<Vec<usize>> = if let Ok(input) = std::fs::read_to_string(order_filename) {
        let mut order: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
        order.remove(0);
        Some(order)
    } else {
        None
    };
    let sol = solve_tsp(pts, order);
    let mut f =
        std::fs::File::create(format!("../spaceship/spaceship{:02}_order.txt", test_id)).unwrap();
    writeln!(f, "{}", sol.len()).unwrap();
    for id in sol {
        writeln!(f, "{}", id).unwrap();
    }
}

pub fn estimate_dist(prev: Point, cur: Point, next: Point, precalc: &Precalc) -> usize {
    let av_dist = estimate_dist_simple(prev, cur, precalc) as i64 + 10;
    let velocity = Point {
        x: (cur.x - prev.x) / av_dist,
        y: (cur.y - prev.y) / av_dist,
    };
    let need_x = next.x - cur.x;
    let need_y = next.y - cur.y;
    for time in 0.. {
        if precalc.is_possible_any_v(time, need_x, velocity.x)
            && precalc.is_possible_any_v(time, need_y, velocity.y)
        {
            return time;
        }
    }
    unreachable!()
}

pub fn estimate_dist_simple(cur: Point, next: Point, precalc: &Precalc) -> usize {
    let need_x = next.x - cur.x;
    let need_y = next.y - cur.y;
    precalc.get_simple(Point {
        x: need_x,
        y: need_y,
    })
    // for time in 0.. {
    //     if precalc.is_possible_any_v(time, need_x, 0) && precalc.is_possible_any_v(time, need_y, 0)
    //     {
    //         assert!(
    //             time == precalc.get_simple(Point {
    //                 x: need_x,
    //                 y: need_y
    //             })
    //         );
    //         return time;
    //     }
    // }
    // unreachable!()
}

fn calc_stats_old(pts: &[Point], sol: &[Point]) {
    let precalc = Precalc::new(100, false);

    let mut pos = Point::ZERO;
    let mut velocity = Point::ZERO;
    let mut need_to_visit: HashSet<Point> = pts.iter().cloned().collect();
    let mut ordered_pts = vec![pos];
    let mut dists = vec![];
    let mut prev_step = 0;
    for (step, &dir) in sol.iter().enumerate() {
        velocity += dir;
        pos += velocity;
        if need_to_visit.remove(&pos) {
            ordered_pts.push(pos);
            dists.push(step - prev_step);
            prev_step = step;
        }
    }
    let mut sum_est_diff = 0;
    for i in 0..dists.len() - 1 {
        let p1 = ordered_pts[i];
        let p2 = ordered_pts[i + 1];
        let p3 = ordered_pts[i + 2];
        let d23 = dists[i + 1];
        let est = estimate_dist(p1, p2, p3, &precalc);
        sum_est_diff += (d23 as i64 - est as i64).abs();
        eprintln!(
            "DX1={}, DX2={}, DY1={}, DY2={}, d23={d23}, est={est}",
            p2.x - p1.x,
            p3.x - p2.x,
            p2.y - p1.y,
            p3.y - p2.y,
        );
    }
    eprintln!(
        "AV ESTIMATE DIFF: {}",
        sum_est_diff as f64 / (dists.len() - 1) as f64
    );
}

fn calc_stats(pts: &[Point], sol: &[Point]) {
    let local_solver = LocalSolver::new();

    eprintln!("Sol len: {}", sol.len());

    let mut pos = Point::ZERO;
    let mut velocity = Point::ZERO;
    let mut need_to_visit: HashMap<Point, usize> = pts
        .iter()
        .cloned()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect();
    let mut ordered_pts = vec![pos];
    let mut dists = vec![];
    let mut prev_step = 0;
    let mut velocities = vec![Point::ZERO];
    let mut real_order = vec![];
    for (step, &dir) in sol.iter().enumerate() {
        velocity += dir;
        pos += velocity;
        if let Some(idx) = need_to_visit.remove(&pos) {
            real_order.push(idx);
            velocities.push(velocity);

            ordered_pts.push(pos);
            dists.push(step - prev_step);
            prev_step = step;
        }
    }
    const LIMIT: i64 = 3;
    let mut i = 0;
    while i < ordered_pts.len() {
        let good_sizes: Vec<_> = (3..100)
            .into_par_iter()
            .filter_map(|sz| {
                if i + sz > ordered_pts.len() {
                    return None;
                }
                let mut pts: Vec<_> = ordered_pts[i..i + sz].to_vec();
                pts[1..sz - 1].reverse();
                let v_start = velocities[i];
                let v_end = velocities[i + sz - 1];

                let my_cost = local_solver.calc_best(&pts, v_start, v_end, LIMIT);
                let real_cost = dists[i..i + sz - 1].iter().sum::<usize>();
                if my_cost < real_cost {
                    Some(sz)
                } else {
                    None
                }
            })
            .collect();
        if !good_sizes.is_empty() {
            let sz = good_sizes[0];
            eprintln!("Optimized {i}. sz = {sz}");
            real_order[i..i + sz - 2].reverse();
            i += sz - 1;
        }
        i += 1;
    }

    let mut f = std::fs::File::create("../spaceship/spaceship19_order.txt").unwrap();
    writeln!(f, "{}", real_order.len()).unwrap();
    for id in real_order {
        writeln!(f, "{}", id).unwrap();
    }
}

pub async fn spaceship_solve() -> bool {
    eprintln!("Hello");

    let task_id = TEST_ID;

    // if do_test2() {
    //     return true;
    // }

    for task_id in task_id..=task_id {
        // let vis_file = format!("../spaceship/spaceship{:02}.viz", task_id);
        let vis_file = "spaceship00.viz";

        eprintln!("Task: {}", task_id);
        let pts = read_input(task_id);

        // eprintln!("Points: {:?}", pts);
        // for _ in 0..100 {
        // do_tsp(task_id, &pts);
        // }
        let solution = read_solution(task_id);
        calc_stats(&pts, &solution);

        // let new_solution = solve(&pts, &solution, task_id, &vis_file);
        // check_solution(&pts, &solution);
        // save_solution(task_id, &new_solution);

        // writeln!(my_score_f, "{task_id}: {}", new_solution.len()).unwrap();
        // my_score_f.flush().unwrap();

        // send_solution(task_id, &solution).await;

        // eprintln!("Need to visit {}, sol len: {}", pts.len(), solution.len());
        // eprintln!("Solution: {:?}", solution);
        // check_solution(&pts, &solution, &vis_file);
    }

    // run_python_viz(0);
    true
}

#[test]
fn precalc_test() {
    let precalc = Precalc::new(30, false);
    let dx = 1;
    let time = 6;
    let start_v = -6..7;

    // precalc.get_vs_range(time, x_range)
}
