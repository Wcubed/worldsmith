use eframe::egui::{Button, CursorIcon, Ui, Widget, WidgetText};

pub fn label_with_copy(ui: &mut Ui, text: impl Into<WidgetText>) {
    let text = text.into();
    if Button::new(text.clone())
        .frame(false)
        .ui(ui)
        .on_hover_text("Click to copy")
        .on_hover_cursor(CursorIcon::Copy)
        .clicked()
    {
        ui.output().copied_text = text.text().to_owned();
    }
}
