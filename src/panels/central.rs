use crate::scene::{Mode, Scene};
use eframe::egui;
const TEXTBOX_MIN_HEIGHT: f32 = 50.0;
pub fn show(
    ctx: &egui::Context,
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    create_index: &mut usize,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Choose Scene");
            if ui.button("◀").clicked() {
                *selected_index = selected_index.saturating_sub(1);
            }
            // シーンのリストボックス
            if let Some(scene) = scenes.get(*selected_index) {
                egui::ComboBox::from_id_salt("scene_combo")
                    .selected_text(&scene.title)
                    .show_ui(ui, |ui| {
                        for (index, scene) in scenes.iter().enumerate() {
                            ui.selectable_value(selected_index, index, &scene.title);
                        }
                    });
            }

            if ui.button("▶").clicked() {
                *selected_index = (*selected_index + 1).min(scenes.len() - 1);
            }
            // モード選択ComboBox
            let current_mode_index = scenes.get(*selected_index).map(|s| s.mode_index);
            if let Some(mode_index) = current_mode_index {
                if let Some(current_mode) = modes.get(mode_index) {
                    ui.label("Choose Mode:");
                    egui::ComboBox::from_id_source("mode_combo")
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
                        });
                }
            }

            // シーン追加/削除ボタン
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("シーン追加").strong())
                        .fill(egui::Color32::DARK_GREEN),
                )
                .clicked()
            {
                scenes.push(Scene::new(*create_index));
                *create_index += 1;
                *selected_index = scenes.len() - 1;
            }

            #[allow(clippy::collapsible_if)]
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("シーン削除").strong())
                        .fill(egui::Color32::DARK_GREEN),
                )
                .clicked()
            {
                if scenes.len() > 1 {
                    scenes.remove(*selected_index);
                    *selected_index = selected_index.saturating_sub(1);
                }
            }
        });

        // テキストボックス欄描写
        ui.horizontal(|ui| {
            // ボタンの数からテキストボックスの高さを計算
            let text_height = calc_height_from_buttons(ui, modes, scenes, *selected_index);

            // テキストの隣のボタン作成
            ui.allocate_ui_with_layout(
                egui::vec2(120.0, text_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    // 必要な値を先に取得して借用を解放
                    let (mode_index, selected_judge_index) =
                        if let Some(scene) = scenes.get(*selected_index) {
                            (scene.mode_index, scene.selected_judge_index)
                        } else {
                            return;
                        };

                    if let Some(mode) = modes.get(mode_index) {
                        for (index, judge) in mode.judges.iter().enumerate() {
                            let is_selected = selected_judge_index == index;
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
            // テキストボックス作成
            if let Some(scene) = scenes.get_mut(*selected_index) {
                ui.add_sized(
                    [ui.available_width(), text_height],
                    egui::TextEdit::multiline(
                        &mut scene.contents[scene.mode_index][scene.selected_judge_index],
                    ),
                );
            }
        });
    });
}

fn calc_height_from_buttons(
    ui: &egui::Ui,
    modes: &[Mode],
    scenes: &[Scene],
    selected_index: usize,
) -> f32 {
    if let Some(scene) = scenes.get(selected_index) {
        if let Some(mode) = modes.get(scene.mode_index) {
            let button_count = mode.judges.len() as f32;
            let button_height = ui.spacing().interact_size.y; // ボタン1つの高さ
            let button_spacing = ui.spacing().item_spacing.y; // ボタン間の余白
            let total_button_height =
                button_count * button_height + (button_count - 1.0) * button_spacing;
            total_button_height.max(TEXTBOX_MIN_HEIGHT)
        } else {
            TEXTBOX_MIN_HEIGHT
        }
    } else {
        TEXTBOX_MIN_HEIGHT
    }
}
