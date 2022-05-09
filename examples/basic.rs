//! A basic example which simple displays a static message

use sphinx::UpdateContext;

struct Basic {
    message: &'static str,
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            message: "Hello, World!",
        }
    }
}

impl sphinx::App for Basic {
    fn update(&mut self, ctx: UpdateContext) {
        // todo
    }
}

fn main() {
    let app = Basic::default();
    sphinx::run(app, sphinx::WindowOptions::default());
}
