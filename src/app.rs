/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MemoApp {
    scenes: Vec<Scene>,
    selected_index: usize,
    // #[serde(skip)], // This how you opt-out of serialization of a field
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Scene {
    title: String,
    content: String,
}

impl MemoApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
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
        }
    }
}

impl eframe::App for MemoApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::SidePanel::left("scene_list").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, scene) in self.scenes.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_index == i, &scene.title)
                        .clicked()
                    {
                        self.selected_index = i;
                    }
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Choose Scene")
                .selected_text(self.scenes[self.selected_index].title.clone())
                .show_ui(ui, |ui| {
                    for (index, scene) in self.scenes.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_index, index, &scene.title);
                    }
                });

            if let Some(scene) = self.scenes.get_mut(self.selected_index) {
                ui.text_edit_multiline(&mut scene.content);
            }
        });
    }
}
