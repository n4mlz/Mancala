#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Player {
    A,
    B,
}

impl Player {
    #[inline]
    pub fn opponent(self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    #[inline]
    pub(crate) fn idx(self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }
}
