use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};

use crossterm::{
    cursor::{self, MoveTo},
    style::{Color, SetBackgroundColor, SetForegroundColor, SetUnderlineColor},
    QueueableCommand,
};
use glam::{uvec2, IVec2, U16Vec2, UVec2};
use unicode_width::UnicodeWidthStr;

use crate::rect::TermRect;

pub struct TermSlice {
    stdout: Stdout,
    rect: TermRect,
}

impl TermSlice {
    #[must_use]
    pub(crate) fn new(context_rect: TermRect, rect: TermRect) -> Self {
        Self {
            stdout: stdout(),
            rect: rect.move_clamp(context_rect),
        }
    }

    #[must_use]
    pub fn rect(&self) -> TermRect {
        self.rect
    }

    #[must_use]
    pub fn pos(&self) -> IVec2 {
        self.rect.pos
    }

    #[must_use]
    pub fn size(&self) -> UVec2 {
        self.rect.size
    }

    #[must_use]
    pub fn x(&self) -> i32 {
        self.rect.x()
    }

    #[must_use]
    pub fn y(&self) -> i32 {
        self.rect.y()
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.rect.width()
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.rect.height()
    }

    pub fn set_text_color(&mut self, color: Color) -> &mut Self {
        self.stdout.queue(SetForegroundColor(color)).ok();
        self
    }

    pub fn reset_text_color(&mut self) -> &mut Self {
        self.stdout.queue(SetForegroundColor(Color::Reset)).ok();
        self
    }

    pub fn set_background_color(&mut self, color: Color) -> &mut Self {
        self.stdout.queue(SetBackgroundColor(color)).ok();
        self
    }

    pub fn reset_background_color(&mut self) -> &mut Self {
        self.stdout.queue(SetBackgroundColor(Color::Reset)).ok();
        self
    }

    pub fn set_underline_color(&mut self, color: Color) -> &mut Self {
        self.stdout.queue(SetUnderlineColor(color)).ok();
        self
    }

    pub fn reset_underline_color(&mut self) -> &mut Self {
        self.stdout.queue(SetUnderlineColor(Color::Reset)).ok();
        self
    }

    pub fn move_cursor(&mut self, pos: impl Into<IVec2>) -> &mut Self {
        let pos = Into::<IVec2>::into(pos).try_into().unwrap_or(U16Vec2::MAX);
        self.stdout.queue(MoveTo(pos.x, pos.y)).ok();
        self
    }

    pub fn write(&mut self, display: impl Display) -> &mut Self {
        let cursor_pos = self.cursor_pos();

        if (cursor_pos.y as i64) < self.y() as i64 {
            return self;
        }

        let string = display.to_string();
        let mut string = string.as_str();

        let columns_to_remove_start = self.x() as i64 - cursor_pos.x as i64;
        let columns_to_remove_end =
            (cursor_pos.x as i64 + string.width() as i64) - (self.x() as i64 + self.width() as i64);

        let string_width = string.width();

        if let Ok(columns_to_remove_start) = usize::try_from(columns_to_remove_start) {
            while string_width - string.width() < columns_to_remove_start {
                let Some((first_index, _)) = string.char_indices().next() else {
                    return self;
                };
                string = &string[first_index..];
            }
        }

        if let Ok(columns_to_remove_end) = usize::try_from(columns_to_remove_end) {
            while string_width - string.width() < columns_to_remove_end {
                let Some((last_index, _)) = string.char_indices().last() else {
                    return self;
                };
                string = &string[..last_index];
            }
        }

        if !string.is_empty() {
            self.stdout.write_all(string.as_bytes()).ok();
        }

        self
    }

    pub fn write_to(&mut self, pos: impl Into<IVec2>, display: impl Display) -> &mut Self {
        self.move_cursor(pos).write(display)
    }

    pub fn cursor_pos(&mut self) -> UVec2 {
        let pos = cursor::position().unwrap_or((u16::MAX, u16::MAX));
        uvec2(pos.0 as u32, pos.1 as u32)
    }
}

impl Clone for TermSlice {
    fn clone(&self) -> Self {
        Self {
            stdout: stdout(),
            rect: self.rect,
        }
    }
}
