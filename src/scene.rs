#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub title: String,
    pub content: String,
    pub scene_type: SceneTypes,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum SceneTypes {
    Explore,
    Text,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            title: String::from("新規シーン"),
            content: String::new(),
            scene_type: SceneTypes::Explore,
        }
    }
}
