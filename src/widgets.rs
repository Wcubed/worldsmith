use eframe::egui::style::WidgetVisuals;
use eframe::egui::{
    Align2, Button, Color32, CursorIcon, FontFamily, FontId, Painter, Pos2, Sense, Ui, Vec2,
    Widget, WidgetText,
};
use worldsmith_lib::units::SolarRadius;

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

/// Draws a chart comparing the given radius to other well-known stars.
pub fn star_size_comparison_chart(ui: &mut Ui, radius: SolarRadius) {
    let widget_size = Vec2::new(400.0, 100.0);

    let (response, painter) = ui.allocate_painter(widget_size, Sense::focusable_noninteractive());
    let rect = response.rect;

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        // This is drawn directly on the ui painter, instead of the
        // painter returned by `allocate_painter`, so that the border is not clipped.
        ui.painter().rect(
            rect.expand(visuals.expansion),
            visuals.rounding,
            visuals.bg_fill,
            visuals.bg_stroke,
        );

        // TODO wybe 08-05-2022: Check how to properly do the picking of the font.
        painter.text(
            rect.left_top(),
            Align2::LEFT_TOP,
            "size comparison",
            FontId::new(15., FontFamily::Proportional),
            visuals.text_color(),
        );

        // Show the subject star.
        draw_relative_orbit_size(&painter, visuals, rect.center(), radius, "This", radius);

        // TODO: Add actual orbits / star sizes to this.
        let comparison_radii: Vec<(SolarRadius, &'static str)> = vec![
            (1.0.into(), "Sun"),
            (4.0.into(), "Orbit 2"),
            (10.0.into(), "Orbit 6"),
        ];

        for (orbit_radius, name) in comparison_radii {
            draw_relative_orbit_size(&painter, visuals, rect.center(), orbit_radius, name, radius)
        }
    }
}

/// Draws a circle at the `center`.
/// The circle will have an on-screen radius relative to `comparison_radius`.
/// TODO: Somehow layout the text so that it doesn't overlap / hides when necessary?
fn draw_relative_orbit_size(
    painter: &Painter,
    style: &WidgetVisuals,
    center: Pos2,
    radius: SolarRadius,
    name: &str,
    comparison_radius: SolarRadius,
) {
    /// The comparison star is always this large.
    const PIXELS_PER_COMPARISON_STAR_RADIUS: f32 = 20.0;

    let radius: f32 = radius.into();
    let comparison_radius: f32 = comparison_radius.into();

    let relative_radius = radius / comparison_radius;
    let circle_radius = relative_radius * PIXELS_PER_COMPARISON_STAR_RADIUS;

    painter.circle(center, circle_radius, Color32::TRANSPARENT, style.fg_stroke);

    // TODO wybe 08-05-2022: Check how to properly do the picking of the font.
    painter.text(
        center + Vec2::new(circle_radius, 0.),
        Align2::RIGHT_CENTER,
        format!("{}\n{:.1}", name, radius),
        FontId::new(15., FontFamily::Proportional),
        style.text_color(),
    );
}
