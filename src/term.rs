use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::Hide,
    event::{self, Event},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use glam::{U16Vec2, UVec2};

use crate::{rect::TermRect, slice::TermSlice};

/// Exposes some terminal's apis for user interface purposes
///
/// You can change terminals options with [`Term::enable_raw_mode`], [`Term::enter_alternate_screen`] and [`Term::hide_cursor`]\
/// Theses options are cleared when the [`Term`] is dropped
pub struct Term {
    stdout: Stdout,
    size: UVec2,
    raw_mode_enabled: bool,
    alternate_screen_enabled: bool,
    cursor_hidden: bool,
}

impl Term {
    #[must_use]
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            size: U16Vec2::from(terminal::size().unwrap_or((0, 0))).into(),
            raw_mode_enabled: false,
            alternate_screen_enabled: false,
            cursor_hidden: false,
        }
    }

    #[must_use]
    pub fn slice(&mut self, rect: TermRect) -> TermSlice {
        TermSlice::new(self.rect(), rect)
    }

    #[must_use]
    pub fn full_slice(&mut self) -> TermSlice {
        TermSlice::new(self.rect(), self.rect())
    }

    pub fn enable_raw_mode(&mut self) -> &mut Self {
        if !self.raw_mode_enabled {
            self.raw_mode_enabled = enable_raw_mode().is_ok();
        }
        self
    }

    pub fn disable_raw_mode(&mut self) -> &mut Self {
        if self.raw_mode_enabled {
            self.raw_mode_enabled = disable_raw_mode().is_err();
        }
        self
    }

    pub fn enter_alternate_screen(&mut self) -> &mut Self {
        if !self.alternate_screen_enabled {
            self.alternate_screen_enabled = self.stdout.queue(EnterAlternateScreen).is_ok();
        }
        self
    }

    pub fn leave_alternate_screen(&mut self) -> &mut Self {
        if self.alternate_screen_enabled {
            self.alternate_screen_enabled = self.stdout.queue(EnterAlternateScreen).is_err();
        }
        self
    }

    pub fn hide_cursor(&mut self) -> &mut Self {
        if !self.cursor_hidden {
            self.cursor_hidden = self.stdout.queue(Hide).is_ok();
        }
        self
    }

    pub fn show_cursor(&mut self) -> &mut Self {
        if self.cursor_hidden {
            self.cursor_hidden = self.stdout.queue(Hide).is_err();
        }
        self
    }

    #[must_use]
    pub fn peek_event(&mut self) -> Option<Event> {
        event::poll(Duration::ZERO)
            .unwrap()
            .then_some(event::read().unwrap())
    }

    #[must_use]
    pub fn wait_for_event(&mut self) -> Event {
        event::read().unwrap()
    }

    #[must_use]
    pub fn size(&self) -> UVec2 {
        self.size
    }

    #[must_use]
    pub fn rect(&self) -> TermRect {
        TermRect::new((0, 0), self.size)
    }

    #[must_use]
    pub fn raw_mode_enabled(&self) -> bool {
        self.raw_mode_enabled
    }

    #[must_use]
    pub fn alternate_screen_enabled(&self) -> bool {
        self.alternate_screen_enabled
    }

    #[must_use]
    pub fn cursor_hidden(&self) -> bool {
        self.cursor_hidden
    }

    pub fn flush(&mut self) -> &mut Self {
        self.stdout.flush().ok();
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.stdout.queue(Clear(ClearType::All)).ok();
        self
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if self.raw_mode_enabled {
            disable_raw_mode().ok();
        }
        if self.alternate_screen_enabled {
            self.stdout.execute(LeaveAlternateScreen).ok();
        }
    }
}
