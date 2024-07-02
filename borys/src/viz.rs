use crate::{
    spaceship::{self, Point},
    zoomer::{Zoomer, PIXELS_PER_POINT, SCREEN_H, SCREEN_W},
    TEST_ID,
};
use eframe::egui::{CentralPanel, Color32, Context, Pos2, Stroke};
use egui::{FontId, Vec2};

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
        // spaceship::check_solution(&input, &solution);

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
                let clip_rect = painter.clip_rect();

                {
                    let top_left = self.zoomer.convert_back(clip_rect.min);
                    let bottom_right = self.zoomer.convert_back(clip_rect.max);

                    let width = (bottom_right.x - top_left.x).abs();
                    let height = (bottom_right.y - top_left.y).abs();

                    const MAX_GRID: i64 = 50;
                    let mut every = 1;
                    while width / every > MAX_GRID || height / every > MAX_GRID {
                        every *= 10;
                    }
                    for xi in top_left.x / every..=bottom_right.x / every {
                        let x = xi * every;
                        let pos1 = self.zoomer.convert(Point::new(x, bottom_right.y));
                        let pos2 = self.zoomer.convert(Point::new(x, top_left.y));
                        painter.line_segment(
                            [pos1, pos2],
                            Stroke {
                                width: 1.0,
                                color: Color32::from_gray(128),
                            },
                        );
                        painter.text(
                            pos1,
                            egui::Align2::LEFT_BOTTOM,
                            format!("{}", x),
                            FontId::default(),
                            Color32::from_gray(128),
                        );
                    }
                    for yi in bottom_right.y / every..=top_left.y / every {
                        let y = yi * every;
                        let pos1 = self.zoomer.convert(Point::new(top_left.x, y));
                        let pos2 = self.zoomer.convert(Point::new(bottom_right.x, y));
                        painter.line_segment(
                            [pos1, pos2],
                            Stroke {
                                width: 1.0,
                                color: Color32::from_gray(128),
                            },
                        );
                        painter.text(
                            pos1,
                            egui::Align2::LEFT_BOTTOM,
                            format!("{}", y),
                            FontId::default(),
                            Color32::from_gray(128),
                        );
                    }
                }

                let mut cnt_inside = 0;
                for p in &self.input {
                    let converted = self.zoomer.convert(*p);
                    if clip_rect.contains(converted) {
                        painter.circle(converted, 6.0, Color32::RED, Stroke::NONE);
                        cnt_inside += 1;
                    }
                }
                if cnt_inside < 20 {
                    for p in &self.input {
                        let converted = self.zoomer.convert(*p);
                        painter.text(
                            converted,
                            egui::Align2::LEFT_BOTTOM,
                            format!("{:?}", p),
                            FontId::default(),
                            Color32::BLACK,
                        );
                    }
                }

                let mut lines_inside = 0;
                for w in self.sol_path.windows(2) {
                    let p1 = self.zoomer.convert(w[0]);
                    let p2 = self.zoomer.convert(w[1]);
                    if clip_rect.contains(p1) || clip_rect.contains(p2) {
                        lines_inside += 1;
                    }
                }
                if lines_inside < 10000 {
                    for w in self.sol_path.windows(2) {
                        let p1 = self.zoomer.convert(w[0]);
                        let p2 = self.zoomer.convert(w[1]);
                        if clip_rect.contains(p1) || clip_rect.contains(p2) {
                            lines_inside += 1;
                            painter.line_segment(
                                [p1, p2],
                                Stroke {
                                    width: 2.0,
                                    color: Color32::BLUE,
                                },
                            );
                        }
                    }
                    for p in self.sol_path.iter() {
                        painter.circle(self.zoomer.convert(*p), 4.0, Color32::BLUE, Stroke::NONE);
                    }
                } else {
                    const STEP: usize = 10;
                    for i in (0..self.sol_path.len() - STEP).step_by(STEP) {
                        let p1 = self.zoomer.convert(self.sol_path[i]);
                        let p2 = self.zoomer.convert(self.sol_path[i + STEP]);
                        painter.line_segment(
                            [p1, p2],
                            Stroke {
                                width: 2.0,
                                color: Color32::BLUE,
                            },
                        );
                    }
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
