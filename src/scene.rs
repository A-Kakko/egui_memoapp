#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct TextSlot {
    /*
     * texts: 各判定のテキスト (judge_index -> text)
     * selected_judge_index: このスロットで選択中の判定インデックス
     * label: スロットの名前 (例: "PC1", "探索地点A" など、任意)
     * icon_path: アイコン画像のパス (将来用、任意)
     */
    pub texts: Vec<String>,
    pub selected_judge_index: usize,
    pub label: Option<String>,
    pub icon_path: Option<std::path::PathBuf>,
}

impl TextSlot {
    /// 空のTextSlotを作成
    ///
    /// # Arguments
    /// * `judge_count` - 判定の数（テキスト配列のサイズ）
    ///
    /// # Returns
    /// 各判定のテキストが空文字列で初期化されたTextSlot
    pub fn new_empty(judge_count: usize) -> Self {
        Self {
            texts: vec![String::new(); judge_count],
            selected_judge_index: 0,
            label: None,
            icon_path: None,
        }
    }

    /// ラベルを設定する（Builderパターン）
    ///
    /// # Arguments
    /// * `label` - 設定するラベル文字列
    ///
    /// # Returns
    /// ラベルが設定されたTextSlot（所有権を移動）
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// アイコンパスを設定する（Builderパターン）
    ///
    /// # Arguments
    /// * `icon_path` - 設定するアイコンのパス
    ///
    /// # Returns
    /// アイコンパスが設定されたTextSlot（所有権を移動）
    pub fn with_icon(mut self, icon_path: std::path::PathBuf) -> Self {
        self.icon_path = Some(icon_path);
        self
    }
}

/// レイアウトキャッシュ（モード変更時に無効化される）
#[derive(Clone, Debug)]
pub struct LayoutCache {
    /// 判定ボタンエリアの幅
    pub judge_width: f32,
    /// アイコンエリアの幅
    pub icon_width: f32,
    /// このキャッシュが有効なモードのインデックス
    pub mode_index: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    /*
     * title:シーン名
     * mode_index:選択中のモードインデックス
     * contents: [mode_index][slot_index] -> TextSlot
     * layout_cache: レイアウト幅のキャッシュ（シリアライズ対象外）
     */
    pub title: String,
    pub mode_index: usize,
    pub contents: Vec<Vec<TextSlot>>,
    #[serde(skip)]
    pub layout_cache: Option<LayoutCache>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Mode {
    /*
     * name:シーンタイプの名前(探索/地の文/etc)
     * judges:持っている判定種類(成功/ファンブルor地の文)
     */
    pub name: String,
    pub judges: Vec<String>,
    pub default_text_num: usize,
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
    /// 各モードに対して、default_text_num個のTextSlotを作成
    pub fn new(index: usize, modes: &[Mode]) -> Self {
        let contents = modes
            .iter()
            .map(|mode| {
                // このモードのdefault_text_num個のスロットを作成
                (0..mode.default_text_num)
                    .map(|_| TextSlot::new_empty(mode.judges.len()))
                    .collect()
            })
            .collect();

        Self {
            title: format!("新規シーン{}", index),
            mode_index: 1,
            contents,
            layout_cache: None,
        }
    }
}
