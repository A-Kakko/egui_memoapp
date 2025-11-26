use crate::constants::constants::*;
use crate::scene;
use crate::widgets::combobox::enable_wheel;
use crate::{
    app::{AppMode, Modal, Player_default},
    scene::{LayoutCache, Mode, Scene},
};
#[allow(unused_imports)]
use eframe::{App, egui};
use egui::widgets;

/// CentralPanelのメイン表示関数
#[allow(clippy::too_many_arguments)]
pub fn show(
    ctx: &egui::Context,
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_scene_index: &mut usize,
    create_index: &mut usize,
    app_mode: &AppMode,
    mordal: &mut Modal,
    toasts: &mut egui_notify::Toasts,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // 上段: シーン選択、モード選択、追加/削除ボタン
        ui.horizontal(|ui| {
            show_scene_selector(ui, scenes, selected_scene_index);
            show_scene_edit_button(ui, &mut mordal.editing_scene_name_modal_open);
            show_mode_selector(ui, modes, scenes, selected_scene_index);
            show_scene_buttons(
                ui,
                modes,
                scenes,
                selected_scene_index,
                create_index,
                &mut mordal.editing_scene_delete_modal_open,
            );
        });

        // 下段: 全スロットを縦に並べて表示
        show_all_slots(ui, modes, scenes, selected_scene_index, app_mode, toasts);
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
        enable_wheel(ui, selected_index, &scenes, &combo_response);
    }

    if ui.button("▶").clicked() {
        *selected_index = (*selected_index + 1).min(scenes.len() - 1);
    }
}

/// モード選択ComboBox（地の文/探索など）
fn show_mode_selector(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut [Scene],
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
                                // モード変更時は各スロットの判定をリセット
                                if let Some(mode_slots) = scene_mut.contents.get_mut(index) {
                                    for slot in mode_slots.iter_mut() {
                                        slot.selected_judge_index = 0;
                                    }
                                }
                                // キャッシュを無効化
                                scene_mut.layout_cache = None;
                            }
                        }
                    }
                })
                .response;

            // ホイールでモード切り替え
            if let Some(scene) = scenes.get_mut(*selected_index) {
                enable_wheel(ui, &mut scene.mode_index, modes, &combo_resp);
            }
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
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    create_index: &mut usize,
    editing_scene_delete_modal_open: &mut bool,
) {
    if ui
        .add(
            egui::Button::new(egui::RichText::new("シーン追加").strong())
                .fill(egui::Color32::DARK_GREEN),
        )
        .clicked()
    {
        scenes.push(Scene::new(*create_index, modes));
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
            *editing_scene_delete_modal_open = true;
        }
    }
}

/// 全スロット表示（縦に並べて表示 + 追加ボタン）
fn show_all_slots(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut Vec<Scene>,
    selected_scene_index: &mut usize,
    app_mode: &AppMode,
    toasts: &mut egui_notify::Toasts,
) {
    ui.vertical(|ui| {
        let mut max_judge_width: f32 = 0.0;
        let mut max_icon_width: f32 = 0.0;
        let mut cache_valid = false;

        // キャッシュのチェック
        if let Some(scene) = scenes.get(*selected_scene_index) {
            if let Some(cache) = &scene.layout_cache {
                if cache.mode_index == scene.mode_index {
                    // キャッシュが有効
                    max_judge_width = cache.judge_width;
                    max_icon_width = cache.icon_width;
                    cache_valid = true;
                }
            }
        }

        // キャッシュが無効な場合は再計算
        if !cache_valid {
            if let Some(scene) = scenes.get(*selected_scene_index) {
                let mode_index = scene.mode_index;
                if let Some(mode_slots) = scene.contents.get(mode_index) {
                    let slot_count = mode_slots.len();

                    // 各スロットを表示し、最大幅を記録
                    for slot_index in 0..slot_count {
                        let (judge_width, icon_width) = show_slot(
                            ui,
                            modes,
                            scenes,
                            selected_scene_index,
                            slot_index,
                            app_mode,
                            toasts,
                        );
                        max_judge_width = max_judge_width.max(judge_width);
                        max_icon_width = max_icon_width.max(icon_width);
                        ui.add_space(3.0);
                    }

                    // キャッシュを更新
                    if let Some(scene_mut) = scenes.get_mut(*selected_scene_index) {
                        scene_mut.layout_cache = Some(LayoutCache {
                            judge_width: max_judge_width,
                            icon_width: max_icon_width,
                            mode_index: scene_mut.mode_index,
                        });
                    }
                }
            }
        } else {
            // キャッシュが有効な場合でもスロットを描画
            if let Some(scene) = scenes.get(*selected_scene_index) {
                let mode_index = scene.mode_index;
                if let Some(mode_slots) = scene.contents.get(mode_index) {
                    let slot_count = mode_slots.len();

                    for slot_index in 0..slot_count {
                        show_slot(
                            ui,
                            modes,
                            scenes,
                            selected_scene_index,
                            slot_index,
                            app_mode,
                            toasts,
                        );
                        ui.add_space(3.0);
                    }
                }
            }
        }

        // +ボタン（左余白を判定ボタン幅+アイコン幅に合わせる）
        show_add_slot_button(
            ui,
            scenes,
            selected_scene_index,
            modes,
            app_mode,
            max_judge_width,
            max_icon_width,
        );
    });
}

