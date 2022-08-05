use eframe::egui::{Button, Color32, CursorIcon, Sense, Ui, Vec2, Widget, WidgetText};

pub fn label_click_to_copy(ui: &mut Ui, text: impl Into<WidgetText>) {
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

pub fn color_click_to_copy(ui: &mut Ui, color: impl Into<Color32>) {
    let color = color.into();

    if Button::new("              ")
        .fill(color)
        .ui(ui)
        .on_hover_text("Click to copy rgb values")
        .on_hover_cursor(CursorIcon::Copy)
        .clicked()
    {
        ui.output().copied_text = format!("{}, {}, {}", color.r(), color.g(), color.b());
    }
}
