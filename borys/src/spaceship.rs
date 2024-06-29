use std::collections::HashSet;
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
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
    let filename = format!("../spaceship/spaceship{:02}.out", id);
    eprintln!("Reading from file: {:?}", filename);
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();
    input.as_bytes().iter().map(|&c| conv_dir(c)).collect()
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
        let removed = need_to_visit.remove(&pos);
        // eprintln!(
        //     "{move_id}. Current pos: {:?}, vel: {:?}, removed: {removed}",
        //     pos, velocity
        // );
        writeln!(f, "{} {}", pos.x, pos.y).unwrap();
    }
    assert!(need_to_visit.is_empty())
}

pub fn solve_spaceship() -> bool {
    eprintln!("Hello");

    let task_id = 18;
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
    true
}