/// 1つのスロットを表示（判定ボタン + アイコン + テキストエディタ）
/// 返り値: (判定ボタン幅, アイコンエリア幅)
fn show_slot(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut [Scene],
    selected_index: &mut usize,
    slot_index: usize,
    app_mode: &AppMode,
    toasts: &mut egui_notify::Toasts,
) -> (f32, f32) {
    let mut judge_width = 0.0;
    let mut icon_width = 0.0;

    ui.horizontal(|ui| {
        let text_height = calc_height_from_buttons(ui, modes, scenes, *selected_index);

        // このスロット用の判定ボタン
        let judge_response =
            show_judge_buttons_for_slot(ui, modes, scenes, selected_index, slot_index, text_height);
        judge_width = judge_response.rect.width();

        // アイコン/名前のエリア
        let icon_response = ui.allocate_ui_with_layout(
            egui::vec2(0.0, text_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                show_player_icon(ui, scenes);
                show_player_name(ui, scenes);
            },
        );
        icon_width = icon_response.response.rect.width();

        show_text_editor_for_slot(
            ui,
            scenes,
            selected_index,
            slot_index,
            text_height,
            app_mode,
            toasts,
        );
    });

    (judge_width, icon_width)
}

/// +ボタン（スロット追加）
fn show_add_slot_button(
    ui: &mut egui::Ui,
    scenes: &mut Vec<Scene>,
    selected_scene_index: &mut usize,
    modes: &[Mode],
    app_mode: &AppMode,
    judge_width: f32,
    icon_width: f32,
) {
    ui.horizontal(|ui| {
        // 判定ボタンと同じ幅を確保
        ui.allocate_space(egui::vec2(judge_width, 0.0));

        // アイコン/名前エリアと同じ幅を確保
        ui.allocate_space(egui::vec2(icon_width, 0.0));

        if let Some(scene) = scenes.get_mut(*selected_scene_index) {
            show_add_textbox_button(ui, scene, modes, app_mode);
        }
    });
}

/// 判定ボタン群（大成功/成功/失敗/ファンブルなど） - 特定スロット用
fn show_judge_buttons_for_slot(
    ui: &mut egui::Ui,
    modes: &[Mode],
    scenes: &mut [Scene],
    selected_index: &usize,
    slot_index: usize,
    text_height: f32,
) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(120.0, text_height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            // 借用エラー回避のため先に必要な値を取得
            let (mode_index, selected_judge_index) =
                if let Some(scene) = scenes.get(*selected_index) {
                    let judge_idx = scene
                        .contents
                        .get(scene.mode_index)
                        .and_then(|slots| slots.get(slot_index))
                        .map(|slot| slot.selected_judge_index)
                        .unwrap_or_else(|| {
                            eprintln!("Warning: slot not found, defaulting to 0");
                            0
                        });
                    (scene.mode_index, judge_idx)
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
                            if let Some(slot) = scene_mut
                                .contents
                                .get_mut(mode_index)
                                .and_then(|slots| slots.get_mut(slot_index))
                            {
                                slot.selected_judge_index = index;
                            }
                        }
                    }
                }
            }
        },
    )
    .response
}

fn show_player_icon(ui: &mut egui::Ui, scenes: &[Scene]) {
    //todo!()
}

fn show_player_name(ui: &mut egui::Ui, scenes: &[Scene]) {
    //todo!()
}

/// テキストエディタ（マルチライン） - 指定されたスロット用
fn show_text_editor_for_slot(
    ui: &mut egui::Ui,
    scenes: &mut [Scene],
    selected_index: &usize,
    slot_index: usize,
    text_height: f32,
    app_mode: &AppMode,
    toasts: &mut egui_notify::Toasts,
) {
    if let Some(scene) = scenes.get_mut(*selected_index) {
        let mode_index = scene.mode_index;

        // このスロットのselected_judge_indexを取得
        let judge_index = scene
            .contents
            .get(mode_index)
            .and_then(|slots| slots.get(slot_index))
            .map(|slot| slot.selected_judge_index)
            .unwrap_or(0);

        // テキストを取得: contents[mode][slot].texts[judge]
        if let Some(content) = scene
            .contents
            .get_mut(mode_index)
            .and_then(|slots| slots.get_mut(slot_index))
            .and_then(|slot| slot.texts.get_mut(judge_index))
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
                    let response = ui.add_sized(
                        [ui.available_width(), text_height],
                        egui::TextEdit::multiline(&mut dummy).desired_width(f32::INFINITY),
                    );
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

fn show_add_textbox_button(
    ui: &mut egui::Ui,
    scene: &mut Scene,
    modes: &[Mode],
    app_mode: &AppMode,
) {
    match app_mode {
        AppMode::Edit => {
            if ui
                .add_sized(
                    [ui.available_width(), 0.0],
                    egui::Button::new(egui::RichText::new("+").strong())
                        .fill(egui::Color32::DARK_GRAY),
                )
                .clicked()
            {
                // 現在のモードにスロットを追加
                let mode_index = scene.mode_index;
                if let Some(mode) = modes.get(mode_index) {
                    if let Some(mode_slots) = scene.contents.get_mut(mode_index) {
                        // 新しいスロットを作成（各判定のテキストは空）
                        let new_slot = scene::TextSlot::new_empty(mode.judges.len());
                        mode_slots.push(new_slot);
                    }
                }
                // スロット追加時はキャッシュを無効化（幅が変わる可能性がある）
                scene.layout_cache = None;
            }
        }

        AppMode::Copy => { /* Nop */ }
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
