use egui::{ComboBox, Response, Ui};

pub fn add_wheel<T>(ui: &mut Ui, current: &mut usize, items: &[T], response: &Response) {
    // マウスホイールでの操作を追加
    if response.hovered() {
        let scroll = ui.ctx().input(|i| i.raw_scroll_delta.y);
        if scroll > 0.0 && *current > 0 {
            *current -= 1;
        } else if scroll < 0.0 && *current < items.len() - 1 {
            *current += 1;
        }
    }
}
