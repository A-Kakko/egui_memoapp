use crate::widgets::combobox::add_wheel;
use crate::{
    app::AppMode,
    scene::{Mode, Scene},
};
use eframe::egui;
use egui::Button;

const TEXTBOX_MIN_HEIGHT: f32 = 50.0;

/// CentralPanelのメイン表示関数
#[allow(clippy::too_many_arguments)]
pub fn show(
    ctx: &egui::Context,
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    create_index: &mut usize,
    app_mode: &AppMode,
    toasts: &mut egui_notify::Toasts,
    editing_scene_name_modal_open: &mut bool,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // 上段: シーン選択、モード選択、追加/削除ボタン
        ui.horizontal(|ui| {
            show_scene_selector(ui, scenes, selected_index);
            show_scene_edit_button(ui, editing_scene_name_modal_open);
            show_mode_selector(ui, modes, scenes, selected_index);
            show_scene_buttons(ui, scenes, selected_index, create_index);
        });

        // 下段: 判定ボタンとテキストエディタ
        ui.horizontal(|ui| {
            let text_height = calc_height_from_buttons(ui, modes, scenes, *selected_index);
            show_judge_buttons(ui, modes, scenes, selected_index, text_height);
            show_text_editor(ui, scenes, selected_index, text_height, app_mode, toasts);
        });
    });
}

/// シーン選択UI（左右ボタン + ComboBox）
fn show_scene_selector(ui: &mut egui::Ui, scenes: &[Scene], selected_index: &mut usize) {
    ui.label("Choose Scene");
    if ui.button("◀").clicked() {
        *selected_index = selected_index.saturating_sub(1);
    }

    if let Some(scene) = scenes.get(*selected_index) {
        let combo_response = egui::ComboBox::from_id_salt("scene_combo")
            .selected_text(&scene.title)
            .show_ui(ui, |ui| {
                for (index, scene) in scenes.iter().enumerate() {
                    ui.selectable_value(selected_index, index, &scene.title);
                }
            })
            .response;
        add_wheel(ui, selected_index, &scenes, &combo_response);
    }

    if ui.button("▶").clicked() {
        *selected_index = (*selected_index + 1).min(scenes.len() - 1);
    }
}

/// モード選択ComboBox（地の文/探索など）
fn show_mode_selector(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
) {
    // 借用エラー回避のため先にmode_indexを取得
    let current_mode_index = scenes.get(*selected_index).map(|s| s.mode_index);
    if let Some(mode_index) = current_mode_index {
        if let Some(current_mode) = modes.get(mode_index) {
            ui.label("Choose Mode:");
            let combo_resp = egui::ComboBox::from_id_source("mode_combo")
                .selected_text(&current_mode.name)
                .show_ui(ui, |ui| {
                    for (index, mode) in modes.iter().enumerate() {
                        if ui
                            .selectable_label(mode_index == index, &mode.name)
                            .clicked()
                        {
                            if let Some(scene_mut) = scenes.get_mut(*selected_index) {
                                scene_mut.mode_index = index;
                                scene_mut.selected_judge_index = 0; // モード変更時は判定をリセット
                            }
                        }
                    }
                })
                .response;
        }
    }
}
/// シーン名編集ボタン
fn show_scene_edit_button(ui: &mut egui::Ui, editing_scene_name_modal_open: &mut bool) {
    if ui
        .add(
            egui::Button::new(egui::RichText::new("✏ 編集").strong())
                .fill(egui::Color32::DARK_BLUE),
        )
        .clicked()
    {
        *editing_scene_name_modal_open = true;
    }
}

/// シーン追加/削除ボタン
#[allow(clippy::collapsible_if)]
fn show_scene_buttons(
    ui: &mut egui::Ui,
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    create_index: &mut usize,
) {
    if ui
        .add(
            egui::Button::new(egui::RichText::new("シーン追加").strong())
                .fill(egui::Color32::DARK_GREEN),
        )
        .clicked()
    {
        scenes.push(Scene::new(*create_index));
        *create_index += 1;
        *selected_index = scenes.len() - 1; // 新規シーンを選択
    }

    if ui
        .add(
            egui::Button::new(egui::RichText::new("シーン削除").strong())
                .fill(egui::Color32::DARK_RED),
        )
        .clicked()
    {
        if scenes.len() > 1 {
            scenes.remove(*selected_index);
            *selected_index = selected_index.saturating_sub(1);
        }
    }
}

/// 判定ボタン群（大成功/成功/失敗/ファンブルなど）
fn show_judge_buttons(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut [Scene],
    selected_index: &mut usize,
    text_height: f32,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(120.0, text_height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            // 借用エラー回避のため先に必要な値を取得
            let (mode_index, selected_judge_index) =
                if let Some(scene) = scenes.get(*selected_index) {
                    (scene.mode_index, scene.selected_judge_index)
                } else {
                    return;
                };

            if let Some(mode) = modes.get(mode_index) {
                for (index, judge) in mode.judges.iter().enumerate() {
                    let is_selected = selected_judge_index == index;
                    // 選択中のボタンは青色でハイライト
                    let button = if is_selected {
                        egui::Button::new(judge).fill(egui::Color32::from_rgb(70, 130, 180))
                    } else {
                        egui::Button::new(judge)
                    };

                    if ui.add(button).clicked() {
                        if let Some(scene_mut) = scenes.get_mut(*selected_index) {
                            scene_mut.selected_judge_index = index;
                        }
                    }
                }
            }
        },
    );
}

/// テキストエディタ（マルチライン）
fn show_text_editor(
    ui: &mut egui::Ui,
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    text_height: f32,
    app_mode: &AppMode,
    toasts: &mut egui_notify::Toasts,
) {
    if let Some(scene) = scenes.get_mut(*selected_index) {
        if let Some(content) = scene
            .contents
            .get_mut(scene.mode_index)
            .and_then(|c| c.get_mut(scene.selected_judge_index))
        {
            match app_mode {
                AppMode::Edit => {
                    ui.add_sized(
                        [ui.available_width(), text_height],
                        egui::TextEdit::multiline(content),
                    );
                }
                AppMode::Copy => {
                    let mut dummy = content.clone();
                    let response =
                        ui.add(egui::TextEdit::multiline(&mut dummy).desired_width(f32::INFINITY));
                    // dummyは捨てる（元のcontentは変更されない）

                    if response.clicked() {
                        ui.ctx().copy_text(content.clone());
                        toasts
                            .success("コピーしました")
                            .duration(Some(std::time::Duration::from_secs(2)));
                    }
                }
            }
        }
    }
}

/// ボタンの数からテキストボックスの高さを計算
fn calc_height_from_buttons(
    ui: &egui::Ui,
    modes: &[Mode],
    scenes: &[Scene],
    selected_index: usize,
) -> f32 {
    if let Some(scene) = scenes.get(selected_index) {
        if let Some(mode) = modes.get(scene.mode_index) {
            let button_count = mode.judges.len() as f32;
            let button_height = ui.spacing().interact_size.y;
            let button_spacing = ui.spacing().item_spacing.y;
            let total_button_height =
                button_count * button_height + (button_count - 1.0) * button_spacing;
            // 最小値とボタンの高さの大きい方を返す
            total_button_height.max(TEXTBOX_MIN_HEIGHT)
        } else {
            TEXTBOX_MIN_HEIGHT
        }
    } else {
        TEXTBOX_MIN_HEIGHT
    }
}
