//! Current status: BETA
//!
#![doc = include_str!("../readme.md")]
use ratatui::layout::{Position, Rect};

/// Widgets can be rendered to a temporary buffer using
/// the buffers own coordinate system.
///
/// To adjust the Rects derived during rendering/layout to
/// the actual screen coordinates a widget can implement this
/// trait.
///
/// Container widgets that support this will call relocate()
/// after rendering the widgets.
pub trait RelocatableState {
    /// Relocate the areas in this widgets state.
    fn relocate(&mut self, shift: (i16, i16), clip: Rect);
}

/// Shift the area by offset and clip it.
pub fn relocate_areas(area: &mut [Rect], shift: (i16, i16), clip: Rect) {
    for a in area {
        *a = relocate_area(*a, shift, clip)
    }
}

/// Shift the area by offset and clip it.
pub fn relocate_area(area: Rect, shift: (i16, i16), clip: Rect) -> Rect {
    let x0 = area.left().saturating_add_signed(shift.0);
    let x1 = area.right().saturating_add_signed(shift.0);
    let y0 = area.top().saturating_add_signed(shift.1);
    let y1 = area.bottom().saturating_add_signed(shift.1);

    let tgt = Rect::new(x0, y0, x1 - x0, y1 - y0);
    let tgt = tgt.intersection(clip);
    if !tgt.is_empty() {
        tgt
    } else {
        // default on empty
        Rect::default()
    }
}

/// Shift the position by offset and clip it.
/// Clipped positions are replaced with (0,0) for this function.
pub fn relocate_positions(pos: &mut [Position], shift: (i16, i16), clip: Rect) {
    for p in pos {
        *p = relocate_position(*p, shift, clip).unwrap_or_default()
    }
}

/// Shift the position by offset and clip it.
/// Returns None if the position is clipped.
pub fn relocate_position(pos: Position, shift: (i16, i16), clip: Rect) -> Option<Position> {
    let x0 = pos.x.saturating_add_signed(shift.0);
    let y0 = pos.y.saturating_add_signed(shift.0);
    let pos0 = Position::new(x0, y0);

    if clip.contains(pos0) {
        Some(pos0)
    } else {
        None
    }
}