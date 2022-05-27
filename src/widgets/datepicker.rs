use crate::widgets::Frame;
use crate::{Area, Key, Order};
use chrono::{Date, TimeZone, Utc};
use eframe::egui::Id;
use std::fmt;

/// A date picker.
pub struct DatePicker<'d, Tz>
where
    Tz: TimeZone,
    Tz::Offset: fmt::Display,
{
    id: Id,
    date: &'d mut Date<Tz>,
}

impl<'a, Tz> DatePicker<'a, Tz>
where
    Tz: TimeZone,
    Tz::Offset: fmt::Display,
{
    /// Create a new date picker.
    pub fn new(date: &'a mut Date<Tz>) -> Self {
        DatePicker {
            id: Id::new(&date),
            date,
        }
    }
}

impl<'d, Tz> super::Widget for DatePicker<'d, Tz>
where
    Tz: TimeZone,
    Tz::Offset: fmt::Display,
{
    fn ui(self, ui: &mut crate::Ui) -> crate::Response {
        let current = self.date.to_string();

        let btn = ui.button(current);

        if btn.clicked() {
            ui.memory().toggle_popup(self.id);
        }

        if ui.memory().is_popup_open(self.id) {
            let ar = Area::new(self.id)
                .order(Order::Foreground)
                .default_pos(btn.rect.left_bottom())
                .movable(false) // fixme try without
                .show(ui.ctx(), |ui| {
                    Frame::popup(ui.style()).show(ui, |ui| {
                        // ----- Render header -----
                        ui.horizontal(|ui| {
                            // todo
                            // todo

                            if ui.button("Today").clicked() {
                                *self.date = Utc::now().with_timezone(&self.date.timezone()).date();
                            }
                        });

                        // todo
                    })
                })
                .response;

            // Exit popup if escape pressed or user clicks outside of it
            if !btn.clicked() && (ui.input().key_pressed(Key::Escape) || ar.clicked_elsewhere()) {
                ui.memory().toggle_popup(self.id);
            }
        }

        btn
    }
}
