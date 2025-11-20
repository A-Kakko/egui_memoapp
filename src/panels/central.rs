use crate::scene::Scene;
use eframe::egui;

pub fn show(
    ctx: &egui::Context,
    scenes: &mut Vec<Scene>,
    selected_index: &mut usize,
    create_index: &mut usize,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if let Some(scene) = scenes.get(*selected_index) {
                egui::ComboBox::from_label("Choose Scene")
                    .selected_text(&scene.title)
                    .show_ui(ui, |ui| {
                        for (index, scene) in scenes.iter().enumerate() {
                            ui.selectable_value(selected_index, index, &scene.title);
                        }
                    });
            }

            if ui
                .add(
                    egui::Button::new(egui::RichText::new("シーン追加").strong())
                        .fill(egui::Color32::DARK_GREEN),
                )
                .clicked()
            {
                scenes.push(Scene {
                    title: format!("新規シーン{}", *create_index),
                    content: String::new(),
                });
                *create_index += 1;
                *selected_index = scenes.len() - 1;
            }

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

        if let Some(scene) = scenes.get_mut(*selected_index) {
            ui.text_edit_multiline(&mut scene.content);
        }
    });
}
