use crate::panels;
use crate::scene::{Mode, Scene};
use egui::Key;
use egui_notify::Toasts;
use std::path::PathBuf;
//TODO:プレイヤー追加
//TODO:アイコン/名前表示
//TODO:ファイルIO
//TODO:パーサー
//TODO:それのやり取りするInterface(Trate)
//TODO:設定ファイル追加
//TODO:ショートカットキー(一部追加済)
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MemoApp {
    scenes: Vec<Scene>,
    modes: Vec<Mode>,
    selected_scene_index: usize,
    create_index: usize,
    app_mode: AppMode,
    player: Vec<Player>,
    #[serde(skip)]
    toasts: Toasts,
    #[serde(skip)]
    editing_scene_name_modal_open: bool,
    #[serde(skip)]
    editing_scene_name_buffer: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Player {
    name: String,
    icon_path: Option<PathBuf>,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum AppMode {
    Edit,
    Copy,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::new(),
            icon_path: None,
        }
    }
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
            player: vec![Player {
                name: String::from("デフォルト太郎"),
                icon_path: None,
            }],
            editing_scene_name_modal_open: false,
            editing_scene_name_buffer: String::new(),
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

    /// シーン名編集モーダルを表示
    fn show_scene_name_edit_modal(&mut self, ctx: &egui::Context) {
        // Escで閉じる
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.editing_scene_name_modal_open = false;
            self.editing_scene_name_buffer.clear();
        }

        egui::Window::new("シーン名を編集")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("新しいシーン名:");
                ui.text_edit_singleline(&mut self.editing_scene_name_buffer);

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        // シーン名を更新

                        if !self.editing_scene_name_buffer.is_empty() {
                            if let Some(scene) = self.scenes.get_mut(self.selected_scene_index) {
                                scene.title = self.editing_scene_name_buffer.clone();
                            }
                        }
                        // モーダルを閉じる
                        self.editing_scene_name_modal_open = false;
                        self.editing_scene_name_buffer.clear();
                    }

                    if ui.button("キャンセル").clicked() {
                        // モーダルを閉じる
                        self.editing_scene_name_modal_open = false;
                        self.editing_scene_name_buffer.clear();
                    }
                });
            });
    }
}

impl eframe::App for MemoApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        // 前のフレームでモーダルが開いていたかを記録
        let was_modal_open = self.editing_scene_name_modal_open;

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
            &mut self.editing_scene_name_modal_open,
        );

        // モーダルが新しく開かれた場合のみバッファを初期化
        if self.editing_scene_name_modal_open && !was_modal_open {
            if let Some(scene) = self.scenes.get(self.selected_scene_index) {
                self.editing_scene_name_buffer = scene.title.clone();
            }
        }

        // シーン名編集モーダル
        if self.editing_scene_name_modal_open {
            self.show_scene_name_edit_modal(ctx);
        }

        self.toasts.show(ctx);
    }
}
