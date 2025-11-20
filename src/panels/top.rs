use eframe::egui;

use crate::app::{self, AppMode};

/// TopPanelのメイン表示関数
pub fn show(ctx: &egui::Context, app_mode: &mut AppMode) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            show_file_menu(ctx, ui);
            show_appmode_buttons(ui, app_mode);
            show_theme_buttons(ui);
        });
    });
}

/// Fileメニュー（Quitボタン）
fn show_file_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    let is_web = cfg!(target_arch = "wasm32");
    if !is_web {
        ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        ui.add_space(16.0);
    }
}

/// AppMode切り替えボタン（編集/コピー）
fn show_appmode_buttons(ui: &mut egui::Ui, app_mode: &mut AppMode) {
    if ui
        .selectable_label(*app_mode == AppMode::Edit, "編集")
        .clicked()
    {
        *app_mode = AppMode::Edit;
    }
    if ui
        .selectable_label(*app_mode == AppMode::Copy, "コピー")
        .clicked()
    {
        *app_mode = AppMode::Copy;
    }
}

/// テーマ切り替えボタン（右端に配置）
fn show_theme_buttons(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        egui::widgets::global_theme_preference_buttons(ui);
    });
}
