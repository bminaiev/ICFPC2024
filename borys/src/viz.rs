use crate::spaceship::{self, Point};
use eframe::egui::{CentralPanel, Color32, Context, Pos2, Stroke};
use egui::Vec2;

const SCREEN_W: f32 = 3500.0;
const SCREEN_H: f32 = 2000.0;
const PIXELS_PER_POINT: f32 = 1.5;
const DEFAULT_ZOOM_FRAC: f32 = 300.0;

const TEST_ID: usize = 9;

struct Zoomer {
    zoom: f32,
    shift: Pos2,
}

impl Zoomer {
    fn new(pts: &[Point]) -> Self {
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

    fn convert(&self, p: Point) -> Pos2 {
        Pos2::new(
            p.x as f32 * self.zoom + self.shift.x,
            -p.y as f32 * self.zoom + self.shift.y,
        )
    }

    fn ensure_fits(&self, pts: &[Point]) {
        for p in pts {
            let pos = self.convert(*p);
            assert!(pos.x >= 0.0 && pos.x <= SCREEN_W);
            assert!(pos.y >= 0.0 && pos.y <= SCREEN_H);
        }
    }

    fn update_scroll(&mut self, mouse_pos: Pos2, scroll_delta: f32) {
        let zoom_frac = scroll_delta / DEFAULT_ZOOM_FRAC;
        let scale = 2.0f32.powf(zoom_frac);
        self.zoom *= scale;
        self.shift = Pos2::new(
            mouse_pos.x - (mouse_pos.x - self.shift.x) * scale,
            mouse_pos.y - (mouse_pos.y - self.shift.y) * scale,
        );
    }

    fn update_drag(&mut self, movement: Vec2) {
        self.shift += movement;
    }
}

struct App {
    input: Vec<Point>,
    zoomer: Zoomer,
    test_id: usize,
    sol_path: Vec<Point>,
}

impl App {
    fn name() -> &'static str {
        "Spaceship"
    }
}

fn convert_sol_to_path(sol: &[Point]) -> Vec<Point> {
    let mut pos = Point::ZERO;
    let mut velocity = Point::ZERO;
    let mut path = vec![pos];
    for &p in sol {
        velocity += p;
        pos += velocity;
        path.push(pos);
    }
    path
}

impl Default for App {
    fn default() -> Self {
        let test_id = TEST_ID;
        let input = spaceship::read_input(test_id);
        let zoomer = Zoomer::new(&input);
        zoomer.ensure_fits(&input);

        let solution = spaceship::read_solution(test_id);

        Self {
            input,
            zoomer,
            test_id,
            sol_path: convert_sol_to_path(&solution),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(PIXELS_PER_POINT);

        let (hover_pos, scroll_delta, pointer_delta, is_pressed) = ctx.input(|input| {
            (
                input.pointer.hover_pos(),
                input.raw_scroll_delta,
                input.pointer.delta(),
                input.pointer.primary_down(),
            )
        });
        if let Some(mousepos) = hover_pos {
            if scroll_delta.y != 0.0 {
                self.zoomer.update_scroll(mousepos, scroll_delta.y);
            }
        }
        if is_pressed {
            self.zoomer.update_drag(pointer_delta);
        }

        // Setup the central panel with a white background
        CentralPanel::default()
            .frame(egui::Frame {
                fill: Color32::WHITE,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.heading(format!(
                    "Test #{}. #points: {}. Score: {}",
                    self.test_id,
                    self.input.len(),
                    self.sol_path.len() - 1
                ));
                let painter = ui.painter();

                for p in &self.input {
                    painter.circle(
                        self.zoomer.convert(*p),
                        5.0,
                        Color32::RED,
                        Stroke {
                            width: 1.0,
                            color: Color32::from_rgb(255, 255, 255),
                        },
                    );
                }

                for w in self.sol_path.windows(2) {
                    painter.line_segment(
                        [self.zoomer.convert(w[0]), self.zoomer.convert(w[1])],
                        Stroke {
                            width: 1.0,
                            color: Color32::BLUE,
                        },
                    );
                }
                for p in self.sol_path.iter() {
                    painter.circle(
                        self.zoomer.convert(*p),
                        2.0,
                        Color32::BLUE,
                        Stroke {
                            width: 1.0,
                            color: Color32::BLACK,
                        },
                    );
                }
            });
    }
}

pub fn viz_main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((SCREEN_W, SCREEN_H)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        App::name(),
        native_options,
        Box::new(|_| Box::new(App::default())),
    )
}
