use std::borrow::BorrowMut;

use super::App;
use crate::utils::Error;
use egui::{Button, Color32, Context, RichText, ScrollArea, TextStyle, Ui, Window};

pub trait ErrorWindow {
    fn draw_error_window(&mut self, ctx: &Context);
    fn _row(ui: &mut Ui, e: &Error);
}

impl ErrorWindow for App {
    fn draw_error_window(&mut self, ctx: &Context) {
        Window::new("Error Log")
            .open(self.show_error_log.borrow_mut())
            .show(ctx, |ui| {
                let row_height = ui.text_style_height(&TextStyle::Body);
                let mut total_rows = self.error_log.len();

                if ui
                    .add_enabled(total_rows > 0, Button::new("Clear log"))
                    .clicked()
                {
                    self.error_log.clear();
                    total_rows = 0;
                }
                ui.separator();

                match total_rows {
                    0 => {
                        ui.heading("Nothing to see here!");
                        ui.label(
                            RichText::new("Imagine there's no errors...")
                                .italics()
                                .color(Color32::DARK_GRAY),
                        );
                    }
                    _ => {
                        ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .max_height(300.0)
                            .show_rows(ui, row_height * 2.0, total_rows, |ui, error_range| {
                                for i in error_range {
                                    Self::_row(ui, self.error_log.get(i).expect("Index of range"));
                                }
                            });
                    }
                }
            });
    }

    fn _row(ui: &mut Ui, e: &Error) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{} |", e.ts.utc)).color(Color32::GRAY));
            ui.label(RichText::new(e.error.to_string()).color(Color32::RED));
        });
        ui.separator();
    }
}
