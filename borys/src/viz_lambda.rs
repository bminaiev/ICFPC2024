use crate::{
    array_2d::Array2D,
    lambdaman::{self, VerySimpleRng, DEFAULT_STEPS_LIMIT},
    spaceship::{self, Point},
    zoomer::{Zoomer, PIXELS_PER_POINT, SCREEN_H, SCREEN_W},
    TEST_ID,
};
use eframe::egui::{CentralPanel, Color32, Context, Pos2, Stroke};
use egui::{FontId, Rect, Rounding, Vec2};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

struct App {
    zoomer: Zoomer,
    test_id: usize,
    task: lambdaman::Task,
    visited: Array2D<bool>,
}

impl App {
    fn name() -> &'static str {
        "Lambdaman"
    }
}

impl Default for App {
    fn default() -> Self {
        let test_id = TEST_ID;

        let task = lambdaman::read_task(test_id);
        let n = task.walls.len();
        let m = task.walls[0].len();
        let input = vec![Point::new(0, 0), Point::new(n as i64, m as i64)];
        let zoomer = Zoomer::new(&input);

        let best_seed = 159444415;
        let best_seed = lambdaman::find_good_seed(&task.walls, task.start);

        let visited = lambdaman::eval(
            &task.walls,
            task.start,
            &mut VerySimpleRng::seed_from_u64(best_seed),
            DEFAULT_STEPS_LIMIT,
        );

        // let visited = Array2D::new(false, n, m);

        Self {
            zoomer,
            test_id,
            task,
            visited,
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
                    "Test #{}. n: {}",
                    self.test_id,
                    self.task.walls.len()
                ));
                let painter = ui.painter();
                let clip_rect = painter.clip_rect();

                let n = self.task.walls.len();
                for i in 0..n {
                    for j in 0..self.task.walls[i].len() {
                        let p1 = self.zoomer.convert(Point::new(i as i64, j as i64 + 1));
                        let p3 = self.zoomer.convert(Point::new(i as i64 + 1, j as i64));
                        let color: Color32 = if self.task.walls[i][j] {
                            Color32::from_rgb(141, 184, 199)
                        } else if self.visited[i][j] {
                            Color32::from_rgb(216, 240, 211)
                        } else {
                            Color32::WHITE
                        };

                        painter.rect(
                            Rect { min: p1, max: p3 },
                            Rounding::default(),
                            color,
                            Stroke { width: 1.0, color },
                        );
                    }
                }
                {
                    // start
                    let p1 = self.zoomer.convert(Point::new(
                        self.task.start.0 as i64,
                        self.task.start.1 as i64 + 1,
                    ));
                    let p3 = self.zoomer.convert(Point::new(
                        self.task.start.0 as i64 + 1,
                        self.task.start.1 as i64,
                    ));
                    let center = Pos2 {
                        x: (p1.x + p3.x) / 2.0,
                        y: (p1.y + p3.y) / 2.0,
                    };
                    let radius = (p3.x - p1.x) / 2.0;
                    painter.circle_filled(center, radius, Color32::DARK_RED);
                }
            });
    }
}

pub fn viz_lambda_main() -> eframe::Result<()> {
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
