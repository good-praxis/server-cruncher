use egui::{Context, Window};
use hcloud::models::Image;

pub fn image_window(unnamed_counter: &mut usize, image: &Image, ctx: &Context) {
    Window::new(
        image
            .name
            .clone()
            .unwrap_or(format!("unnamed image {}", get_unique_id(unnamed_counter)))
            .as_str(),
    )
    .show(ctx, |ui| {
        ui.label(format!("type: {:?}", image.r#type));
        ui.label(format!("Bound to: {:?}", image.bound_to));
        ui.label(format!("Status: {:?}", image.status));
        ui.collapsing("details", |ui| ui.label(format!("{:?}", image)));
    });
}

fn get_unique_id(counter: &mut usize) -> usize {
    let id = *counter;
    *counter += 1;
    id
}
