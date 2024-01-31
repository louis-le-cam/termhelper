mod rect;
mod slice;
mod term;

pub use crossterm::{
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
        ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
    },
    style::Color,
};
pub use glam::{ivec2, uvec2, IVec2, UVec2};

pub use crate::{rect::TermRect, slice::TermSlice, term::Term};
