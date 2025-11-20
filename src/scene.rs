#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    /*
     * title:シーン名
     * mode_index:表示テキスト
     * contents:シーンタイプ(探索/地の文)
     * selected_judge_index:選択中の判定インデックス
     */
    pub title: String,
    pub mode_index: usize,
    pub selected_judge_index: usize,
    pub contents: Vec<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Mode {
    /*
     * name:シーンタイプの名前(探索/地の文/etc)
     * judges:持っている判定種類(成功/ファンブルor地の文)
     */
    pub name: String,
    pub judges: Vec<String>,
}

// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct Judgement {
//     /*
//      * name:判定の名前
//      * content:表示されるテキスト
//      * */
//     pub name: String,
//     pub text: String,
// }

impl Scene {
    pub fn new(index: usize) -> Self {
        Self {
            title: format!("新規シーン{}", index),
            mode_index: 1,
            contents: vec![
                vec![String::new()], // 地の文用（1つ）
                vec![String::new(), String::new(), String::new(), String::new()], // 探索用（4つ）
            ],
            selected_judge_index: 0,
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new(0)
    }
}
