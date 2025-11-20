use crate::panels;
use crate::scene::Scene;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MemoApp {
    scenes: Vec<Scene>,
    selected_index: usize,
    create_index: usize,
}

impl Default for MemoApp {
    fn default() -> Self {
        Self {
            scenes: vec![
                Scene {
                    title: String::from("シーン1"),
                    content: String::from("テスト内容1"),
                },
                Scene {
                    title: String::from("シーン2"),
                    content: String::from("テスト内容2"),
                },
                Scene {
                    title: String::from("シーン3"),
                    content: String::from("テスト内容3"),
                },
            ],
            selected_index: 0,
            create_index: 1,
        }
    }
}

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

        panels::top::show(ctx);
        panels::side::show(ctx, &self.scenes, &mut self.selected_index);
        panels::central::show(
            ctx,
            &mut self.scenes,
            &mut self.selected_index,
            &mut self.create_index,
        );
    }
}

