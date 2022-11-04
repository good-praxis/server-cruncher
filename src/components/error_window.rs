use crate::utils::Error;
use egui::{Color32, Context, RichText, ScrollArea, TextStyle, Ui, Window};

pub fn error_window(error_log: &Vec<Error>, open: &mut bool, ctx: &Context) {
    Window::new("error_log").open(open).show(ctx, |ui| {
        let row_height = ui.text_style_height(&TextStyle::Body);
        let total_rows = error_log.len();

        ScrollArea::vertical().stick_to_bottom(true).show_rows(
            ui,
            row_height * 2.0,
            total_rows,
            |ui, error_range| {
                for i in error_range {
                    row(ui, error_log.get(i).expect("Index of range"));
                }
            },
        );
    });
}

fn row(ui: &mut Ui, e: &Error) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{} |", e.ts)).color(Color32::GRAY));
        ui.label(RichText::new(e.error.to_string()).color(Color32::RED));
    });
    ui.separator();
}
