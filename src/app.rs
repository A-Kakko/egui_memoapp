use crate::panels;
use crate::scene::{Mode, Scene};
use egui_notify::Toasts;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MemoApp {
    scenes: Vec<Scene>,
    modes: Vec<Mode>,
    selected_scene_index: usize,
    create_index: usize,
    app_mode: AppMode,
    #[serde(skip)]
    toasts: Toasts,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum AppMode {
    Edit,
    Copy,
}

impl Default for MemoApp {
    fn default() -> Self {
        Self {
            scenes: vec![
                Scene {
                    title: String::from("シーン1"),
                    mode_index: 1,
                    contents: vec![
                        vec![String::from("地の文1")],
                        vec![
                            String::from("大成功1"),
                            String::from("成功1"),
                            String::from("失敗1"),
                            String::from("ファンブル1"),
                        ],
                    ],
                    selected_judge_index: 0,
                },
                Scene {
                    title: String::from("シーン2"),
                    mode_index: 1,
                    contents: vec![
                        vec![String::from("地の文2")],
                        vec![
                            String::from("大成功2"),
                            String::from("成功2"),
                            String::from("失敗2"),
                            String::from("ファンブル2"),
                        ],
                    ],
                    selected_judge_index: 0,
                },
                Scene {
                    title: String::from("シーン3"),
                    mode_index: 1,
                    contents: vec![
                        vec![String::from("地の文3")],
                        vec![
                            String::from("大成功3"),
                            String::from("成功3"),
                            String::from("失敗3"),
                            String::from("ファンブル3"),
                        ],
                    ],
                    selected_judge_index: 0,
                },
            ],
            modes: vec![
                Mode {
                    name: String::from("地の文"),
                    judges: vec![String::from("本文")],
                },
                Mode {
                    name: String::from("探索"),
                    judges: vec![
                        String::from("大成功"),
                        String::from("成功"),
                        String::from("失敗"),
                        String::from("ファンブル"),
                    ],
                },
            ],
            selected_scene_index: 0,
            create_index: 1,
            app_mode: AppMode::Edit,
            toasts: Toasts::default(),
        }
    }
}

#[warn(dead_code)]
impl MemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for MemoApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        panels::top::show(ctx, &mut self.app_mode);
        panels::side::show(ctx, &self.scenes, &mut self.selected_scene_index);
        panels::central::show(
            ctx,
            &self.modes,
            &mut self.scenes,
            &mut self.selected_scene_index,
            &mut self.create_index,
            &self.app_mode,
            &mut self.toasts,
        );
        self.toasts.show(ctx);
    }
}
