use crate::{
    array_2d::Array2D,
    spaceship::{Point, Precalc},
};

pub struct LocalSolver {
    precalc: Precalc,
}

impl LocalSolver {
    pub fn new() -> Self {
        Self {
            precalc: Precalc::new(200, false),
        }
    }

    fn need_time(&self, shift: Point, v_start: Point, v_end: Point) -> usize {
        let at_least_time = (v_end.x - v_start.x).abs().max((v_end.y - v_start.y).abs()) as usize;
        for time in at_least_time.. {
            if self.precalc.is_possible(time, shift.x, v_start.x, v_end.x)
                && self.precalc.is_possible(time, shift.y, v_start.y, v_end.y)
            {
                return time;
            }
        }
        unreachable!()
    }

    pub fn calc_best(&self, pts: &[Point], start_v: Point, end_v: Point, limit: i64) -> usize {
        if start_v.x.abs() > limit || start_v.y.abs() > limit {
            return usize::MAX;
        }
        if end_v.x.abs() > limit || end_v.y.abs() > limit {
            return usize::MAX;
        }

        let mut dp = Array2D::new(usize::MAX, limit as usize * 2 + 1, limit as usize * 2 + 1);
        let get = |dp: &Array2D<usize>, v: Point| -> usize {
            let x = (v.x + limit) as usize;
            let y = (v.y + limit) as usize;
            dp[x][y]
        };
        let set = |dp: &mut Array2D<usize>, v: Point, value: usize| {
            let x = (v.x + limit) as usize;
            let y = (v.y + limit) as usize;
            dp[x][y] = value
        };
        set(&mut dp, start_v, 0);
        let max_delta = 7;
        for pts_i in 0..pts.len() - 1 {
            let cur = pts[pts_i];
            let next = pts[pts_i + 1];
            let shift = Point {
                x: next.x - cur.x,
                y: next.y - cur.y,
            };
            let mut ndp = Array2D::new(usize::MAX, limit as usize * 2 + 1, limit as usize * 2 + 1);
            let mut bests = vec![];
            for x in -limit..=limit {
                for y in -limit..=limit {
                    let v = Point { x, y };
                    let cur_val = get(&mut dp, v);
                    if cur_val == usize::MAX {
                        continue;
                    }
                    bests.push((cur_val, x, y));
                }
            }
            bests.sort();
            bests.truncate(15);
            for (cur_val, x, y) in bests {
                let v = Point { x, y };
                for nx in -limit.max(x - max_delta)..=limit.min(y + max_delta) {
                    for ny in -limit.max(y - max_delta)..=limit.min(y + max_delta) {
                        let nv = Point { x: nx, y: ny };
                        let extra_time = self.need_time(shift, v, nv);
                        let new_val = cur_val + extra_time;
                        let set_val = get(&mut ndp, nv).min(new_val);
                        set(&mut ndp, nv, set_val);
                    }
                }
            }

            dp = ndp;
        }
        get(&mut dp, end_v)
    }
}
