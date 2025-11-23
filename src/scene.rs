#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    /*
     * title:シーン名
     * mode_index:選択中のモードインデックス
     * selected_judge_index:選択中の判定インデックス
     * contents:シーンタイプ(探索/地の文)
     */
    pub title: String,
    pub mode_index: usize,
    pub selected_judge_index: usize,
    pub player_index: usize,
    pub contents_index: usize,
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
    /// modesから動的にcontentsを生成する
    /// 各モードのjudgesの数だけString::new()を作成
    pub fn new(index: usize, modes: &[Mode]) -> Self {
        let contents = modes
            .iter()
            .map(|mode| vec![String::new(); mode.judges.len()])
            .collect();

        Self {
            title: format!("新規シーン{}", index),
            mode_index: 1,
            contents,
            selected_judge_index: 0,
            player_index: 0,
            contents_index: 0,
        }
    }
}
