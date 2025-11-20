use crate::scene::Scene;
use eframe::egui;

pub fn show(ctx: &egui::Context, scenes: &[Scene], selected_index: &mut usize) {
    egui::SidePanel::left("scene_list").show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, scene) in scenes.iter().enumerate() {
                if ui
                    .selectable_label(*selected_index == i, &scene.title)
                    .clicked()
                {
                    *selected_index = i;
                }
            }
        });
    });
}
