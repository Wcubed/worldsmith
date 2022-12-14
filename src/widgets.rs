use eframe::egui::{
    Align2, Button, Color32, CursorIcon, FontFamily, FontId, Painter, Pos2, Sense, Ui, Vec2,
    Widget, WidgetText,
};
use worldsmith_lib::units::{SolarRadius, Unit};

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

    let (response, painter) = ui.allocate_painter(widget_size, Sense::hover());
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

        let subject_star_pixel_radius = 20.0;

        // Show the subject star.
        let mut subject_star_background = visuals.fg_stroke.color;
        subject_star_background = subject_star_background.linear_multiply(0.2);

        painter.circle(
            rect.center(),
            subject_star_pixel_radius,
            subject_star_background,
            visuals.fg_stroke,
        );
        // TODO wybe 08-05-2022: Check how to properly do the picking of the font.
        painter.text(
            rect.center() + Vec2::new(0., subject_star_pixel_radius),
            Align2::CENTER_TOP,
            format!("{:.1}", radius),
            FontId::new(15., FontFamily::Proportional),
            visuals.text_color(),
        );

        // TODO: cache draw radii, and make this vector a constant?
        let comparison_radii: Vec<(SolarRadius, &'static str)> = vec![
            (0.18.into(), "EZ Aquarii A"),
            (1.0.into(), "Sun"),
            (3.8.into(), "Pi\nAndromedae A"),
            (9.8.into(), "Theta Orionis C"),
            (83.2.into(), "Avg. orbit\nMercury"),
        ];

        // Draw comparison stars / orbits
        for (comparison_radius, name) in comparison_radii {
            let draw_radius =
                calculate_draw_radius(comparison_radius, radius, subject_star_pixel_radius);

            painter.circle(
                rect.center(),
                draw_radius,
                Color32::TRANSPARENT,
                visuals.fg_stroke,
            );

            if draw_radius >= subject_star_pixel_radius {
                // TODO wybe 08-05-2022: Check how to properly do the picking of the font.
                // TODO: When to draw the text so that it doesn't overlap each other?
                painter.text(
                    rect.center() + Vec2::new(draw_radius, 0.),
                    Align2::LEFT_TOP,
                    format!("{:.1} {}\n{}", comparison_radius, SolarRadius::SYMBOL, name),
                    FontId::new(15., FontFamily::Proportional),
                    visuals.text_color(),
                );
            }
        }

        // Draw ruler and tick marks.
        let major_tick_mark_height = 10.0;
        let minor_tick_mark_height = 5.0;
        let major_tick_mark_frequency = 5;

        let ruler_range = rect.left()..=rect.right();

        painter.hline(ruler_range, rect.center().y, visuals.fg_stroke);

        let tick_mark_distance =
            calculate_draw_radius(1.0.into(), radius, subject_star_pixel_radius);
        let tick_mark_amount = (rect.center().x / tick_mark_distance).ceil() as usize;

        for i in 0..tick_mark_amount {
            let tick_mark_height = if i % major_tick_mark_frequency == 0 {
                major_tick_mark_height
            } else {
                minor_tick_mark_height
            };

            // Paint 2 tick marks mirrored along the stars center.
            painter.vline(
                rect.center().x - i as f32 * tick_mark_distance,
                (rect.center().y - tick_mark_height)..=rect.center().y,
                visuals.fg_stroke,
            );
            painter.vline(
                rect.center().x + i as f32 * tick_mark_distance,
                (rect.center().y - tick_mark_height)..=rect.center().y,
                visuals.fg_stroke,
            );
        }
    }
}

/// Calculates the radius in pixels of a given orbit, relative to the reference radius.
fn calculate_draw_radius(
    radius: SolarRadius,
    reference_radius: SolarRadius,
    reference_radius_pixels: f32,
) -> f32 {
    let radius: f32 = radius.into();
    let reference_radius: f32 = reference_radius.into();

    (radius / reference_radius) * reference_radius_pixels
}
