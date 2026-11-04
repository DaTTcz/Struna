//! "Now playing" indicator derived from the Struna app icon:
//! the string between two pegs, vibrating as a standing wave.
//!
//! Drawn with the egui painter (no image asset), so it stays sharp at any
//! size and DPI. Below `SMALL_BELOW` px it switches to the simplified list
//! variant (no ghost, thicker string, bigger swing), like `struna-small.svg`.
//!
//! Usage in a track row, in place of the track number:
//! ```ignore
//! if is_current {
//!     widgets::now_playing(ui, 18.0, player.is_playing());
//! } else {
//!     ui.label(track_no.to_string());
//! }
//! ```

use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};
use std::f32::consts::TAU;
use std::time::Duration;

pub const COPPER: Color32 = Color32::from_rgb(0xE0, 0xA1, 0x5C);
pub const SAGE: Color32 = Color32::from_rgb(0x8F, 0xB8, 0xB0);

/// One full swing there and back, in seconds.
const PERIOD: f32 = 1.2;
/// Line segments used to approximate the sine.
const SEGMENTS: usize = 24;
/// Below this size the simplified list variant is drawn.
const SMALL_BELOW: f32 = 32.0;
/// ~30 fps is plenty for a small indicator and keeps CPU use low.
const FRAME: Duration = Duration::from_millis(33);

/// Allocates a `size`×`size` square and draws the indicator in it.
/// While `playing` is true the string vibrates; when paused it rests
/// at full swing, i.e. it looks exactly like the app icon.
pub fn now_playing(ui: &mut Ui, size: f32, playing: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        let phase = if playing {
            let t = ui.input(|i| i.time) as f32;
            (t / PERIOD * TAU).cos()
        } else {
            1.0
        };
        paint(ui.painter(), rect, phase);
        if playing {
            ui.ctx().request_repaint_after(FRAME);
        }
    }
    response
}

/// Draws one frame. `phase` runs from -1 to 1 (1 = the icon's pose).
pub fn paint(painter: &Painter, rect: Rect, phase: f32) {
    let small = rect.width() < SMALL_BELOW;

    // Geometry on the icon's 200×200 grid, mapped onto `rect`.
    // The small variant crops tighter around the string (viewBox 24..176).
    let (origin, span) = if small { (24.0, 152.0) } else { (16.0, 168.0) };
    let s = rect.width() / span;
    let p = |x: f32, y: f32| {
        Pos2::new(
            rect.min.x + (x - origin) * s,
            rect.center().y + (y - 100.0) * s,
        )
    };

    let (amp, string_w, peg_r) = if small { (36.0, 18.0, 15.0) } else { (24.0, 7.0, 9.0) };
    let px = |w: f32| (w * s).max(1.0);

    let wave = |k: f32| -> Vec<Pos2> {
        (0..=SEGMENTS)
            .map(|i| {
                let u = i as f32 / SEGMENTS as f32;
                p(40.0 + 120.0 * u, 100.0 - amp * k * (TAU * u).sin())
            })
            .collect()
    };

    if !small {
        // Ghost: the opposite extreme, faint.
        painter.add(Shape::line(
            wave(-phase),
            Stroke::new(px(4.0), COPPER.gamma_multiply(0.35)),
        ));
    }
    painter.add(Shape::line(wave(phase), Stroke::new(px(string_w), COPPER)));

    // Pegs drawn last so they also cover the line ends.
    painter.circle_filled(p(40.0, 100.0), px(peg_r), SAGE);
    painter.circle_filled(p(160.0, 100.0), px(peg_r), SAGE);
}
