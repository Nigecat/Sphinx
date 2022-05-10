use super::Widget;

/// An animation (video or gif) player.
pub struct AnimationPlayer {}

impl Widget for AnimationPlayer {
    fn ui(self, _ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        unimplemented!();
    }
}
