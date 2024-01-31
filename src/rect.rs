use glam::{IVec2, UVec2};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermRect {
    pub pos: IVec2,
    pub size: UVec2,
}

impl TermRect {
    #[must_use]
    pub fn new(pos: impl Into<IVec2>, size: impl Into<UVec2>) -> Self {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    /// Return `self` moved by the position of `rhs` and limited to the size of `rhs`
    #[must_use]
    pub fn move_clamp(&self, rhs: Self) -> Self {
        let pos = rhs.pos + self.pos;
        let size = rhs.size.min(rhs.size);
        Self { pos, size }
    }

    #[must_use]
    pub fn x(&self) -> i32 {
        self.pos.x
    }

    #[must_use]
    pub fn y(&self) -> i32 {
        self.pos.y
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.size.x
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.size.y
    }
}
