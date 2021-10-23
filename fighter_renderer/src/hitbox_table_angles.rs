use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use web_sys::Document;

const ARROW_RADIUS: f64 = 20.0;

pub fn draw_hitbox_table_angles(document: &Document) {
    let elements = document.get_elements_by_class_name("hitbox-angle-render");
    for i in 0..elements.length() {
        let element = elements.item(i).unwrap();
        let angle_degrees = element.get_attribute("angle").unwrap();
        let hitbox_id = element.get_attribute("hitbox-id").unwrap();

        let color = match hitbox_id.as_str() {
            "0" => "#EF6400",
            "1" => "#FF0000",
            "2" => "#FF00FF",
            "3" => "#18d6c9",
            "4" => "#24d618",
            _ => "#FFFFFF",
        };

        let canvas: web_sys::HtmlCanvasElement = element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        if angle_degrees == "361" {
            canvas.set_width(ARROW_RADIUS as u32 * 4);
            canvas.set_height(ARROW_RADIUS as u32 * 2);

            draw_angle(&context, ARROW_RADIUS, ARROW_RADIUS, 0.0, color);
            draw_angle(&context, ARROW_RADIUS * 3.0, ARROW_RADIUS, 44.0, color);
        } else {
            canvas.set_width(ARROW_RADIUS as u32 * 2);
            canvas.set_height(ARROW_RADIUS as u32 * 2);

            draw_angle(
                &context,
                ARROW_RADIUS,
                ARROW_RADIUS,
                angle_degrees.parse().unwrap(),
                color,
            );
        }
    }
}

fn draw_angle(
    context: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    angle_degrees: f64,
    color: &str,
) {
    let angle_radians = angle_degrees / 180.0 * PI;
    context.set_stroke_style(&color.into());
    context.set_fill_style(&color.into());
    context.set_line_width(2.0);
    context.begin_path();
    context.arc(x as f64, y as f64, 2.0, 0.0, PI * 2.0).unwrap();
    context.fill();

    context.begin_path();
    context.move_to(x, y);
    let head_x = x + angle_radians.cos() * ARROW_RADIUS;
    let head_y = y - angle_radians.sin() * ARROW_RADIUS;
    context.line_to(head_x, head_y);
    context.move_to(
        head_x + (angle_radians + PI + 0.4).cos() * ARROW_RADIUS / 2.0,
        head_y - (angle_radians + PI + 0.4).sin() * ARROW_RADIUS / 2.0,
    );
    context.line_to(head_x, head_y);
    context.line_to(
        head_x + (angle_radians + PI - 0.4).cos() * ARROW_RADIUS / 2.0,
        head_y - (angle_radians + PI - 0.4).sin() * ARROW_RADIUS / 2.0,
    );
    context.stroke();
}
