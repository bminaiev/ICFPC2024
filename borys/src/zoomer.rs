use egui::{Pos2, Vec2};

use crate::spaceship::Point;

pub const SCREEN_W: f32 = 3500.0;
pub const SCREEN_H: f32 = 2000.0;
pub const PIXELS_PER_POINT: f32 = 1.5;
const DEFAULT_ZOOM_FRAC: f32 = 300.0;

pub struct Zoomer {
    zoom: f32,
    shift: Pos2,
}

impl Zoomer {
    pub fn new(pts: &[Point]) -> Self {
        let min_x = pts.iter().map(|p| p.x).min().unwrap();
        let max_x = pts.iter().map(|p| p.x).max().unwrap();
        let min_y = pts.iter().map(|p| p.y).min().unwrap();
        let max_y = pts.iter().map(|p| p.y).max().unwrap();

        let screen_w = SCREEN_W / PIXELS_PER_POINT;
        let screen_h = SCREEN_H / PIXELS_PER_POINT;

        let need_zoom_x = screen_w / (max_x - min_x) as f32;
        let need_zoom_y = screen_h / (max_y - min_y) as f32;
        let zoom = need_zoom_x.min(need_zoom_y) * 0.9;

        let mid_x = (max_x + min_x) as f32 / 2.0;
        let mid_y = -(max_y + min_y) as f32 / 2.0;

        // (mid_x, mid_y) should be in the center of the screen
        let shift = Pos2::new(screen_w / 2.0 - mid_x * zoom, screen_h / 2.0 - mid_y * zoom);
        Self { zoom, shift }
    }

    pub fn convert(&self, p: Point) -> Pos2 {
        Pos2::new(
            p.x as f32 * self.zoom + self.shift.x,
            -p.y as f32 * self.zoom + self.shift.y,
        )
    }

    pub fn convert_back(&self, pos: Pos2) -> Point {
        Point::new(
            ((pos.x - self.shift.x) / self.zoom) as i64,
            (-(pos.y - self.shift.y) / self.zoom) as i64,
        )
    }

    pub fn ensure_fits(&self, pts: &[Point]) {
        for p in pts {
            let pos = self.convert(*p);
            assert!(pos.x >= 0.0 && pos.x <= SCREEN_W);
            assert!(pos.y >= 0.0 && pos.y <= SCREEN_H);
        }
    }

    pub fn update_scroll(&mut self, mouse_pos: Pos2, scroll_delta: f32) {
        let zoom_frac = scroll_delta / DEFAULT_ZOOM_FRAC;
        let scale = 2.0f32.powf(zoom_frac);
        self.zoom *= scale;
        self.shift = Pos2::new(
            mouse_pos.x - (mouse_pos.x - self.shift.x) * scale,
            mouse_pos.y - (mouse_pos.y - self.shift.y) * scale,
        );
    }

    pub fn update_drag(&mut self, movement: Vec2) {
        self.shift += movement;
    }
}
