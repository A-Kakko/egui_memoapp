#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub title: String,
    pub content: String,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            title: String::from("新規シーン"),
            content: String::new(),
        }
    }
}
